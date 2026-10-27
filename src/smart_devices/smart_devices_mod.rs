use super::functions::ReportDevice;
use super::socket::SmartSocet;
use super::thermometer::SmartTermometr;

/// Устройство умного дома — перечисление поддерживаемых типов.
///
/// Каждый вариант оборачивает конкретную структуру устройства:
/// [`SmartTermometr`] или [`SmartSocet`]. Перечисление реализует
/// типаж [`ReportDevice`], делегируя вызовы `name` и `print_status`
/// внутреннему устройству через `match`.
///
/// Это даёт «статический полиморфизм»: `SmartDevice` можно хранить
/// в `Vec<SmartDevice>` (например, в [`Room`](super::room::Room))
/// без `Box<dyn ReportDevice>`, а при необходимости — использовать
/// и как `dyn ReportDevice`.
///
/// # Examples
///
/// ```
/// use smart_home::smart_devices::smart_devices_mod::SmartDevice;
/// use smart_home::smart_devices::thermometer::SmartTermometr;
/// use smart_home::smart_devices::socket::SmartSocet;
/// use smart_home::smart_devices::functions::ReportDevice;
///
/// let t = SmartDevice::Thermometer(SmartTermometr::new("Кухня".to_string(), 22));
/// let s = SmartDevice::Socket(SmartSocet::new("Офис".to_string(), true, 300));
///
/// assert_eq!(t.name(), "Кухня");
/// assert_eq!(s.name(), "Офис");
///
/// let devices: Vec<SmartDevice> = vec![t, s];
/// for d in &devices {
///     d.print_status();
/// }
/// ```
#[derive(Debug)]
pub enum SmartDevice {
    /// Термометр — см. [`SmartTermometr`].
    Thermometer(SmartTermometr),
    /// Розетка — см. [`SmartSocet`].
    Socket(SmartSocet),
}

impl ReportDevice for SmartDevice {
    /// Возвращает имя внутреннего устройства.
    ///
    /// Делегирует вызов соответствующему варианту:
    /// [`SmartTermometr::name`](super::thermometer::SmartTermometr)
    /// или [`SmartSocet::name`](super::socket::SmartSocet).
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::smart_devices_mod::SmartDevice;
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    /// use smart_home::smart_devices::functions::ReportDevice;
    ///
    /// let d = SmartDevice::Thermometer(SmartTermometr::new("Зал".to_string(), 20));
    /// assert_eq!(d.name(), "Зал");
    /// ```
    fn name(&self) -> &str {
        match self {
            SmartDevice::Thermometer(t) => t.name(),
            SmartDevice::Socket(s) => s.name(),
        }
    }

    /// Печатает статус внутреннего устройства.
    ///
    /// Делегирует вызов соответствующему варианту:
    /// [`SmartTermometr::print_status`](super::thermometer::SmartTermometr)
    /// или [`SmartSocet::print_status`](super::socket::SmartSocet).
    /// Формат вывода зависит от типа устройства.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::smart_devices_mod::SmartDevice;
    /// use smart_home::smart_devices::socket::SmartSocet;
    /// use smart_home::smart_devices::functions::ReportDevice;
    ///
    /// let d = SmartDevice::Socket(SmartSocet::new("Кухня".to_string(), true, 1500));
    /// d.print_status(); // Розетка Кухня: Статус: Включена, Мощность: 1500 Вт
    /// ```
    fn print_status(&self) {
        match self {
            SmartDevice::Thermometer(t) => t.print_status(),
            SmartDevice::Socket(s) => s.print_status(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Вспомогательные функции ---

    fn thermometer(name: &str, temp: i64) -> SmartDevice {
        SmartDevice::Thermometer(SmartTermometr::new(name.to_string(), temp))
    }

    fn socket(name: &str, is_on: bool, power: u64) -> SmartDevice {
        SmartDevice::Socket(SmartSocet::new(name.to_string(), is_on, power))
    }

    #[test]
    fn test_create_thermometer_variant() {
        let d: SmartDevice = thermometer("Кухня", 22);
        assert!(matches!(d, SmartDevice::Thermometer(_)));
    }

    #[test]
    fn test_create_socket_variant() {
        let d = socket("Гостиная", true, 100);
        assert!(matches!(d, SmartDevice::Socket(_)));
    }

    #[test]
    fn test_name_for_thermometer() {
        let d = thermometer("Спальня", 18);
        assert_eq!(d.name(), "Спальня");
    }

    #[test]
    fn test_name_for_socket() {
        let d = socket("Офис", true, 500);
        assert_eq!(d.name(), "Офис");
    }

    #[test]
    fn test_name_returns_correct_variant_name() {
        let t = thermometer("Тест", 20);
        let s = socket("Тест", true, 100);
        assert_eq!(t.name(), "Тест");
        assert_eq!(s.name(), "Тест");
    }

    #[test]
    fn test_print_status_thermometer_does_not_panic() {
        let d = thermometer("Тест", 25);
        d.print_status();
    }

    #[test]
    fn test_print_status_socket_on_does_not_panic() {
        let d = socket("Тест", true, 100);
        d.print_status();
    }

    #[test]
    fn test_print_status_socket_off_does_not_panic() {
        let d = socket("Тест", false, 100);
        d.print_status();
    }

    #[test]
    fn test_thermometer_inner_state_accessible() {
        let d = thermometer("Кухня", 22);
        if let SmartDevice::Thermometer(t) = &d {
            assert_eq!(t.get_temperature(), 22);
            assert_eq!(t.get_name(), "Кухня");
        } else {
            panic!("Ожидался вариант Thermometer");
        }
    }

    #[test]
    fn test_socket_inner_state_accessible() {
        let d = socket("Гостиная", true, 1500);
        if let SmartDevice::Socket(s) = &d {
            assert!(s.is_enabled());
            assert_eq!(s.get_power(), 1500);
            assert_eq!(s.get_name(), "Гостиная");
        } else {
            panic!("Ожидался вариант Socket");
        }
    }

    #[test]
    fn test_socket_off_inner_power_is_zero() {
        let d = socket("Тест", false, 1500);
        if let SmartDevice::Socket(s) = &d {
            assert_eq!(s.get_power(), 0);
        } else {
            panic!("Ожидался вариант Socket");
        }
    }

    #[test]
    fn test_debug_format() {
        let d = thermometer("Кухня", 22);
        let s: String = format!("{:?}", d);
        assert!(s.contains("Thermometer"));
        assert!(s.contains("Кухня"));
    }
}
