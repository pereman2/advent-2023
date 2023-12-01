#!/bin/bash
output=$(cargo test --profile=release-with-debug --no-run --message-format=json)

# Use jq to parse the output and extract the executable path for src/lib.rs
executable_path=$(echo "$output" | jq -r 'select(.profile.test == true) | select(.target.kind[] == "lib") | .executable')

echo $executable_path
perf record -g --call-graph=dwarf -F 99 hyperfine --warmup 3 "${executable_path} day_1_2" "${executable_path} day_1_2_speed"
