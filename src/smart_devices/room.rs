use super::functions::ReportDevice;
use super::smart_devices_mod::SmartDevice;

pub struct Room {
    pub name: String,
    pub devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(name: String, devices: Vec<SmartDevice>) -> Self {
        Self { name, devices }
    }

    pub fn get_device(&self, index: usize) -> &SmartDevice {
        if index >= self.devices.len() {
            panic!(
                "Индекс устройства {} выходит за пределы комнаты {}!",
                index, self.name
            );
        }
        &self.devices[index]
    }

    pub fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice {
        if index >= self.devices.len() {
            panic!(
                "Индекс устройства {} выходит за пределы комнаты {}!",
                index, self.name
            );
        }
        &mut self.devices[index]
    }

    pub fn print_report(&self) {
        println!("Комната: {}", self.name);
        for device in &self.devices {
            device.print_status();
            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::smart_devices::smart_devices_mod::SmartDevice;
    use crate::smart_devices::socket::SmartSocet;
    use crate::smart_devices::thermometer::SmartTermometr;

    fn thermometer(name: &str, temp: i64) -> SmartDevice {
        SmartDevice::Thermometer(SmartTermometr::new(name.to_string(), temp))
    }

    fn socket(name: &str, is_on: bool, power: u64) -> SmartDevice {
        SmartDevice::Socket(SmartSocet::new(name.to_string(), is_on, power))
    }

    fn empty_room() -> Room {
        Room::new("Пустая".to_string(), vec![])
    }

    fn room_with_devices() -> Room {
        Room::new(
            "Гостиная".to_string(),
            vec![thermometer("Термометр", 22), socket("Розетка", true, 500)],
        )
    }

    // --- Конструктор ---

    #[test]
    fn test_new_sets_name_and_devices() {
        let r = room_with_devices();
        assert_eq!(r.name, "Гостиная");
        assert_eq!(r.devices.len(), 2);
    }

    #[test]
    fn test_new_empty_devices() {
        let r = empty_room();
        assert_eq!(r.name, "Пустая");
        assert!(r.devices.is_empty());
    }

    #[test]
    fn test_get_device_returns_first() {
        let r = room_with_devices();
        assert_eq!(r.get_device(0).name(), "Термометр");
    }

    #[test]
    fn test_get_device_returns_second() {
        let r = room_with_devices();
        assert_eq!(r.get_device(1).name(), "Розетка");
    }

    #[test]
    fn test_get_device_returns_correct_variant() {
        let r = room_with_devices();
        assert!(matches!(r.get_device(0), SmartDevice::Thermometer(_)));
        assert!(matches!(r.get_device(1), SmartDevice::Socket(_)));
    }

    #[test]
    #[should_panic(expected = "выходит за пределы комнаты")]
    fn test_get_device_index_out_of_bounds() {
        let r = room_with_devices();
        r.get_device(2);
    }

    #[test]
    #[should_panic(expected = "выходит за пределы комнаты")]
    fn test_get_device_on_empty_room_panics() {
        let r = empty_room();
        r.get_device(0);
    }

    #[test]
    #[should_panic(expected = "Гостиная")]
    fn test_get_device_panic_message_contains_room_name() {
        let r = room_with_devices();
        r.get_device(99);
    }

    #[test]
    #[should_panic(expected = "Индекс устройства 5")]
    fn test_get_device_panic_message_contains_index() {
        let r = room_with_devices();
        r.get_device(5);
    }

    #[test]
    fn test_get_device_mut_allows_modification() {
        let mut r = Room::new("Кухня".to_string(), vec![socket("Розетка", false, 100)]);
        if let SmartDevice::Socket(s) = r.get_device_mut(0) {
            s.turn_on();
            s.set_power(750);
        } else {
            panic!("Ожидался вариант Socket");
        }
        if let SmartDevice::Socket(s) = r.get_device(0) {
            assert!(s.is_enabled());
            assert_eq!(s.get_power(), 750);
        } else {
            panic!("Ожидался вариант Socket");
        }
    }

    #[test]
    fn test_get_device_mut_thermometer_temperature() {
        let mut r = Room::new("Спальня".to_string(), vec![thermometer("Термометр", 18)]);
        if let SmartDevice::Thermometer(t) = r.get_device_mut(0) {
            t.set_temperature(25);
        } else {
            panic!("Ожидался вариант Thermometer");
        }
        if let SmartDevice::Thermometer(t) = r.get_device(0) {
            assert_eq!(t.get_temperature(), 25);
        } else {
            panic!("Ожидался вариант Thermometer");
        }
    }

    #[test]
    fn test_get_device_mut_does_not_change_name() {
        let mut r = room_with_devices();
        let _ = r.get_device_mut(0);
        assert_eq!(r.get_device(0).name(), "Термометр");
    }

    #[test]
    #[should_panic(expected = "выходит за пределы комнаты")]
    fn test_get_device_mut_index_out_of_bounds() {
        let mut r = room_with_devices();
        r.get_device_mut(2);
    }

    #[test]
    #[should_panic(expected = "выходит за пределы комнаты")]
    fn test_get_device_mut_on_empty_room_panics() {
        let mut r = empty_room();
        r.get_device_mut(0);
    }

    #[test]
    fn test_print_report_does_not_panic_with_devices() {
        room_with_devices().print_report();
    }

    #[test]
    fn test_print_report_does_not_panic_empty_room() {
        empty_room().print_report();
    }

    #[test]
    fn test_print_report_does_not_panic_after_mutation() {
        let mut r = room_with_devices();
        if let SmartDevice::Socket(s) = r.get_device_mut(1) {
            s.turn_off();
        }
        r.print_report();
    }

    #[test]
    fn test_many_devices_iteration() {
        let r = Room::new(
            "Дом".to_string(),
            vec![
                thermometer("T1", 10),
                socket("S1", true, 100),
                thermometer("T2", 20),
                socket("S2", false, 200),
            ],
        );
        assert_eq!(r.devices.len(), 4);
        let names: Vec<&str> = r.devices.iter().map(|d| d.name()).collect();
        assert_eq!(names, vec!["T1", "S1", "T2", "S2"]);
    }

    #[test]
    fn test_devices_vec_is_public_and_accessible() {
        let r = room_with_devices();
        assert_eq!(r.devices[0].name(), "Термометр");
        assert_eq!(r.devices[1].name(), "Розетка");
    }

    #[test]
    #[should_panic(expected = "Индекс устройства 5 выходит за пределы комнаты Гостиная!")]
    fn test_get_device_panic_message_full_text() {
        let r = room_with_devices();
        r.get_device(5);
    }
}
