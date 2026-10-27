//! Модуль устройств умного дома.
//!
//! Объединяет типы и функции для моделирования умного дома:
//! устройства (`thermometer`, `socket`, `smart_devices_mod`), комнаты
//! (`room`) и дом в целом (`home`). Общие функции и типаж `ReportDevice`
//! лежат в модуле `functions`.

/// Общие функции и типаж `ReportDevice`.
///
/// Содержит `get_random_number` и трейт `ReportDevice`,
/// который реализуют все устройства.
pub mod functions;

/// Умный дом — верхний уровень иерархии.
///
/// Содержит `SmartHouse` — коллекцию комнат.
pub mod home;

/// Комнаты умного дома.
///
/// Содержит `Room` — коллекцию устройств с именем.
pub mod room;

/// Перечисление устройств умного дома.
///
/// Содержит `SmartDevice` — обёртку над `SmartTermometr` и `SmartSocet`,
/// реализующую `ReportDevice`.
pub mod smart_devices_mod;

/// Умная розетка.
///
/// Содержит `SmartSocet` — устройство с состоянием включения и мощностью.
pub mod socket;

/// Умный термометр.
///
/// Содержит `SmartTermometr` — устройство с текущей температурой.
pub mod thermometer;
