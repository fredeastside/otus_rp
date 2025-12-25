use smart_home::building::Building;
use smart_home::device::Device;
use smart_home::room::Room;
use smart_home::socket::Socket;
use smart_home::state::Switchable;
use smart_home::temperature::Units;
use smart_home::thermometer::Thermometer;

fn main() {
    let mut thermo = Thermometer::new(Units::Celsius(20.0));
    thermo.on();
    // println!("{}", thermo);
    let mut socket = Socket::new(110.0);
    socket.on();
    // println!("{}", socket);
    let devices = [
        Device::Thermometer(thermo),
        Device::Socket(socket),
        Device::Thermometer(Thermometer::new(Units::Celsius(0.0))),
        Device::Socket(Socket::new(110.0)),
        Device::Socket(Socket::new(220.0)),
    ];
    let room = Room::new(devices);
    // println!("{}", room.get_device(0));
    let mut building = Building::new([room]);
    // println!("{}", building.get_room(0));
    println!("{}", building);
    let thermo = building.get_mut_room(0).get_mut_device(0);
    if let Device::Thermometer(t) = thermo {
        t.fahrenheit();
        // println!("{}", thermo);
    }
    println!("{}", building);
}
