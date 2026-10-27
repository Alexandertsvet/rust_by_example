use super::room::Room;

/// Умный дом — именованная коллекция комнат.
///
/// Верхний уровень иерархии: дом содержит [`Room`], каждая комната —
/// [`SmartDevice`](super::smart_devices_mod::SmartDevice). `SmartHouse`
/// агрегирует отчёты по всем комнатам через
/// [`print_report`](SmartHouse::print_report).
///
/// Комнаты можно получать по индексу (включая мутабельный доступ),
/// а через них — добираться до устройств:
/// `house.get_room(0).get_device(1)`.
///
/// # Examples
///
/// ```
/// use smart_home::smart_devices::home::SmartHouse;
/// use smart_home::smart_devices::room::Room;
/// use smart_home::smart_devices::smart_devices_mod::SmartDevice;
/// use smart_home::smart_devices::thermometer::SmartTermometr;
///
/// let house = SmartHouse::new(
///     "Мой дом".to_string(),
///     vec![
///         Room::new(
///             "Гостиная".to_string(),
///             vec![SmartDevice::Thermometer(SmartTermometr::new("Т".to_string(), 22))],
///         ),
///     ],
/// );
///
/// assert_eq!(house.name, "Мой дом");
/// assert_eq!(house.rooms.len(), 1);
/// assert_eq!(house.get_room(0).name, "Гостиная");
/// ```
pub struct SmartHouse {
    /// Имя дома (например, «Мой дом» или «Дача»).
    pub name: String,
    /// Комнаты в доме. Порядок сохраняется.
    pub rooms: Vec<Room>,
}

impl SmartHouse {
    /// Создаёт новый дом с заданным именем и списком комнат.
    ///
    /// # Аргументы
    ///
    /// * `name` — имя дома.
    /// * `rooms` — вектор комнат (может быть пустым).
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::home::SmartHouse;
    ///
    /// let empty = SmartHouse::new("Пустой дом".to_string(), vec![]);
    /// assert!(empty.rooms.is_empty());
    /// ```
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    /// Возвращает ссылку на комнату по индексу.
    ///
    /// # Panics
    ///
    /// Паникует, если `index >= self.rooms.len()`. Сообщение содержит
    /// индекс и имя дома, например:
    /// `"Индекс комнаты 5 выходит за пределы дома Мой дом!"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::home::SmartHouse;
    /// use smart_home::smart_devices::room::Room;
    ///
    /// let house = SmartHouse::new(
    ///     "Дом".to_string(),
    ///     vec![Room::new("Кухня".to_string(), vec![])],
    /// );
    /// assert_eq!(house.get_room(0).name, "Кухня");
    /// ```
    ///
    /// ```should_panic
    /// use smart_home::smart_devices::home::SmartHouse;
    ///
    /// let house = SmartHouse::new("Пустой".to_string(), vec![]);
    /// house.get_room(0); // паника: индекс вне границ
    /// ```
    pub fn get_room(&self, index: usize) -> &Room {
        if index >= self.rooms.len() {
            panic!(
                "Индекс комнаты {} выходит за пределы дома {}!",
                index, self.name
            );
        }
        &self.rooms[index]
    }

