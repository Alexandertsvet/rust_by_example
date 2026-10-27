use super::functions::ReportDevice;

/// Умная розетка — устройство с именем, состоянием включения и мощностью.
///
/// Ключевая особенность: [`get_power`](SmartSocet::get_power) возвращает
/// `0`, если розетка выключена, даже если [`current_power`](SmartSocet::current_power)
/// задан. При включении ранее установленная мощность снова становится видимой.
/// Это имитирует физическое поведение: выключенная розетка не потребляет энергию.
///
/// Реализует типаж [`ReportDevice`], поэтому может использоваться
/// в коллекциях и отображаться в отчётах
/// устройств (например, в [`Room`](super::room::Room) и
/// [`SmartHouse`](super::home::SmartHouse)).
///
/// # Examples
///
/// ```
/// use smart_home::smart_devices::socket::SmartSocet;
///
/// let mut s = SmartSocet::new("Кухня".to_string(), true, 1500);
/// assert_eq!(s.get_name(), "Кухня");
/// assert!(s.is_enabled());
/// assert_eq!(s.get_power(), 1500);
///
/// s.turn_off();
/// assert_eq!(s.get_power(), 0); // выключена — мощность скрыта
///
/// s.turn_on();
/// assert_eq!(s.get_power(), 1500); // включена — мощность снова видна
/// ```
#[derive(Debug)]
pub struct SmartSocet {
    /// Имя розетки (например, «Кухня» или «Гостиная»).
    pub name: String,
    /// Состояние: `true` — включена, `false` — выключена.
    pub is_on: bool,
    /// Заданная мощность в ваттах. Видна через [`get_power`](SmartSocet::get_power)
    /// только при `is_on == true`.
    pub current_power: u64,
}

impl SmartSocet {
    /// Создаёт новую розетку с заданным именем, состоянием и мощностью.
    ///
    /// Если `is_on == false`, мощность будет скрыта до включения,
    /// но сохранена в поле `current_power`.
    ///
    /// # Аргументы
    ///
    /// * `name` — имя устройства.
    /// * `is_on` — начальное состояние (`true` — включена).
    /// * `current_power` — заданная мощность в ваттах.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// // Включённая розетка сразу показывает мощность
    /// let s = SmartSocet::new("Офис".to_string(), true, 500);
    /// assert_eq!(s.get_power(), 500);
    ///
    /// // Выключенная — мощность скрыта, но сохранена
    /// let off = SmartSocet::new("Спальня".to_string(), false, 500);
    /// assert_eq!(off.get_power(), 0);
    /// assert_eq!(off.current_power, 500); // поле доступно напрямую
    /// ```
    pub fn new(name: String, is_on: bool, current_power: u64) -> Self {
        Self {
            name,
            is_on,
            current_power,
        }
    }

    /// Возвращает имя розетки как строковый срез.
    ///
    /// Возвращается `&str`, а не `String`, чтобы избежать лишнего
    /// клонирования при чтении.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let s = SmartSocet::new("Гостиная".to_string(), true, 100);
    /// assert_eq!(s.get_name(), "Гостиная");
    /// ```
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Выключает розетку.
    ///
    /// После вызова [`get_power`](SmartSocet::get_power) будет возвращать `0`,
    /// пока розетку снова не включат. Заданная мощность при этом не теряется.
    ///
    /// Идемпотентно: повторный вызов на уже выключенной розетке ничего не меняет.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let mut s = SmartSocet::new("Тест".to_string(), true, 100);
    /// s.turn_off();
    /// assert!(!s.is_enabled());
    /// assert_eq!(s.get_power(), 0);
    /// ```
    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    /// Включает розетку.
    ///
    /// После вызова [`get_power`](SmartSocet::get_power) снова возвращает
    /// заданную мощность (`current_power`).
    ///
    /// Идемпотентно: повторный вызов на уже включённой розетке ничего не меняет.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let mut s = SmartSocet::new("Тест".to_string(), false, 100);
    /// s.turn_on();
    /// assert!(s.is_enabled());
    /// assert_eq!(s.get_power(), 100);
    /// ```
    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    /// Возвращает `true`, если розетка включена.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let s = SmartSocet::new("Тест".to_string(), true, 100);
    /// assert!(s.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.is_on
    }

    /// Возвращает текущую потребляемую мощность в ваттах.
    ///
    /// Если розетка выключена (`is_on == false`), возвращается `0`,
    /// независимо от значения `current_power`. Если включена —
    /// возвращается `current_power`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let on = SmartSocet::new("Тест".to_string(), true, 2000);
    /// assert_eq!(on.get_power(), 2000);
    ///
    /// let off = SmartSocet::new("Тест".to_string(), false, 2000);
    /// assert_eq!(off.get_power(), 0);
    /// ```
    pub fn get_power(&self) -> u64 {
        if self.is_on { self.current_power } else { 0 }
    }

    /// Устанавливает заданную мощность в ваттах.
    ///
    /// Меняет только поле `current_power`, не трогая состояние `is_on`.
    /// Если розетка выключена, новое значение сохранится, но
    /// [`get_power`](SmartSocet::get_power) продолжит возвращать `0`
    /// до включения.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let mut s = SmartSocet::new("Тест".to_string(), true, 100);
    /// s.set_power(750);
    /// assert_eq!(s.get_power(), 750);
    ///
    /// // Выключенная розетка скрывает новую мощность
    /// s.turn_off();
    /// s.set_power(300);
    /// assert_eq!(s.get_power(), 0);
    /// s.turn_on();
    /// assert_eq!(s.get_power(), 300);
    /// ```
    pub fn set_power(&mut self, power: u64) {
        self.current_power = power;
    }
}

impl ReportDevice for SmartSocet {
    /// Возвращает имя устройства для отчёта.
    ///
    /// Совпадает с результатом [`get_name`](SmartSocet::get_name).
    fn name(&self) -> &str {
        &self.name
    }

    /// Печатает статус розетки в стандартный поток вывода.
    ///
    /// Формат строки:
    /// `Розетка <имя>: Статус: <Включена|Выключена>, Мощность: <Вт> Вт`.
    /// Для выключенной розетки мощность всегда `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::socket::SmartSocet;
    /// use smart_home::smart_devices::functions::ReportDevice;
    ///
    /// let s = SmartSocet::new("Кухня".to_string(), true, 1500);
    /// s.print_status(); // Розетка Кухня: Статус: Включена, Мощность: 1500 Вт
    /// ```
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
