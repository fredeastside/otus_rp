use std::fmt::Display;

use crate::{socket::Socket, thermometer::Thermometer};
/// A unified wrapper for different types of smart devices in the home automation system.
///
/// This enum allows heterogeneous devices (like Sockets and Thermometers) to be stored
/// in common collections and processed uniformly.
pub enum Device {
    /// A generic smart thermometer used to report temperature data.
    Thermometer(Thermometer),
    /// A smart power socket that can be toggled on or off.
    Socket(Socket),
}

impl Display for Device {
    /// Formats the device for display with a specific prefix.
    ///
    /// This implementation delegates the core formatting to the underlying inner type,
    /// but prepends the device type and an indentation tab (`\t`) for structured output.
    ///
    /// # Format
    ///
    /// * **Thermometer:** `"\tThermometer: <inner_display>"`
    /// * **Socket:** `"\tSocket: <inner_display>"`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::Thermometer(t) => write!(f, "\tThermometer: {}", t),
            Device::Socket(s) => write!(f, "\tSocket: {}", s),
        }
    }
}
