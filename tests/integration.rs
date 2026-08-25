use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even, use_after_free};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sums_empty_slice() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn counts_empty_buffer() {
    assert_eq!(leak_buffer(&[]), 0);
}

#[test]
fn reads_owned_value_safely() {
    assert_eq!(use_after_free(), 84);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::fast_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::fast_fib(10), 55);
}

#[test]
fn algorithms_handle_boundaries() {
    assert_eq!(algo::fast_dedup(&[]), Vec::<u64>::new());
    assert_eq!(algo::fast_fib(0), 0);
    assert_eq!(algo::fast_fib(1), 1);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_all_whitespace() {
    assert_eq!(
        normalize(" Hello\tWorld\nRust\u{2003}! "),
        "helloworldrust!"
    );
}

#[test]
fn normalize_expanding_lowercase() {
    assert_eq!(normalize(" İ\u{2003}RUST "), "i\u{307}rust");
}

#[test]
fn averages_only_positive() {
    assert!((average_positive(&[-5, 5, 15]) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn average_without_positive_values_is_zero() {
    assert_eq!(average_positive(&[-5, 0]), 0.0);
    assert_eq!(average_positive(&[]), 0.0);
}

#[test]
fn concurrent_increment_is_correct() {
    assert_eq!(broken_app::concurrency::race_increment(1_000, 4), 4_000);
}
