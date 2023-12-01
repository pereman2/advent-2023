# reports

## Day 1

```bash
Benchmark 1: day_1_2
  Time (mean ± σ):     960.1 µs ± 137.5 µs    [User: 356.0 µs, System: 665.6 µs]
  Range (min … max):   802.3 µs … 1522.5 µs    1850 runs
Benchmark 2: day_1_2_speed_1
  Time (mean ± σ):     836.7 µs ± 154.8 µs    [User: 266.4 µs, System: 602.2 µs]
  Range (min … max):   665.1 µs … 1430.3 µs    1913 runs
Benchmark 3: day_1_2_speed_2
  Time (mean ± σ):     716.3 µs ± 151.7 µs    [User: 234.8 µs, System: 630.1 µs]
  Range (min … max):   572.1 µs … 2210.4 µs    2015 runs
Summary
  day_1_2_speed_2 ran
    1.17 ± 0.33 times faster than day_1_2_speed_1
    1.34 ± 0.34 times faster than day_1_2
```
meh, can go faster I guess. I still don't know all the intricacies of rust functions

day_1_2_speed_2 removed most branching and unnecessary functions that did copies.