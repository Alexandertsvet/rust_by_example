use super::functions::ReportDevice;

#[derive(Debug)]
pub struct SmartSocet {
    pub name: String,
    pub is_on: bool,
    pub current_power: u64,
}

impl SmartSocet {
    pub fn new(name: String, is_on: bool, current_power: u64) -> Self {
        Self {
            name,
            is_on,
            current_power,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }
    pub fn is_enabled(&self) -> bool {
        self.is_on
    }
    pub fn get_power(&self) -> u64 {
        if self.is_on { self.current_power } else { 0 }
    }
    pub fn set_power(&mut self, power: u64) {
        self.current_power = power;
    }
}

impl ReportDevice for SmartSocet {
    fn name(&self) -> &str {
        &self.name
    }

    fn print_status(&self) {
        println!(
            "Розетка {}: Статус: {}, Мощность: {} Вт",
            self.name,
            if self.is_enabled() {
                "Включена"
            } else {
                "Выключена"
            },
            self.get_power()
        );
    }
}
