use smart_home::smart_devices::functions::get_random_number;
use smart_home::smart_devices::home::SmartHouse;
use smart_home::smart_devices::room::Room;
use smart_home::smart_devices::smart_devices_mod::SmartDevice;
use smart_home::smart_devices::socket::SmartSocet;
use smart_home::smart_devices::thermometer::SmartTermometr;

fn main() {
    let t1: SmartDevice = SmartDevice::Thermometer(SmartTermometr::new(
        "Комнатный".to_string(),
        get_random_number(15, 20),
    ));
    println!("{:?}", t1);

    println!("Термометр создан успешно!");
    let t2: SmartDevice = SmartDevice::Thermometer(SmartTermometr::new("Оконный".to_string(), 26));
    let s1: SmartDevice = SmartDevice::Socket(SmartSocet::new("Телевизор".to_string(), true, 220));
    let room_devices = vec![t1, t2, s1];
    let mut living_room = Room::new("Гостиная".to_string(), room_devices);
    living_room.print_report();
    println!("{:?}", living_room.get_device(0));
    println!("{:?}", living_room.get_device(1));
    println!("{:?}", living_room.get_device(2));
    println!("{:?}", living_room.get_device_mut(0));
    println!("{:?}", living_room.get_device_mut(1));
    println!("{:?}", living_room.get_device_mut(2));

    let my_house = SmartHouse::new("Дача".to_string(), vec![living_room]);

    my_house.print_report();
}
