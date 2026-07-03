use crate::device::Device;
use crate::report::Reporter;
use std::collections::HashMap;
use std::fmt::Display;

/// Represents a room containing several smart devices.
#[derive(Debug)]
pub struct Room {
    name: String,
    devices: HashMap<String, Device>,
}

impl Room {
    /// Creates a new room with a given set of devices.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: HashMap::new(),
        }
    }

    /// Adds a device to the room, keyed by its name.
    ///
    /// Accepts any type convertible into a [`Device`] (e.g. [`Socket`](crate::socket::Socket)
    /// or [`Thermometer`](crate::thermometer::Thermometer)). If a device with the same name
    /// already exists, it is replaced.
    pub fn add_device(&mut self, device: impl Into<Device>) {
        let device = device.into();
        self.devices.insert(device.name().to_string(), device);
    }

    /// Returns a reference to a device by its index.
    #[must_use]
    pub fn get_device(&self, name: &str) -> Option<&Device> {
        self.devices.get(name)
    }

    /// Returns a mutable reference to a device by its index.
    pub fn get_mut_device(&mut self, name: &str) -> Option<&mut Device> {
        self.devices.get_mut(name)
    }

    /// Returns the room's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Removes the device with the given name, returning it if it was present.
    pub fn remove_device(&mut self, name: &str) -> Option<Device> {
        self.devices.remove(name)
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Room: {}", self.name)?;
        self.devices
            .values()
            .try_for_each(|d| writeln!(f, "-- {}: \n{d}", d.kind()))
    }
}

impl Reporter for Room {
    fn report(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::socket::Socket;
    use crate::state::Switchable;
    use crate::temperature::Units;
    use crate::thermometer::Thermometer;

    #[test]
    fn test_new_room() {
        let mut room = Room::new("room1");
        room.add_device(Thermometer::new("thermometer1", Units::default()));
        room.add_device(Socket::new("socket1", 220.0));
        room.add_device(Thermometer::new("thermometer2", Units::default()));
        room.add_device(Socket::new("socket2", 110.0));
        room.add_device(Socket::new("socket3", 220.0));
        assert_eq!(room.devices.len(), 5);
    }

    #[test]
    fn test_get_device() {
        let mut room = Room::new("room1");
        let t = Thermometer::new("thermometer1", Units::Celsius(20.0));
        let t_name = t.name().to_string();
        room.add_device(t);
        let device = room.get_device(&t_name).unwrap();
        assert_eq!(device.name(), t_name);
    }

    #[test]
    fn test_get_mut_device() {
        let mut room = Room::new("room1");
        let s = Socket::new("socket1", 220.0);
        let s_name = s.name().to_string();
        room.add_device(s);
        let device = room.get_mut_device(&s_name).expect("should exist");
        device.on();
        let device = room.get_device(&s_name).expect("should exist");
        assert_eq!(device.to_string(), "Socket: On, Voltage: 220V");
    }

    #[test]
    fn test_get_device_none_on_wrong_name() {
        let room = Room::new("room1");
        assert!(room.get_device("not_exists").is_none());
    }
}
