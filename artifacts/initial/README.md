# Исходное состояние

Состояние зафиксировано `2026-08-25T06:04:23+03:00` до исправлений и изменений бенчмарков.

## Ревизии

- broken-app: `1a9b149dd6d78f0fe9bf37f8f3a959f59e47259a`
- reference-app: `4fa8121eec742b207663433eb8eaf326b34d9385`

## Окружение

```text
rustc 1.94.0 (4a4ef493e 2026-03-02)
cargo 1.94.0 (85eff7c80 2026-01-15)
host: x86_64-unknown-linux-gnu
LLVM version: 21.1.8
```

## Команды и результаты

| Проект | Команда | Код завершения | Результат |
| --- | --- | ---: | --- |
| broken-app | `cargo check --all-targets` | 0 | Проверка прошла с тремя предупреждениями `unsafe_op_in_unsafe_fn` и одним предупреждением о неиспользуемом импорте |
| broken-app | `cargo test --tests` | 101 | Тест `average_positive` упал; `sum_even` аварийно завершил процесс после выхода `get_unchecked` за границы среза |
| broken-app | `cargo run --quiet --bin demo` | 134 | Процесс аварийно завершился в `sum_even`, не успев вывести штатный результат |
| reference-app | `cargo check --all-targets` | 0 | Проверка прошла с одним предупреждением о неиспользуемом импорте |
| reference-app | `cargo test --tests` | 0 | Все 7 интеграционных тестов прошли успешно |
| reference-app | `cargo run --quiet --bin demo` | 0 | Получен ожидаемый вывод, сохранённый в `reference-demo.txt` |

Полный значимый вывод команд сохранён рядом с этим файлом.
