use rand;

pub fn get_random_number(start: i64, end: i64) -> i64 {
    if start == end {
        return start;
    }
    let min = start.min(end);
    let max = start.max(end);
    rand::random_range(min..=max)
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

pub trait ReportDevice {
    fn name(&self) -> &str;
    fn print_status(&self);
}
