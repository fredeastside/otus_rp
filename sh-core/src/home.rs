use std::collections::HashMap;
use std::fmt::Display;

use crate::device::Device;
use crate::error::HomeError;
use crate::report::Reporter;
use crate::room::Room;

/// Represents a building composed of multiple rooms.
#[derive(Debug)]
pub struct Home {
    rooms: HashMap<String, Room>,
}

impl Home {
    /// Creates a new building with a given set of rooms.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    /// Adds a room to the home, keyed by its name.
    ///
    /// If a room with the same name already exists, it is replaced.
    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name().to_string(), room);
    }

    /// Returns a reference to a room by its index.
    #[must_use]
    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    /// Returns a mutable reference to a room by its index.
    pub fn get_mut_room(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    /// Returns a reference to a device, looked up by its room name and device name.
    ///
    /// # Errors
    ///
    /// Returns [`HomeError::RoomNotFound`] if no room matches `room_name`, or
    /// [`HomeError::DeviceNotFound`] if the room exists but has no device named `device_name`.
    pub fn get_device(&self, room_name: &str, device_name: &str) -> Result<&Device, HomeError> {
        let room = self
            .get_room(room_name)
            .ok_or_else(|| HomeError::RoomNotFound(room_name.to_string()))?;
        room.get_device(device_name)
            .ok_or_else(|| HomeError::DeviceNotFound {
                room: room_name.to_string(),
                device: device_name.to_string(),
            })
    }

    /// Removes the room with the given name, returning it if it was present.
    pub fn remove_room(&mut self, name: &str) -> Option<Room> {
        self.rooms.remove(name)
    }
}

impl Default for Home {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Home {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rooms
            .values()
            .try_for_each(|r| writeln!(f, "Room: \n{r}"))
    }
}

impl Reporter for Home {
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

    fn create_test_room() -> Room {
        let mut room = Room::new("room1");
        room.add_device(Thermometer::new("thermometer1", Units::default()));
        room.add_device(Socket::new("socket1", 220.0));
        room.add_device(Thermometer::new("thermometer2", Units::default()));
        room.add_device(Socket::new("socket2", 110.0));
        room.add_device(Socket::new("socket3", 220.0));
        room
    }

    #[test]
    fn test_new_building() {
        let room = create_test_room();
        let mut building = Home::new();
        building.add_room(room);
        assert_eq!(building.rooms.len(), 1);
    }

    #[test]
    fn test_get_room() {
        let room = create_test_room();
        let room_name = room.name().to_string();
        let mut building = Home::new();
        building.add_room(room);
        let fetched_room = building.get_room(room_name.as_str());
        assert!(fetched_room.is_some());
    }

    #[test]
    fn test_get_mut_room() {
        let mut room = Room::new("room1");
        let room_name = room.name().to_string();
        let s = Socket::new("socket1", 220.0);
        let s_name = s.name().to_string();
        room.add_device(s);
        let mut building = Home::new();
        building.add_room(room);
        building
            .get_mut_room(room_name.as_str())
            .expect("Room should exist")
            .get_mut_device(&s_name)
            .expect("Device should exist")
            .on();
        assert_eq!(
            building
                .get_room(room_name.as_str())
                .expect("Room should exist")
                .get_device(&s_name)
                .expect("Device should exist")
                .to_string(),
            "Socket: On, Voltage: 220V"
        );
    }

    #[test]
    fn test_get_room_none_on_wrong_name() {
        let building = Home::new();
        assert!(building.get_room("non_existent_room").is_none());
    }
}
