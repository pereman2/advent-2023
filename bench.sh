#!/bin/bash

# PERF="perf record -g --call-graph=dwarf -F 99 "
# PERF="perf stat "
# $PERF hyperfine --warmup 10 --min-runs 10000 "${executable_path} day_1_2_speed_2" "${executable_path} part_two"
# $PERF cargo bench "day_2"
$PERF cargo bench "day_3"

