pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|value| value % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|byte| **byte != 0).count()
}

/// Удаляет пробельные символы и приводит строку к нижнему регистру.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Среднее арифметическое положительных значений.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<i64> = values.iter().copied().filter(|value| *value > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    positives.iter().sum::<i64>() as f64 / positives.len() as f64
}

/// Возвращает сумму двух чтений одного значения.
pub fn use_after_free() -> i32 {
    let value = 42;
    value + value
}
