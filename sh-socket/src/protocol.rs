//! Wire protocol for the smart socket.
//!
//! Defines the [`Request`]/[`Response`] messages, their byte encoding
//! ([`Encode`]/[`Decode`]), and length-prefixed framing ([`write_frame`] /
//! [`read_frame`]) over any [`Read`]/[`Write`] stream.

use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

/// A command sent from a client to the smart socket server.
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Switch the socket on.
    On,
    /// Switch the socket off.
    Off,
    /// Ask the socket for its current power output.
    GetPower,
}

impl Encode for Request {
    fn encode(&self) -> Vec<u8> {
        match self {
            Request::On => vec![0],
            Request::Off => vec![1],
            Request::GetPower => vec![2],
        }
    }
}

impl Decode for Request {
    fn decode(data: &[u8]) -> Result<Self, ProtocolError> {
        let (&tag, rest) = data.split_first().ok_or(ProtocolError::TruncatedPayload)?;
        if !rest.is_empty() {
            return Err(ProtocolError::LengthMismatch);
        }
        match tag {
            0 => Ok(Request::On),
            1 => Ok(Request::Off),
            2 => Ok(Request::GetPower),
            _ => Err(ProtocolError::UnknownTag),
        }
    }
}

/// A reply sent from the server back to the client.
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The command was applied successfully.
    Ack,
    /// The socket's current power output.
    Power(f64),
    /// The request failed; carries a human-readable reason.
    Failure(String),
}

impl Encode for Response {
    fn encode(&self) -> Vec<u8> {
        match self {
            Response::Ack => vec![0],
            Response::Power(v) => {
                let mut buf = vec![1];
                buf.extend_from_slice(&v.to_be_bytes());
                buf
            }
            Response::Failure(s) => {
                let mut buf = vec![2];
                buf.extend_from_slice(s.as_bytes());
                buf
            }
        }
    }
}

impl Decode for Response {
    fn decode(data: &[u8]) -> Result<Self, ProtocolError> {
        let (&tag, rest) = data.split_first().ok_or(ProtocolError::TruncatedPayload)?;
        match tag {
            0 => Ok(Response::Ack),
            1 => {
                let value: [u8; 8] = rest.try_into().map_err(|_| ProtocolError::LengthMismatch)?;
                let power = f64::from_be_bytes(value);
                Ok(Response::Power(power))
            }
            2 => {
                let err = std::str::from_utf8(rest)?.to_owned();
                Ok(Response::Failure(err))
            }
            _ => Err(ProtocolError::UnknownTag),
        }
    }
}

/// An error produced while encoding, decoding, or framing a protocol message.
#[derive(Debug)]
pub enum ProtocolError {
    /// The message tag byte does not correspond to any known variant.
    UnknownTag,
    /// The payload was empty when at least a tag byte was expected.
    TruncatedPayload,
    /// The payload length does not match what the decoded variant requires.
    LengthMismatch,
    /// A `Failure` payload was not valid UTF-8.
    InvalidUtf8(std::str::Utf8Error),
    /// An underlying I/O error occurred while reading or writing a frame.
    Io(std::io::Error),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::UnknownTag => write!(f, "unknown tag"),
            ProtocolError::LengthMismatch => write!(f, "length mismatch"),
            ProtocolError::TruncatedPayload => write!(f, "truncated payload"),
            ProtocolError::InvalidUtf8(_) => write!(f, "invalid utf8"),
            ProtocolError::Io(_) => write!(f, "io error"),
        }
    }
}

impl Error for ProtocolError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProtocolError::Io(err) => Some(err),
            ProtocolError::InvalidUtf8(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(value: std::io::Error) -> Self {
        ProtocolError::Io(value)
    }
}

impl From<std::str::Utf8Error> for ProtocolError {
    fn from(value: std::str::Utf8Error) -> Self {
        ProtocolError::InvalidUtf8(value)
    }
}

/// Serializes a value into its on-the-wire byte payload (without a length prefix).
pub trait Encode {
    /// Returns the encoded payload for this value.
    fn encode(&self) -> Vec<u8>;
}

