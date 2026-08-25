#!/usr/bin/env bash
set -euo pipefail

mkdir -p artifacts/profile
CARGO_PROFILE_BENCH_DEBUG=true cargo bench --bench criterion --no-run

bench_binary="$({ find target/release/deps -maxdepth 1 -type f -executable -name 'criterion-*' -printf '%T@ %p\n'; } | sort -nr | sed -n '1s/^[^ ]* //p')"
profile_svg="target/flamegraph-after.svg"
flamegraph --output "$profile_svg" \
    --title 'broken-app hot paths after' \
    -- "$bench_binary" --profile-time 5 --bench

convert -background white "$profile_svg" artifacts/profile/flamegraph-after.png

perf report --stdio --no-children --input perf.data \
    --sort symbol --percent-limit 0.5
