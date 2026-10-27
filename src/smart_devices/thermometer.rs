use super::functions::ReportDevice;

#[derive(Debug)]
pub struct SmartTermometr {
    pub name: String,
    pub temperature: i64,
}

impl SmartTermometr {
    pub fn new(name: String, temperature: i64) -> Self {
        Self { name, temperature }
    }
    pub fn get_temperature(&self) -> i64 {
        self.temperature
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_temperature(&mut self, temperature: i64) {
        self.temperature = temperature;
    }
}

impl ReportDevice for SmartTermometr {
    fn name(&self) -> &str {
        &self.name
    }

    fn print_status(&self) {
        println!(
            "Термометр {}: Текущая температура: {}°C",
            self.name,
            self.get_temperature()
        );
    }
}
