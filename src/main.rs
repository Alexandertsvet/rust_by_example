use smart_home::smart_devices::functions::get_random_number;
use smart_home::smart_devices::home::SmartHouse;
use smart_home::smart_devices::room::Room;
use smart_home::smart_devices::smart_devices_mod::SmartDevice;
use smart_home::smart_devices::socket::SmartSocet;
use smart_home::smart_devices::thermometer::SmartTermometr;

fn main() {
    // --- Создание устройств ---
    let t1 = SmartDevice::Thermometer(SmartTermometr::new(
        "Комнатный".to_string(),
        get_random_number(15, 20),
    ));
    let t2 = SmartDevice::Thermometer(SmartTermometr::new("Оконный".to_string(), 26));
    let s1 = SmartDevice::Socket(SmartSocet::new("Телевизор".to_string(), true, 220));

    println!("Создано устройств: 3");
    println!("  {:?}", t1);
    println!("  {:?}", t2);
    println!("  {:?}", s1);

    // --- Комната ---
    let mut living_room = Room::new("Гостиная".to_string(), vec![t1, t2, s1]);
    living_room.print_report();

    // --- Доступ по индексу ---
    println!("Устройство 0: {:?}", living_room.get_device(0));
    println!("Устройство 1: {:?}", living_room.get_device(1));
    println!("Устройство 2: {:?}", living_room.get_device(2));

    // --- Мутация через get_device_mut ---
    // Включаем/выключаем розетку и меняем температуру термометра.
    if let SmartDevice::Thermometer(t) = living_room.get_device_mut(0) {
        t.set_temperature(21);
    }
    if let SmartDevice::Socket(s) = living_room.get_device_mut(2) {
        s.turn_off();
        s.set_power(0);
    }
    println!("После изменения устройств:");
    living_room.print_report();

    // --- Дом ---
    let my_house = SmartHouse::new("Дача".to_string(), vec![living_room]);
    my_house.print_report();
}
