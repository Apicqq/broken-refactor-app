# Диагностика до исправлений

Проверена ревизия `d6ed417` на `x86_64-unknown-linux-gnu`.

## Инструменты

```text
rustc 1.100.0-nightly (fb6531d55 2026-08-23)
cargo 1.100.0-nightly (e8cb624d5 2026-08-22)
miri 0.1.0 (fb6531d550 2026-08-23)
GNU gdb 15.1
valgrind 3.22.0
```

## Результаты

| Инструмент | Проверка | Результат |
| --- | --- | --- |
| GDB | `average_positive([-5, 5, 15])` | На строке деления `sum = 15`, но делитель равен длине всего среза `3`; результат `5` вместо `10` |
| Miri | `sums_even_numbers` | UB в `src/lib.rs:11`: `get_unchecked(values.len())` |
| Valgrind | `counts_non_zero_bytes` | 5 байт definitely lost; источник — `leak_buffer`, `src/lib.rs:23` |
| ASan | временный вызов `use_after_free()` | heap-use-after-free в `src/lib.rs:62` |
| TSan | временный вызов `race_increment(100_000, 4)` | Две гонки на глобальном `COUNTER`, `src/concurrency.rs:15` |

Полезная нагрузка ASan и TSan находилась в отдельном диагностическом Cargo-драйвере и не меняла репозиторий:

```rust
// ASan
unsafe { println!("{}", broken_app::use_after_free()); }

// TSan
println!("{}", broken_app::concurrency::race_increment(100_000, 4));
```

Полные значимые фрагменты вывода и команды сохранены в соседних файлах. Ненулевые коды завершения ожидаемы: каждый инструмент обнаружил подготовленный дефект.
