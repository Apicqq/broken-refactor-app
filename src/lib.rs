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
    let mut output = input.to_lowercase();
    output.retain(|character| !character.is_whitespace());
    output
}

/// Среднее арифметическое положительных значений.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .copied()
        .filter(|value| *value > 0)
        .fold((0_i64, 0_usize), |(sum, count), value| {
            (sum + value, count + 1)
        });
    if count == 0 {
        return 0.0;
    }
    sum as f64 / count as f64
}

/// Возвращает сумму двух чтений одного значения.
pub fn use_after_free() -> i32 {
    let value = 42;
    value + value
}
