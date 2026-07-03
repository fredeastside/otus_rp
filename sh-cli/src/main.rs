use sh_core::home::Home;
use sh_core::room::Room;
use sh_core::socket::Socket;
use sh_core::state::Switchable;
use sh_core::temperature::Units;
use sh_core::thermometer::Thermometer;

fn main() {
    let mut thermo = Thermometer::new("thermometer in the kitchen", Units::Celsius(20.0));
    let thermo_name = thermo.name().to_string();
    thermo.on();
    // println!("{}", thermo);
    let mut socket = Socket::new("socket in the kitchen", 110.0);
    socket.on();
    // println!("{}", socket);
    let mut room = Room::new("kitchen");
    room.add_device(thermo);
    if let Some(device) = room.get_device(&thermo_name) {
        println!("{device}");
    }
    let mut building = Home::new();
    building.add_room(room);
    // println!("{}", building.get_room(0));
    println!("{building}");
    if let Some(room) = building.get_mut_room("room1")
        && let Some(t) = room.get_mut_device(&thermo_name)
    {
        println!("{} -- {}", t.kind(), t.name());
    }
    println!("{building}");
}
