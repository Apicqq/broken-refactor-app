# Отчёт по broken-app

Эталон: [reference-app](https://github.com/Apicqq/verified-app), commit `4fa8121eec742b207663433eb8eaf326b34d9385`.

## Исправления

- удалён выход `sum_even` за границы среза;
- устранена утечка `leak_buffer`;
- удалён use-after-free и весь `unsafe` из библиотечной логики;
- исправлены Unicode-нормализация и среднее положительных значений;
- `static mut` заменён на `AtomicU64`, гонка устранена;
- добавлены регрессионные тесты, включая пустые входы, Unicode и конкурентный счётчик.

## Оптимизации

- Fibonacci: рекурсивный `O(2ⁿ)` заменён итеративным `O(n)`;
- dedup: повторные линейные поиски и сортировки заменены `HashSet` и одной итоговой сортировкой, `O(n + u log u)`;
- `normalize`: одна возвращаемая строка, пробельные символы удаляются на месте через `String::retain`;
- `average_positive`: один проход без временного `Vec`.

## Производительность

| Сценарий | До | После | Ускорение |
| --- | ---: | ---: | ---: |
| `fib_32` | 4,2974 ms | 15,327 ns | 280381,0× |
| `dedup_10k` | 8,4821 ms | 102,52 µs | 82,7× |
| `normalize_65k` | 98,444 µs | 71,033 µs | 1,39× |

Подробные результаты: `artifacts/benchmarks/before.txt`, `artifacts/benchmarks/after.txt` и `artifacts/profile/README.md`.

## Проверка

- `cargo test --all-targets`: 14/14 интеграционных тестов;
- `cargo +nightly miri test`: успешно, UB не обнаружено;
- Valgrind: `definitely lost: 0`, `ERROR SUMMARY: 0`;
- ASan: 14/14 тестов, предупреждений нет;
- TSan: конкурентный тест завершён с кодом `0`, предупреждений нет;
- `cargo fmt --check` и Clippy с `-D warnings`: успешно.

Команды и логи находятся в `artifacts/initial/` и `artifacts/diagnostics/{before,after}/`.
