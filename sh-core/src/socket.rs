use crate::state::{State, Switchable};
use std::fmt;

/// A smart socket that can be switched on/off and provides a specific voltage.
#[derive(Debug)]
pub struct Socket {
    name: String,
    state: State,
    voltage: f64,
}

impl Socket {
    /// Creates a new `Socket` with a given voltage. It is off by default.
    #[must_use]
    pub fn new(name: &str, voltage: f64) -> Self {
        Self {
            name: name.to_string(),
            voltage,
            state: State::Off,
        }
    }

    /// Returns the kind of the device, always `"Socket"`.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        "Socket"
    }

    /// Returns the socket's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the configured voltage of the socket, regardless of its state.
    #[must_use]
    pub fn voltage(&self) -> f64 {
        self.voltage
    }

    /// Returns the actual voltage output: the configured voltage when on, or `0.0` when off.
    #[must_use]
    pub fn output(&self) -> f64 {
        if self.is_on() { self.voltage() } else { 0.0 }
    }
}

impl Switchable for Socket {
    fn on(&mut self) {
        self.state = State::On;
    }

    fn off(&mut self) {
        self.state = State::Off;
    }

    fn state(&self) -> State {
        self.state
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            State::On => write!(
                f,
                "{} ({}): {}, Voltage: {}V",
                self.kind(),
                self.name(),
                self.state,
                self.voltage()
            ),
            State::Off => write!(f, "{} ({}): {}", self.kind(), self.name(), self.state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_socket_is_off() {
        let socket = Socket::new("socket1", 220.0);
        assert!(!socket.is_on());
    }

    #[test]
    fn test_socket_can_be_turned_on() {
        let mut socket = Socket::new("socket1", 220.0);
        socket.on();
        assert!(socket.is_on());
    }

    #[test]
    fn test_socket_can_be_turned_off() {
        let mut socket = Socket::new("socket1", 220.0);
        socket.on();
        socket.off();
        assert!(!socket.is_on());
    }

    #[test]
    fn test_output_is_zero_when_off() {
        let socket = Socket::new("socket1", 220.0);
        assert!((socket.voltage() - 220.0).abs() < f64::EPSILON);
        assert!((socket.output() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_voltage_is_correct_when_on() {
        let mut socket = Socket::new("socket1", 220.0);
        socket.on();
        assert!((socket.voltage() - 220.0).abs() < f64::EPSILON);
        assert!((socket.output() - 220.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display_when_off() {
        let socket = Socket::new("socket1", 220.0);
        assert_eq!(format!("{socket}"), "Socket (socket1): Off");
    }

    #[test]
    fn test_display_when_on() {
        let mut socket = Socket::new("socket1", 220.0);
        socket.on();
        assert_eq!(format!("{socket}"), "Socket (socket1): On, Voltage: 220V");
    }
}
