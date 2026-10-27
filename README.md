//! # SmartHome
//!
//! `smart_home` — библиотека для моделирования умного дома и автоматизаций.
//!
//! Позволяет создавать устройства (термометры и розетки), объединять их
//! в комнаты, а комнаты — в дом, а также получать отчёты о состоянии
//! всей иерархии.
//!
//! ## Основные возможности
//!
//! - Создание устройств умного дома: [`SmartTermometr`](smart_devices::thermometer::SmartTermometr)
//!   и [`SmartSocet`](smart_devices::socket::SmartSocet).
//! - Объединение разнотипных устройств в [`SmartDevice`](smart_devices::smart_devices_mod::SmartDevice).
//! - Создание комнат ([`Room`](smart_devices::room::Room)) с именем и списком устройств.
//! - Создание дома ([`SmartHouse`](smart_devices::home::SmartHouse)) с именем и списком комнат.
//! - Доступ к комнатам и устройствам по индексу (или мутабельной ссылки).
//! - Вывод отчётов по отдельному устройству, комнате и дому целиком.
//! - Утилита [`get_random_number`](smart_devices::functions::get_random_number)
//!   для генерации случайных чисел в диапазоне.
//!
//! ## Пример
//!
//! ```
//! use smart_home::smart_devices::home::SmartHouse;
//! use smart_home::smart_devices::room::Room;
//! use smart_home::smart_devices::smart_devices_mod::SmartDevice;
//! use smart_home::smart_devices::thermometer::SmartTermometr;
//! use smart_home::smart_devices::socket::SmartSocet;
//!
//! let house = SmartHouse::new(
//!     "Мой дом".to_string(),
//!     vec![Room::new(
//!         "Гостиная".to_string(),
//!         vec![
//!             SmartDevice::Thermometer(SmartTermometr::new("Термометр".to_string(), 22)),
//!             SmartDevice::Socket(SmartSocet::new("Розетка".to_string(), true, 500)),
//!         ],
//!     )],
//! );
//!
//! assert_eq!(house.name, "Мой дом");
//! assert_eq!(house.get_room(0).name, "Гостиная");
//! house.print_report();
//! ```
//!
//! ## Структура модулей
//!
//! - [`smart_devices`] — всё, что связано с устройствами:
//!   - [`thermometer`](smart_devices::thermometer) — термометр;
//!   - [`socket`](smart_devices::socket) — розетка;
//!   - [`smart_devices_mod`](smart_devices::smart_devices_mod) — перечисление устройств;
//!   - [`room`](smart_devices::room) — комната;
//!   - [`home`](smart_devices::home) — дом;
//!   - [`functions`](smart_devices::functions) — общие функции и типаж `ReportDevice`.

/// Модуль устройств умного дома.
///
/// Содержит типы устройств, комнаты, дом и общие функции.