    /// Возвращает мутабельную ссылку на комнату по индексу.
    ///
    /// Позволяет изменять комнату на месте: добавлять/удалять устройства
    /// через публичное поле `devices` или менять состояние существующих
    /// устройств через [`Room::get_device_mut`].
    ///
    /// # Panics
    ///
    /// Паникует, если `index >= self.rooms.len()`. Сообщение содержит
    /// индекс и имя дома, как у [`get_room`](SmartHouse::get_room).
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::home::SmartHouse;
    /// use smart_home::smart_devices::room::Room;
    /// use smart_home::smart_devices::smart_devices_mod::SmartDevice;
    /// use smart_home::smart_devices::socket::SmartSocet;
    ///
    /// let mut house = SmartHouse::new(
    ///     "Дом".to_string(),
    ///     vec![Room::new("Офис".to_string(), vec![])],
    /// );
    ///
    /// // Добавляем устройство в комнату через мутабельную ссылку
    /// house.get_room_mut(0).devices.push(
    ///     SmartDevice::Socket(SmartSocet::new("Р".to_string(), false, 100)),
    /// );
    ///
    /// // Меняем состояние устройства «на месте»
    /// if let SmartDevice::Socket(s) = house.get_room_mut(0).get_device_mut(0) {
    ///     s.turn_on();
    ///     s.set_power(750);
    /// }
    /// ```
    ///
    /// ```should_panic
    /// use smart_home::smart_devices::home::SmartHouse;
    ///
    /// let mut house = SmartHouse::new("Пустой".to_string(), vec![]);
    /// house.get_room_mut(0); // паника: индекс вне границ
    /// ```
    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        if index >= self.rooms.len() {
            panic!(
                "Индекс комнаты {} выходит за пределы дома {}!",
                index, self.name
            );
        }
        &mut self.rooms[index]
    }

    /// Печатает общий отчёт по дому: заголовок и отчёт каждой комнаты.
    ///
    /// Формат:
    /// ```text
    /// =================================
    ///
    /// === ОТЧЕТ ПО ДОМУ: <имя> ===
    /// <отчёт комнаты 1>
    /// <отчёт комнаты 2>
    /// ...
    /// =================================
    ///
    /// ```
    /// Отчёт каждой комнаты печатается её собственным
    /// [`Room::print_report`](Room::print_report), который, в свою очередь,
    /// вызывает [`print_status`](ReportDevice::print_status) каждого устройства.
    /// Для дома без комнат печатается только заголовок и закрывающая линия.
    ///
    /// # Examples
    ///
    /// ```
    /// use smart_home::smart_devices::home::SmartHouse;
    /// use smart_home::smart_devices::room::Room;
    /// use smart_home::smart_devices::smart_devices_mod::SmartDevice;
    /// use smart_home::smart_devices::thermometer::SmartTermometr;
    ///
    /// let house = SmartHouse::new(
    ///     "Дача".to_string(),
    ///     vec![Room::new(
    ///         "Спальня".to_string(),
    ///         vec![SmartDevice::Thermometer(SmartTermometr::new("Т".to_string(), 18))],
    ///     )],
    /// );
    /// house.print_report();
    /// // =================================
    /// //
    /// // === ОТЧЕТ ПО ДОМУ: Дача ===
    /// // Комната: Спальня
    /// // Термометр Т: Текущая температура: 18°C
    /// // =================================
    /// ```
    pub fn print_report(&self) {
        println!("=================================\n");
        println!("=== ОТЧЕТ ПО ДОМУ: {} ===", self.name);
        for room in &self.rooms {
            room.print_report();
        }
        println!("=================================\n");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::smart_devices::functions::ReportDevice;
    use crate::smart_devices::room::Room;
    use crate::smart_devices::smart_devices_mod::SmartDevice;
    use crate::smart_devices::socket::SmartSocet;
    use crate::smart_devices::thermometer::SmartTermometr;

    // --- Вспомогательные конструкторы ---

    fn thermometer(name: &str, temp: i64) -> SmartDevice {
        SmartDevice::Thermometer(SmartTermometr::new(name.to_string(), temp))
    }

    fn socket(name: &str, is_on: bool, power: u64) -> SmartDevice {
        SmartDevice::Socket(SmartSocet::new(name.to_string(), is_on, power))
    }

    fn empty_room(name: &str) -> Room {
        Room::new(name.to_string(), vec![])
    }

    fn room_with_devices(name: &str) -> Room {
        Room::new(
            name.to_string(),
            vec![thermometer("Термометр", 22), socket("Розетка", true, 500)],
        )
    }

    fn empty_house() -> SmartHouse {
        SmartHouse::new("Пустой дом".to_string(), vec![])
    }

    fn house_with_rooms() -> SmartHouse {
        SmartHouse::new(
            "Мой дом".to_string(),
            vec![room_with_devices("Гостиная"), empty_room("Кладовая")],
        )
    }

    // --- Конструктор ---

    #[test]
    fn test_new_sets_name_and_rooms() {
        let h = house_with_rooms();
        assert_eq!(h.name, "Мой дом");
        assert_eq!(h.rooms.len(), 2);
    }

    #[test]
    fn test_new_empty_rooms() {
        let h = empty_house();
        assert_eq!(h.name, "Пустой дом");
        assert!(h.rooms.is_empty());
    }

    // --- get_room ---

    #[test]
    fn test_get_room_returns_first() {
        let h = house_with_rooms();
        assert_eq!(h.get_room(0).name, "Гостиная");
    }

    #[test]
    fn test_get_room_returns_second() {
        let h = house_with_rooms();
        assert_eq!(h.get_room(1).name, "Кладовая");
    }

    #[test]
    fn test_get_room_gives_access_to_devices() {
        let h = house_with_rooms();
        let room = h.get_room(0);
        assert_eq!(room.devices.len(), 2);
        assert_eq!(room.get_device(0).name(), "Термометр");
        assert_eq!(room.get_device(1).name(), "Розетка");
    }

    #[test]
    #[should_panic(expected = "выходит за пределы дома")]
    fn test_get_room_index_out_of_bounds() {
        let h = house_with_rooms();
        h.get_room(2);
    }

    #[test]
    #[should_panic(expected = "выходит за пределы дома")]
    fn test_get_room_on_empty_house_panics() {
        let h = empty_house();
        h.get_room(0);
    }

    #[test]
    #[should_panic(expected = "Мой дом")]
    fn test_get_room_panic_message_contains_house_name() {
        let h = house_with_rooms();
        h.get_room(99);
    }

    #[test]
    #[should_panic(expected = "Индекс комнаты 5")]
    fn test_get_room_panic_message_contains_index() {
        let h = house_with_rooms();
        h.get_room(5);
    }

    // --- get_room_mut ---

    #[test]
    fn test_get_room_mut_allows_modification() {
        let mut h = SmartHouse::new(
            "Дом".to_string(),
            vec![Room::new(
                "Кухня".to_string(),
                vec![socket("Розетка", false, 100)],
            )],
        );
        // Меняем устройство внутри комнаты через мутабельную ссылку на комнату
        if let SmartDevice::Socket(s) = h.get_room_mut(0).get_device_mut(0) {
            s.turn_on();
            s.set_power(750);
        } else {
            panic!("Ожидался вариант Socket");
        }
        // Проверяем через неизменяемый доступ
        if let SmartDevice::Socket(s) = h.get_room(0).get_device(0) {
            assert!(s.is_enabled());
            assert_eq!(s.get_power(), 750);
        } else {
            panic!("Ожидался вариант Socket");
        }
    }

    #[test]
    fn test_get_room_mut_can_add_device() {
        let mut h = SmartHouse::new("Дом".to_string(), vec![empty_room("Кухня")]);
        h.get_room_mut(0).devices.push(thermometer("Новый", 20));
        assert_eq!(h.get_room(0).devices.len(), 1);
        assert_eq!(h.get_room(0).get_device(0).name(), "Новый");
    }

    #[test]
    fn test_get_room_mut_does_not_change_house_name() {
        let mut h = house_with_rooms();
        let _ = h.get_room_mut(0);
        assert_eq!(h.name, "Мой дом");
    }

    #[test]
    #[should_panic(expected = "выходит за пределы дома")]
    fn test_get_room_mut_index_out_of_bounds() {
        let mut h = house_with_rooms();
        h.get_room_mut(2);
    }

    #[test]
    #[should_panic(expected = "выходит за пределы дома")]
    fn test_get_room_mut_on_empty_house_panics() {
        let mut h = empty_house();
        h.get_room_mut(0);
    }

    // --- print_report ---

    #[test]
    fn test_print_report_does_not_panic_with_rooms() {
        house_with_rooms().print_report();
    }

    #[test]
    fn test_print_report_does_not_panic_empty_house() {
        empty_house().print_report();
    }

    #[test]
    fn test_print_report_does_not_panic_empty_room_inside() {
        let h = SmartHouse::new("Дом".to_string(), vec![empty_room("Пустая комната")]);
        h.print_report();
    }

    #[test]
    fn test_print_report_does_not_panic_after_mutation() {
        let mut h = house_with_rooms();
        if let SmartDevice::Socket(s) = h.get_room_mut(0).get_device_mut(1) {
            s.turn_off();
        }
        h.print_report();
    }

    // --- Вложенность и множественные комнаты ---

    #[test]
    fn test_many_rooms_iteration() {
        let h = SmartHouse::new(
            "Большой дом".to_string(),
            vec![
                empty_room("R1"),
                room_with_devices("R2"),
                empty_room("R3"),
                room_with_devices("R4"),
            ],
        );
        assert_eq!(h.rooms.len(), 4);
        let names: Vec<&str> = h.rooms.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["R1", "R2", "R3", "R4"]);
    }

    #[test]
    fn test_nested_access_house_room_device() {
        let h = house_with_rooms();
        // Дом -> комната 0 -> устройство 0 (термометр)
        let device = h.get_room(0).get_device(0);
        assert_eq!(device.name(), "Термометр");
        if let SmartDevice::Thermometer(t) = device {
            assert_eq!(t.get_temperature(), 22);
        } else {
            panic!("Ожидался вариант Thermometer");
        }
    }

    #[test]
    fn test_rooms_vec_is_public_and_accessible() {
        let h = house_with_rooms();
        assert_eq!(h.rooms[0].name, "Гостиная");
        assert_eq!(h.rooms[1].name, "Кладовая");
        // Прямой доступ к устройствам через публичные поля
        assert_eq!(h.rooms[0].devices.len(), 2);
    }
}
