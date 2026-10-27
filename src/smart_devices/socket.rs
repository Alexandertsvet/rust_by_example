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
    pub fn get_name(&self) -> &str {
        &self.name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sets_fields() {
        let s = SmartSocet::new("Кухня".to_string(), true, 220);
        assert_eq!(s.get_name(), "Кухня");
        assert!(s.is_enabled());
        assert_eq!(s.get_power(), 220);
    }

    #[test]
    fn test_new_off_device() {
        let s = SmartSocet::new("Спальня".to_string(), false, 500);
        assert!(!s.is_enabled());
        assert_eq!(s.get_power(), 0);
    }

    #[test]
    fn test_get_name() {
        let s = SmartSocet::new("Гостиная".to_string(), true, 110);
        assert_eq!(s.get_name(), "Гостиная");
    }

    #[test]
    fn test_turn_off() {
        let mut s = SmartSocet::new("Тест".to_string(), true, 110);
        s.turn_off();
        assert!(!s.is_enabled());
    }

    #[test]
    fn test_turn_on() {
        let mut s = SmartSocet::new("Тест".to_string(), false, 100);
        s.turn_on();
        assert!(s.is_enabled());
    }

    #[test]
    fn test_turn_off_twice_is_idempotent() {
        let mut s = SmartSocet::new("Тест".to_string(), true, 100);
        s.turn_off();
        s.turn_off();
        assert!(!s.is_enabled());
    }

    #[test]
    fn test_turn_on_twice_is_idempotent() {
        let mut s = SmartSocet::new("Тест".to_string(), false, 100);
        s.turn_on();
        s.turn_on();
        assert!(s.is_enabled());
    }

    #[test]
    fn test_toggle_off_on_off() {
        let mut s = SmartSocet::new("Тест".to_string(), true, 100);
        s.turn_off();
        s.turn_on();
        s.turn_off();
        assert!(!s.is_enabled());
    }

    #[test]
    fn test_get_power_when_on() {
        let s = SmartSocet::new("Тест".to_string(), true, 2000);
        assert_eq!(s.get_power(), 2000);
    }

    #[test]
    fn test_get_power_when_off_is_zero() {
        let s = SmartSocet::new("Тест".to_string(), false, 2000);
        assert_eq!(s.get_power(), 0);
    }

    #[test]
    fn test_get_power_zero_current_power() {
        let s = SmartSocet::new("Тест".to_string(), true, 0);
        assert_eq!(s.get_power(), 0);
    }

    #[test]
    fn test_set_power_updates_value() {
        let mut s = SmartSocet::new("Тест".to_string(), true, 100);
        s.set_power(750);
        assert_eq!(s.get_power(), 750);
    }

    #[test]
    fn test_set_power_while_off_keeps_power_hidden() {
        let mut s = SmartSocet::new("Тест".to_string(), false, 100);
        s.set_power(750);
        assert_eq!(s.get_power(), 0);
        s.turn_on();
        assert_eq!(s.get_power(), 750);
    }

    #[test]
    fn test_set_power_does_not_change_state() {
        let mut s = SmartSocet::new("Тест".to_string(), true, 100);
        s.set_power(300);
        assert!(s.is_enabled());
    }

    #[test]
    fn test_set_power_does_not_change_name() {
        let mut s = SmartSocet::new("Офис".to_string(), true, 100);
        s.set_power(300);
        assert_eq!(s.get_name(), "Офис");
    }

    #[test]
    fn test_set_power_max_u64() {
        let mut s = SmartSocet::new("Тест".to_string(), true, 0);
        s.set_power(u64::MAX);
        assert_eq!(s.get_power(), u64::MAX);
    }

    #[test]
    fn test_trait_name_returns_str() {
        let s = SmartSocet::new("Балкон".to_string(), true, 50);
        let name: &str = s.name();
        assert_eq!(name, "Балкон");
    }

    #[test]
    fn test_trait_name_matches_get_name() {
        let s = SmartSocet::new("Балкон".to_string(), true, 50);
        assert_eq!(s.name(), s.get_name());
    }

    #[test]
    fn test_print_status_on_does_not_panic() {
        let s = SmartSocet::new("Тест".to_string(), true, 42);
        s.print_status();
    }

    #[test]
    fn test_print_status_off_does_not_panic() {
        let s = SmartSocet::new("Тест".to_string(), false, 42);
        s.print_status();
    }
}
