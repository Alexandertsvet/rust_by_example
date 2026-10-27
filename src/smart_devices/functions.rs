use rand;
/// Возвращает случайное целое число из диапазона `[start, end]` (включительно).
///
/// Границы можно передавать в любом порядке: функция сама определяет
/// минимум и максимум. Если `start == end`, возвращается это же значение
/// без обращения к генератору случайных чисел.
///
/// # Аргументы
///
/// * `start` — одна из границ диапазона (включительно).
/// * `end` — другая граница диапазона (включительно).
///
/// # Возвращаемое значение
///
/// Случайное число `i64`, удовлетворяющее условию
/// `min(start, end) <= результат <= max(start, end)`.
///
/// # Panics
///
/// В текущей реализации функция не паникует: `rand::random_range`
/// корректно обрабатывает пустой диапазон только при `start == end`,
/// а этот случай отсекается заранее. Однако если диапазон окажется
/// пустым из-за переполнения при вычислении `max - min` (например,
/// `start = i64::MIN`, `end = i64::MAX`), поведение зависит от версии
/// `rand` — в некоторых версиях возможна паника. См. раздел «Ограничения».
///
/// # Examples
///
/// ```
/// use smart_home::smart_devices::functions::get_random_number;
///
/// // Обычный диапазон
/// let n = get_random_number(1, 10);
/// assert!((1..=10).contains(&n));
///
/// // Границы в обратном порядке — результат тот же
/// let m = get_random_number(10, 1);
/// assert!((1..=10).contains(&m));
///
/// // Равные границы — всегда одно и то же число
/// assert_eq!(get_random_number(42, 42), 42);
/// ```
///
/// # Ограничения
///
/// * Диапазон должен помещаться в `i64`. Если `start = i64::MIN`
///   и `end = i64::MAX`, длина диапазона превышает `i64::MAX`,
///   и поведение не гарантировано.
/// * Функция использует глобальный ГПСЧ из крейта `rand`
///   (`rand::random_range`), поэтому для воспроизводимых тестов
///   лучше передавать свой `Rng` — см. примечание ниже.
pub fn get_random_number(start: i64, end: i64) -> i64 {
    if start == end {
        return start;
    }
    let min = start.min(end);
    let max = start.max(end);
    rand::random_range(min..=max)
}

/// Общий интерфейс устройства умного дома.
///
/// Реализуется конкретными устройствами ([`SmartTermometr`](super::thermometer::SmartTermometr),
/// [`SmartSocet`](super::socket::SmartSocet)) и их обёрткой
/// [`SmartDevice`](super::smart_devices_mod::SmartDevice).
/// Позволяет единообразно получать имя и печатать статус устройства
/// в отчётах комнат и дома.
pub trait ReportDevice {
    /// Возвращает имя устройства.
    fn name(&self) -> &str;

    /// Печатает текущий статус устройства в стандартный поток вывода.
    fn print_status(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_bounds() {
        // Если границы равны, должно всегда возвращаться это же число
        assert_eq!(get_random_number(42, 42), 42);
        assert_eq!(get_random_number(-5, -5), -5);
    }

    #[test]
    fn test_inverted_bounds() {
        // Функция должна работать, даже если start > end
        let num = get_random_number(10, 1);
        assert!((1..=10).contains(&num));
    }

    #[test]
    fn test_range_inclusion() {
        // Проверяем, что границы диапазона действительно могут выпасть
        let mut hit_min = false;
        let mut hit_max = false;

        for _ in 0..1000 {
            let num = get_random_number(1, 3);
            assert!((1..=3).contains(&num));
            if num == 1 {
                hit_min = true;
            }
            if num == 3 {
                hit_max = true;
            }
        }

        // За 1000 итераций границы 1 и 3 должны выпасть хотя бы раз
        assert!(hit_min, "Ни разу не выпала нижняя граница (1)");
        assert!(hit_max, "Ни разу не выпала верхняя граница (3)");
    }
}
