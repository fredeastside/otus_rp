use sh_core::error::HomeError;
use sh_core::home::Home;
use sh_core::report::{self};
use sh_core::room::Room;
use sh_core::socket::Socket;
use sh_core::state::Switchable;
use sh_core::temperature::Units;
use sh_core::thermometer::Thermometer;

fn main() {
    add_and_remove_rooms();
    add_and_remove_devices();
    report();
    error_handling();
}

fn add_and_remove_rooms() {
    let kitchen = Room::new("kitchen");
    let living_room = Room::new("living_room");
    let mut h = Home::new();
    h.add_room(kitchen);
    h.add_room(living_room);
    if let Some(k) = h.get_room("kitchen") {
        report::report(k);
    }
    h.remove_room("kitchen");
    if let Err(err) = h.get_device("kitchen", "socket") {
        println!("Error: {err}");
    }
}

fn add_and_remove_devices() {
    let mut kitchen = Room::new("kitchen");
    let thermo = Thermometer::new("thermometer in the kitchen", Units::Celsius(20.0));
    let socket = Socket::new("socket in the kitchen", 110.0);
    kitchen.add_device(thermo);
    kitchen.add_device(socket);
    let mut h = Home::new();
    h.add_room(kitchen);
    if let Ok(s) = h.get_device("kitchen", "socket in the kitchen") {
        report::report(s);
    }
    if let Some(r) = h.get_mut_room("kitchen") {
        r.remove_device("socket in the kitchen");
    }
    if let Err(err) = h.get_device("kitchen", "socket in the kitchen") {
        println!("Error: {err}");
    }
}

fn report() {
    let mut thermo = Thermometer::new("thermometer in the kitchen", Units::Celsius(20.0));
    thermo.on();
    let mut socket = Socket::new("socket in the kitchen", 110.0);
    socket.on();
    let mut room = Room::new("kitchen");
    room.add_device(thermo);
    room.add_device(socket);
    let mut h = Home::new();
    h.add_room(room);
    report::report(&h);
}

fn error_handling() {
    let thermo = Thermometer::new("thermometer in the kitchen", Units::Celsius(20.0));
    let socket = Socket::new("socket in the kitchen", 110.0);
    let mut room = Room::new("kitchen");
    room.add_device(thermo);
    room.add_device(socket);
    let mut h = Home::new();
    h.add_room(room);
    match h.get_device("living_room", "socket in the kitchen") {
        Ok(d) => report::report(d),
        Err(HomeError::RoomNotFound(r)) => {
            println!("Room {r} not found.");
        }
        Err(HomeError::DeviceNotFound { room, device }) => {
            println!("Device {device} in {room} not found.");
        }
    }
    match h.get_device("kitchen", "device1") {
        Ok(d) => report::report(d),
        Err(HomeError::RoomNotFound(r)) => {
            println!("Room {r} not found.");
        }
        Err(HomeError::DeviceNotFound { room, device }) => {
            println!("Device {device} in {room} not found.");
        }
    }
}
