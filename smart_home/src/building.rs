use std::fmt::Display;

use crate::room::Room;

/// Represents a building composed of multiple rooms.
pub struct Building {
    rooms: [Room; 1],
}

impl Building {
    /// Creates a new building with a given set of rooms.
    pub fn new(rooms: [Room; 1]) -> Self {
        Self { rooms }
    }

    /// Returns a reference to a room by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get_room(&self, idx: usize) -> &Room {
        if let Some(r) = self.rooms.get(idx) {
            r
        } else {
            panic!("Wrong room index.");
        }
    }

    /// Returns a mutable reference to a room by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get_mut_room(&mut self, idx: usize) -> &mut Room {
        if let Some(r) = self.rooms.get_mut(idx) {
            r
        } else {
            panic!("Wrong room index.");
        }
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rooms
            .iter()
            .try_for_each(|d| writeln!(f, "Room: \n{}", d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Device;
    use crate::socket::Socket;
    use crate::state::Switchable;
    use crate::thermometer::Thermometer;

    fn create_test_room() -> Room {
        let devices = [
            Device::Thermometer(Thermometer::new(Default::default())),
            Device::Socket(Socket::new(220.0)),
            Device::Thermometer(Thermometer::new(Default::default())),
            Device::Socket(Socket::new(110.0)),
            Device::Socket(Socket::new(220.0)),
        ];
        Room::new(devices)
    }

    #[test]
    fn test_new_building() {
        let room = create_test_room();
        let building = Building::new([room]);
        assert_eq!(building.rooms.len(), 1);
    }

    #[test]
    fn test_get_room() {
        let room = create_test_room();
        let building = Building::new([room]);
        let fetched_room = building.get_room(0);
        assert_eq!(fetched_room.get_device(1).to_string(), "\tSocket: State: off");
    }

    #[test]
    fn test_get_mut_room() {
        let room = create_test_room();
        let mut building = Building::new([room]);
        let fetched_room = building.get_mut_room(0);
        if let Device::Socket(s) = fetched_room.get_mut_device(1) {
            s.on();
        }
        assert_eq!(
            building.get_room(0).get_device(1).to_string(),
            "	Socket: State: on, voltage: 220V"
        );
    }

    #[test]
    #[should_panic(expected = "Wrong room index.")]
    fn test_get_room_panics_on_wrong_index() {
        let room = create_test_room();
        let building = Building::new([room]);
        building.get_room(1);
    }
}
