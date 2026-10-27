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
    pub fn get_name(&self) -> &str {
        &self.name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sets_fields() {
        let t = SmartTermometr::new("Кухня".to_string(), 22);
        assert_eq!(t.get_name(), "Кухня");
        assert_eq!(t.get_temperature(), 22);
    }

    #[test]
    fn test_get_temperature() {
        let t = SmartTermometr::new("Зал".to_string(), -5);
        assert_eq!(t.get_temperature(), -5);
    }

    #[test]
    fn test_set_temperature_updates_value() {
        let mut t = SmartTermometr::new("Улица".to_string(), 0);
        t.set_temperature(15);
        assert_eq!(t.get_temperature(), 15);
    }

    #[test]
    fn test_set_temperature_allows_negative() {
        let mut t = SmartTermometr::new("Морозилка".to_string(), 5);
        t.set_temperature(-20);
        assert_eq!(t.get_temperature(), -20);
    }

    #[test]
    fn test_set_temperature_does_not_change_name() {
        let mut t = SmartTermometr::new("Офис".to_string(), 21);
        t.set_temperature(25);
        assert_eq!(t.get_name(), "Офис");
    }

    #[test]
    fn test_trait_name_returns_str() {
        let t = SmartTermometr::new("Гостиная".to_string(), 20);
        let name: &str = t.name();
        assert_eq!(name, "Гостиная");
    }

    #[test]
    fn test_trait_name_matches_get_name() {
        let t = SmartTermometr::new("Балкон".to_string(), 10);
        assert_eq!(t.name(), t.get_name());
    }

    #[test]
    fn test_print_status_does_not_panic() {
        let t = SmartTermometr::new("Тест".to_string(), 42);
        t.print_status();
    }
}
