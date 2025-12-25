use std::fmt::Display;

use crate::device::Device;

/// Represents a room containing several smart devices.
pub struct Room {
    devices: [Device; 5],
}

impl Room {
    /// Creates a new room with a given set of devices.
    pub fn new(devices: [Device; 5]) -> Self {
        Self { devices }
    }

    /// Returns a reference to a device by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get_device(&self, idx: usize) -> &Device {
        if let Some(d) = self.devices.get(idx) {
            d
        } else {
            panic!("Wrong device index.");
        }
    }

    /// Returns a mutable reference to a device by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get_mut_device(&mut self, idx: usize) -> &mut Device {
        if let Some(d) = self.devices.get_mut(idx) {
            d
        } else {
            panic!("Wrong device index.");
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.devices
            .iter()
            .try_for_each(|d| writeln!(f, "-- Device: \n{}", d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::socket::Socket;
    use crate::state::Switchable;
    use crate::temperature::Units;
    use crate::thermometer::Thermometer;

    fn create_test_devices() -> [Device; 5] {
        [
            Device::Thermometer(Thermometer::new(Units::Celsius(20.0))),
            Device::Socket(Socket::new(220.0)),
            Device::Thermometer(Thermometer::new(Units::Fahrenheit(70.0))),
            Device::Socket(Socket::new(110.0)),
            Device::Socket(Socket::new(220.0)),
        ]
    }

    #[test]
    fn test_new_room() {
        let devices = create_test_devices();
        let room = Room::new(devices);
        assert_eq!(room.devices.len(), 5);
    }

    #[test]
    fn test_get_device() {
        let devices = create_test_devices();
        let room = Room::new(devices);
        let device = room.get_device(0);
        assert!(matches!(device, Device::Thermometer(_)));
    }

    #[test]
    fn test_get_mut_device() {
        let devices = create_test_devices();
        let mut room = Room::new(devices);
        let device = room.get_mut_device(1);
        if let Device::Socket(s) = device {
            s.on();
        }
        let device = room.get_device(1);
        assert_eq!(device.to_string(), "\tSocket: State: on, voltage: 220V");
    }

    #[test]
    #[should_panic(expected = "Wrong device index.")]
    fn test_get_device_panics_on_wrong_index() {
        let devices = create_test_devices();
        let room = Room::new(devices);
        room.get_device(5);
    }
}
