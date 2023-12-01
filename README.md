# reports

## Day 1

```bash
Benchmark 1: /home/peristocles/fun/ad2023/advent/target/release-with-debug/deps/advent-db84e2febb37137e day_1_2
  Time (mean ± σ):     941.5 µs ± 135.7 µs    [User: 369.8 µs, System: 672.2 µs]
  Range (min … max):   798.0 µs … 1528.5 µs    1865 runs
Benchmark 2: /home/peristocles/fun/ad2023/advent/target/release-with-debug/deps/advent-db84e2febb37137e day_1_2_speed_1
  Time (mean ± σ):     807.1 µs ± 144.9 µs    [User: 254.7 µs, System: 628.0 µs]
  Range (min … max):   673.8 µs … 1640.7 µs    1668 runs
Benchmark 3: /home/peristocles/fun/ad2023/advent/target/release-with-debug/deps/advent-db84e2febb37137e day_1_2_speed_2
  Time (mean ± σ):     692.2 µs ± 141.7 µs    [User: 242.1 µs, System: 649.2 µs]
  Range (min … max):   566.4 µs … 1624.4 µs    2138 runs
Summary
  /home/peristocles/fun/ad2023/advent/target/release-with-debug/deps/advent-db84e2febb37137e day_1_2_speed_2 ran
    1.17 ± 0.32 times faster than /home/peristocles/fun/ad2023/advent/target/release-with-debug/deps/advent-db84e2febb37137e day_1_2_speed_1
    1.36 ± 0.34 times faster than /home/peristocles/fun/ad2023/advent/target/release-with-debug/deps/advent-db84e2febb37137e day_1_2
```
meh, can go faster I guess. I still don't know all the intricacies of rust functions

day_1_2_speed_2 removed most branching and unnecessary functions that did copies.