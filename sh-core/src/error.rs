use std::error::Error;
use std::fmt::Display;

/// Errors that can occur when looking up rooms or devices in a [`Home`](crate::home::Home).
#[derive(Debug)]
pub enum HomeError {
    /// No room with the given name exists. Holds the requested room name.
    RoomNotFound(String),
    /// The room exists but contains no device with the given name.
    DeviceNotFound {
        /// Name of the room that was searched.
        room: String,
        /// Name of the device that was not found.
        device: String,
    },
}

impl Display for HomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeError::RoomNotFound(room) => write!(f, "room {room} not found"),
            HomeError::DeviceNotFound { room, device } => {
                write!(f, "device {device} not found in room {room}")
            }
        }
    }
}

impl Error for HomeError {}
