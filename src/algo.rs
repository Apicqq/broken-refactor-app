use std::collections::HashSet;

/// Возвращает отсортированные уникальные значения за O(n + u log u).
pub fn fast_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());
    for &value in values {
        if seen.insert(value) {
            out.push(value);
        }
    }
    out.sort_unstable();
    out
}

/// Итеративное вычисление числа Фибоначчи за O(n).
pub fn fast_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut previous = 0;
            let mut current = 1;
            for _ in 2..=n {
                (previous, current) = (current, previous + current);
            }
            current
        }
    }
}
