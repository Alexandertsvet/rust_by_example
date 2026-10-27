use super::functions::ReportDevice;

/// Умный термометр — устройство с именем и текущей температурой.
///
/// Хранит температуру как `i64` (градусы Цельсия). Отрицательные значения
/// допустимы и обозначают температуру ниже нуля.
///
/// Реализует типаж [`ReportDevice`], поэтому может использоваться
/// в коллекциях и отображаться в отчётах
/// устройств (например, в [`Room`](super::room::Room) и
/// [`SmartHouse`](super::home::SmartHouse)).
///
/// # Examples
///
/// ```
/// use smart_home::smart_devices::thermometer::SmartTermometr;
///
/// let mut t = SmartTermometr::new("Кухня".to_string(), 22);
/// assert_eq!(t.get_name(), "Кухня");
/// assert_eq!(t.get_temperature(), 22);
///
/// t.set_temperature(-5);
/// assert_eq!(t.get_temperature(), -5);
/// ```
#[derive(Debug)]
pub struct SmartTermometr {
    /// Имя термометра (например, «Кухня» или «Спальня»).
    pub name: String,
    /// Текущая температура в градусах Цельсия. Может быть отрицательной.
    pub temperature: i64,
}

impl SmartTermometr {
    /// Создаёт новый термометр с заданным именем и температурой.
    ///
    /// # Аргументы
    ///
    /// * `name` — имя устройства.
    /// * `temperature` — начальная температура в °C (может быть отрицательной).
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    ///
    /// let t = SmartTermometr::new("Улица".to_string(), -10);
    /// assert_eq!(t.get_temperature(), -10);
    /// ```
    pub fn new(name: String, temperature: i64) -> Self {
        Self { name, temperature }
    }

    /// Возвращает текущую температуру в °C.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    ///
    /// let t = SmartTermometr::new("Зал".to_string(), 20);
    /// assert_eq!(t.get_temperature(), 20);
    /// ```
    pub fn get_temperature(&self) -> i64 {
        self.temperature
    }

    /// Возвращает имя термометра как строковый срез.
    ///
    /// Возвращается `&str`, а не `String`, чтобы избежать лишнего
    /// клонирования при чтении.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    ///
    /// let t = SmartTermometr::new("Спальня".to_string(), 18);
    /// assert_eq!(t.get_name(), "Спальня");
    /// ```
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Устанавливает новую температуру.
    ///
    /// Значение может быть любым `i64`, включая отрицательные —
    /// ограничений на диапазон нет.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    ///
    /// let mut t = SmartTermometr::new("Морозилка".to_string(), 5);
    /// t.set_temperature(-20);
    /// assert_eq!(t.get_temperature(), -20);
    /// ```
    pub fn set_temperature(&mut self, temperature: i64) {
        self.temperature = temperature;
    }
}

impl ReportDevice for SmartTermometr {
    /// Возвращает имя устройства для отчёта.
    ///
    /// Совпадает с результатом [`get_name`](SmartTermometr::get_name).
    fn name(&self) -> &str {
        &self.name
    }

    /// Печатает статус термометра в стандартный поток вывода.
    ///
    /// Формат строки:
    /// `Термометр <имя>: Текущая температура: <температура>°C`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    /// use smart_home::smart_devices::functions::ReportDevice;
    ///
    /// let t = SmartTermometr::new("Кухня".to_string(), 22);
    /// t.print_status(); // выведет: Термометр Кухня: Текущая температура: 22°C
    /// ```
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
