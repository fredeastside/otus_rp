/// Builds a [`Room`](crate::room::Room) from a name and zero or more
/// `key => device` pairs.
///
/// # Examples
///
/// ```
/// use sh_core::make_room;
/// use sh_core::socket::Socket;
///
/// let empty = make_room!("kitchen");
/// let furnished = make_room!("kitchen", "s1" => Socket::new("s1", 220.0));
/// ```
#[macro_export]
macro_rules! make_room {
    // No pairs: no `mut` needed, so no `unused_mut` warning.
    ($name:expr $(,)?) => {
        $crate::room::Room::new($name)
    };
    // One or more `key => device` pairs.
    ($name:expr $(, $key:expr => $device:expr)+ $(,)?) => {{
        let mut room = $crate::room::Room::new($name);
        $( room.insert_device($key, $device); )+
        room
    }};
}

#[cfg(test)]
mod tests {
    use crate::socket::Socket;
    use crate::temperature::Units;
    use crate::thermometer::Thermometer;

    #[test]
    fn creates_empty_room() {
        let room = make_room!("kitchen");
        assert_eq!(room.name(), "kitchen");
        assert!(room.get_device("s1").is_none());
    }

    #[test]
    fn creates_room_with_devices() {
        let room = make_room!(
            "kitchen",
            "t1" => Thermometer::new("t1", Units::default()),
            "s1" => Socket::new("s1", 220.0),
        );
        assert_eq!(room.name(), "kitchen");
        assert!(room.get_device("t1").is_some());
        assert!(room.get_device("s1").is_some());
        assert!(room.get_device("missing").is_none());
    }

    #[test]
    fn trailing_comma_is_allowed() {
        let room = make_room!("kitchen", "s1" => Socket::new("s1", 220.0),);
        assert!(room.get_device("s1").is_some());
    }
}
