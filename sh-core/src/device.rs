use crate::{report::Reporter, socket::Socket, state::Switchable, thermometer::Thermometer};
use std::fmt::{Debug, Display};

/// A smart device that can live in a [`Room`](crate::room::Room).
///
/// This enum unifies the concrete device types behind a single value so a room
/// can store them together in one collection.
#[derive(Debug)]
pub enum Device {
    /// A [`Socket`] device.
    Socket(Socket),
    /// A [`Thermometer`] device.
    Thermometer(Thermometer),
}

impl Device {
    /// Returns the human-readable kind of the device (e.g. `"Socket"`).
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Device::Socket(socket) => socket.kind(),
            Device::Thermometer(thermometer) => thermometer.kind(),
        }
    }

    /// Returns the device's name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Device::Socket(socket) => socket.name(),
            Device::Thermometer(thermometer) => thermometer.name(),
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::Socket(socket) => write!(f, "{socket}"),
            Device::Thermometer(thermometer) => write!(f, "{thermometer}"),
        }
    }
}

impl Reporter for Device {
    fn report(&self) -> String {
        self.to_string()
    }
}

impl From<Socket> for Device {
    fn from(socket: Socket) -> Self {
        Device::Socket(socket)
    }
}

impl From<Thermometer> for Device {
    fn from(thermometer: Thermometer) -> Self {
        Device::Thermometer(thermometer)
    }
}

impl Switchable for Device {
    fn on(&mut self) {
        match self {
            Device::Socket(socket) => socket.on(),
            Device::Thermometer(thermometer) => thermometer.on(),
        }
    }

    fn off(&mut self) {
        match self {
            Device::Socket(socket) => socket.off(),
            Device::Thermometer(thermometer) => thermometer.off(),
        }
    }

    fn state(&self) -> crate::state::State {
        match self {
            Device::Socket(socket) => socket.state(),
            Device::Thermometer(thermometer) => thermometer.state(),
        }
    }
}