/// Reconstructs a value from its on-the-wire byte payload.
pub trait Decode: Sized {
    /// Decodes a value from a single, complete message payload.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError`] if the payload is empty
    /// ([`TruncatedPayload`](ProtocolError::TruncatedPayload)), carries an
    /// unknown tag ([`UnknownTag`](ProtocolError::UnknownTag)), has the wrong
    /// length ([`LengthMismatch`](ProtocolError::LengthMismatch)), or contains
    /// invalid UTF-8 ([`InvalidUtf8`](ProtocolError::InvalidUtf8)).
    fn decode(data: &[u8]) -> Result<Self, ProtocolError>;
}

/// Writes `payload` to `w` as a length-prefixed frame: a big-endian `u32`
/// length followed by the payload bytes.
///
/// # Errors
///
/// Returns [`ProtocolError::LengthMismatch`] if the payload is longer than
/// [`u32::MAX`], or [`ProtocolError::Io`] if writing to the stream fails.
pub fn write_frame(w: &mut impl Write, payload: &[u8]) -> Result<(), ProtocolError> {
    let length = u32::try_from(payload.len()).map_err(|_| ProtocolError::LengthMismatch)?;
    w.write_all(&length.to_be_bytes())?;
    w.write_all(payload)?;
    Ok(())
}

/// Reads a single length-prefixed frame from `r` and returns its payload.
///
/// Reads a big-endian `u32` length, then exactly that many payload bytes.
///
/// # Errors
///
/// Returns [`ProtocolError::Io`] if the stream ends before a full frame is
/// read or if a read otherwise fails.
pub fn read_frame(r: &mut impl Read) -> Result<Vec<u8>, ProtocolError> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    let length = u32::from_be_bytes(buf);
    let mut payload = vec![0u8; length as usize];
    r.read_exact(&mut payload)?;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_request() {
        let all = [Request::On, Request::Off, Request::GetPower];
        for req in all {
            let encoded = req.encode();
            assert_eq!(Request::decode(&encoded).unwrap(), req);
        }
    }

    #[test]
    fn round_trips_response() {
        let all = [
            Response::Ack,
            Response::Power(42.5),
            Response::Failure("some error".to_string()),
        ];
        for res in all {
            let encoded = res.encode();
            assert_eq!(Response::decode(&encoded).unwrap(), res);
        }
        let res = Response::Power(f64::NAN);
        let encoded = res.encode();
        let decoded = Response::decode(&encoded).unwrap();
        if let Response::Power(v) = decoded {
            assert!(f64::is_nan(v));
        } else {
            panic!("incorrect response type");
        }
    }

    #[test]
    fn rejects_malformed_request() {
        struct Case {
            name: &'static str,
            input: &'static [u8],
            expected: fn(&ProtocolError) -> bool,
        }
        let cases = [
            Case {
                name: "truncated payload",
                input: &[],
                expected: |e| matches!(e, ProtocolError::TruncatedPayload),
            },
            Case {
                name: "unknown tag",
                input: &[5],
                expected: |e| matches!(e, ProtocolError::UnknownTag),
            },
            Case {
                name: "length mismatch",
                input: &[5, 0],
                expected: |e| matches!(e, ProtocolError::LengthMismatch),
            },
        ];
        for tc in cases {
            let e = Request::decode(tc.input).unwrap_err();
            assert!((tc.expected)(&e), "{}", tc.name);
        }
    }

    #[test]
    fn rejects_malformed_response() {
        struct Case {
            name: &'static str,
            input: &'static [u8],
            expected: fn(&ProtocolError) -> bool,
        }
        let cases = [
            Case {
                name: "truncated payload",
                input: &[],
                expected: |e| matches!(e, ProtocolError::TruncatedPayload),
            },
            Case {
                name: "unknown tag",
                input: &[5],
                expected: |e| matches!(e, ProtocolError::UnknownTag),
            },
            Case {
                name: "length mismatch",
                input: &[1, 0],
                expected: |e| matches!(e, ProtocolError::LengthMismatch),
            },
            Case {
                name: "invalid utf",
                input: &[2, 255],
                expected: |e| matches!(e, ProtocolError::InvalidUtf8(_)),
            },
        ];
        for tc in cases {
            let e = Response::decode(tc.input).unwrap_err();
            assert!((tc.expected)(&e), "{}", tc.name);
        }
    }
}
