use std::fmt;

use crate::state::{State, Switchable};

/// A smart socket that can be switched on/off and provides a specific voltage.
pub struct Socket {
    state: State,
    voltage: f64,
}

impl Socket {
    /// Creates a new `Socket` with a given voltage. It is off by default.
    pub fn new(voltage: f64) -> Self {
        Self {
            voltage,
            state: State::Off,
        }
    }

    /// Returns the voltage of the socket.
    ///
    /// Returns 0.0 if the socket is off.
    pub fn voltage(&self) -> f64 {
        if self.is_on() {
            self.voltage
        } else {
            0.0
        }
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
            State::On => write!(f, "State: on, voltage: {}V", self.voltage()),
            State::Off => write!(f, "State: off"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_socket_is_off() {
        let socket = Socket::new(220.0);
        assert!(!socket.is_on());
    }

    #[test]
    fn test_socket_can_be_turned_on() {
        let mut socket = Socket::new(220.0);
        socket.on();
        assert!(socket.is_on());
    }

    #[test]
    fn test_socket_can_be_turned_off() {
        let mut socket = Socket::new(220.0);
        socket.on();
        socket.off();
        assert!(!socket.is_on());
    }

    #[test]
    fn test_voltage_is_zero_when_off() {
        let socket = Socket::new(220.0);
        assert_eq!(socket.voltage(), 0.0);
    }

    #[test]
    fn test_voltage_is_correct_when_on() {
        let mut socket = Socket::new(220.0);
        socket.on();
        assert_eq!(socket.voltage(), 220.0);
    }

    #[test]
    fn test_display_when_off() {
        let socket = Socket::new(220.0);
        assert_eq!(format!("{}", socket), "State: off");
    }

    #[test]
    fn test_display_when_on() {
        let mut socket = Socket::new(220.0);
        socket.on();
        assert_eq!(format!("{}", socket), "State: on, voltage: 220V");
    }
}
