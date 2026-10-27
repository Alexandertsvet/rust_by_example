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
