mod day1;
mod day2;
mod day3;
mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

mod advent {
    const NUMBERS_NAMES: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const NUMBERS_STRINGS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    use crate::day1::day1::get_input;
    use std::{
        cmp::max,
        collections::HashMap,
        fs::read_to_string,
        path::Path,
        sync::Arc,
        thread::{self, JoinHandle, ScopedJoinHandle},
    };
    pub fn _read_file(path: String) -> String {
        let path = Path::new(&path);
        return read_to_string(&path).unwrap();
    }
    pub fn day_1_1() -> u32 {
        let contents = _read_file("input_1_1".to_string());
        let mut sum = 0;
        for line in contents.lines() {
            let mut first = 0;
            let mut last = 0;
            let mut start = true;
            for chr in line.chars() {
                if chr.is_numeric() {
                    if start {
                        start = false;
                        first = chr.to_digit(10).unwrap();
                    }
                    last = chr.to_digit(10).unwrap();
                }
            }
            sum += (first * 10) + last;
        }
        return sum;
    }

    #[inline(never)]
    pub fn day_1_2() -> u32 {
        let contents = _read_file("input_1_1".to_string());
        let mut sum = 0;
        for line in contents.split('\n') {
            let mut first = 0;
            let mut last = 0;
            let mut i: usize = 0;
            while i < line.len() {
                if line[i..line.len()].starts_with("one") {
                    last = 1;
                } else if line[i..line.len()].starts_with("two") {
                    last = 2;
                } else if line[i..line.len()].starts_with("three") {
                    last = 3;
                } else if line[i..line.len()].starts_with("four") {
                    last = 4;
                } else if line[i..line.len()].starts_with("five") {
                    last = 5;
                } else if line[i..line.len()].starts_with("six") {
                    last = 6;
                } else if line[i..line.len()].starts_with("seven") {
                    last = 7;
                } else if line[i..line.len()].starts_with("eight") {
                    last = 8;
                } else if line[i..line.len()].starts_with("nine") {
                    last = 9;
                } else {
                    let chr = line.chars().nth(i).unwrap();
                    if chr.is_numeric() {
                        last = chr.to_digit(10).unwrap();
                    }
                }
                if first == 0 && last != 0 {
                    first = last;
                }
                i += 1
            }
            sum += (first * 10) + last;
        }
        return sum;
    }

    #[inline(never)]
    pub fn day_1_2_speed_1() -> u32 {
        let mut sum = 0;
        for line in get_input().iter() {
            let mut first = 0;
            let mut last = 0;
            let mut i: usize = 0;
            while i < line.len() {
                if line[i..line.len()].starts_with("one") {
                    first = 1;
                } else if line[i..line.len()].starts_with("two") {
                    first = 2;
                } else if line[i..line.len()].starts_with("three") {
                    first = 3;
                } else if line[i..line.len()].starts_with("four") {
                    first = 4;
                } else if line[i..line.len()].starts_with("five") {
                    first = 5;
                } else if line[i..line.len()].starts_with("six") {
                    first = 6;
                } else if line[i..line.len()].starts_with("seven") {
                    first = 7;
                } else if line[i..line.len()].starts_with("eight") {
                    first = 8;
                } else if line[i..line.len()].starts_with("nine") {
                    first = 9;
                } else {
                    let chr = line.chars().nth(i).unwrap();
                    if chr.is_numeric() {
                        first = chr.to_digit(10).unwrap();
                    }
                }
                if first != 0 {
                    break;
                }
                i += 1;
            }
            i = line.len() - 1;
            while i >= 0 {
                if line[i..line.len()].starts_with("one") {
                    last = 1;
                } else if line[i..line.len()].starts_with("two") {
                    last = 2;
                } else if line[i..line.len()].starts_with("three") {
                    last = 3;
                } else if line[i..line.len()].starts_with("four") {
                    last = 4;
                } else if line[i..line.len()].starts_with("five") {
                    last = 5;
                } else if line[i..line.len()].starts_with("six") {
                    last = 6;
                } else if line[i..line.len()].starts_with("seven") {
                    last = 7;
                } else if line[i..line.len()].starts_with("eight") {
                    last = 8;
                } else if line[i..line.len()].starts_with("nine") {
                    last = 9;
                } else {
                    let chr = line.chars().nth(i).unwrap();
                    if chr.is_numeric() {
                        last = chr.to_digit(10).unwrap();
                    }
                }
                if last != 0 {
                    break;
                }
                i -= 1;
            }
            sum += format!("{}{}", first, last).parse::<u32>().unwrap();
        }
        return sum;
    }

    #[inline(never)]
    pub fn day_1_2_speed_2() -> u32 {
        let mut sum = 0;
        for line in get_input().iter() {
            let mut first = 0;
            let mut last = 0;
            let line = line.as_bytes();
            for (i, chr) in line.iter().enumerate() {
                let rest = &line[i..];
                // let rest = line[i..line.len()]; RETURNS A COPY WTF!
                // REMOVE BRANCHING
                first = first | (rest.starts_with("one".as_bytes()) as u32 * 1);
                first = first | (rest.starts_with("two".as_bytes()) as u32 * 2);
                first = first | (rest.starts_with("three".as_bytes()) as u32 * 3);
                first = first | (rest.starts_with("four".as_bytes()) as u32 * 4);
                first = first | (rest.starts_with("five".as_bytes()) as u32 * 5);
                first = first | (rest.starts_with("six".as_bytes()) as u32 * 6);
                first = first | (rest.starts_with("seven".as_bytes()) as u32 * 7);
                first = first | (rest.starts_with("eight".as_bytes()) as u32 * 8);
                first = first | (rest.starts_with("nine".as_bytes()) as u32 * 9);
                first = first | chr.is_ascii_digit() as u32 * ((chr - b'0') as u32);
                if first != 0 {
                    break;
                }
            }
            for (i, chr) in line.iter().rev().enumerate() {
                let i = line.len() - i - 1;
                let rest = &line[..=i];
                last = last | (rest.ends_with("one".as_bytes()) as u32 * 1);
                last = last | (rest.ends_with("two".as_bytes()) as u32 * 2);
                last = last | (rest.ends_with("three".as_bytes()) as u32 * 3);
                last = last | (rest.ends_with("four".as_bytes()) as u32 * 4);
                last = last | (rest.ends_with("five".as_bytes()) as u32 * 5);
                last = last | (rest.ends_with("six".as_bytes()) as u32 * 6);
                last = last | (rest.ends_with("seven".as_bytes()) as u32 * 7);
                last = last | (rest.ends_with("eight".as_bytes()) as u32 * 8);
                last = last | (rest.ends_with("nine".as_bytes()) as u32 * 9);
                last = last | chr.is_ascii_digit() as u32 * ((chr - b'0') as u32);
                if last != 0 {
                    break;
                }
            }
            sum += (first * 10) + last;
        }
        return sum;
    }

    #[inline(never)]
    pub fn day_2_1() -> usize {
        let contents = crate::day2::day2::get_input();
        let mut m = HashMap::new();
        m.insert("green", 13);
        m.insert("blue", 14);
        m.insert("red", 12);
        let mut sum = 0;
        let start = 8;
        let mut game_offset = 0;
        for (game, line) in contents.iter().enumerate() {
            if game + 1 == 10 || game + 1 == 100 {
                game_offset += 1;
            }
            let line = &line[start + (game_offset)..];
            let mut possible = true;
            for reveal in line.split("; ") {
                for cube in reveal.split(", ") {
                    let mut sp = cube.split(' ');
                    let number = sp.next().unwrap().parse::<u32>().unwrap();
                    let color = sp.next().unwrap();
                    possible &= number <= *m.get(color).unwrap();
                }
            }
            sum += possible as usize * (game + 1);
        }
        return sum;
    }

    #[inline(never)]
    pub fn day_2_2() -> u32 {
        let contents = crate::day2::day2::get_input();
        let mut sum = 0;
        let start = 8;
        let mut game_offset = 0;
        for (game, line) in contents.iter().enumerate() {
            if game + 1 == 10 || game + 1 == 100 {
                game_offset += 1;
            }
            let line = &line[start + (game_offset)..];
            let mut minim: HashMap<&str, u32> =
                HashMap::from([("green", 0), ("blue", 0), ("red", 0)]);
            for reveal in line.split("; ") {
                for cube in reveal.split(", ") {
                    let mut sp = cube.split(' ');
                    let number = sp.next().unwrap().parse::<u32>().unwrap();
                    let color = sp.next().unwrap();
                    let minim_color = minim.get(color).unwrap();
                    minim.insert(color, max(*minim_color, number));
                }
            }
            let mut power = 1;
            for v in minim.values() {
                power *= *v;
            }
            sum += power;
        }
        return sum;
    }

    #[inline(never)]
    pub fn day_2_2_speed_1() -> u32 {
        let contents = crate::day2::day2::get_input();
        let mut sum = 0;
        let start = 8;
        let mut game_offset = 0;
        for (game, line) in contents.iter().enumerate() {
            if game + 1 == 10 || game + 1 == 100 {
                game_offset += 1;
            }
            let line = &line[start + (game_offset)..];
            let mut green = 0;
            let mut blue = 0;
            let mut red = 0;
            for reveal in line.split("; ") {
                for cube in reveal.split(", ") {
                    let mut sp = cube.split(' ');
                    let number = sp.next().unwrap().parse::<u32>().unwrap();
                    let color = sp.next().unwrap().chars().next().unwrap();
                    let mut v = match color {
                        'g' => &mut green,
                        'b' => &mut blue,
                        'r' => &mut red,
                        _ => &mut green,
                    };
                    *v = max(*v, number);
                }
            }
            sum += green * blue * red;
        }
        return sum;
    }
}

mod test {
    use crate::advent::*;

    #[test]
    fn day_1() {
        println!("res {}", day_1_1());
        println!("res {}", day_1_2());
        assert_eq!(day_1_2(), day_1_2_speed_1());
        assert_eq!(day_1_2(), day_1_2_speed_2());
    }
    #[test]
    fn day_2() {
        println!("res {}", day_2_1());
        println!("res {}", day_2_2());
        assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_3() {
        use crate::day3::day3::*;
        println!("res {}", day_3_1());
        println!("res {}", day_3_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_4() {
        use crate::day4::*;
        println!("res {}", day_4_1());
        println!("res {}", day_4_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_5() {
        use crate::day5::*;
        println!("res {}", day_5_1());
        println!("res {}", day_5_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_6() {
        use crate::day6::*;
        println!("res {}", day_6_1());
        println!("res {}", day_6_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_7() {
        use crate::day7::{day_7_1, day_7_2};
        println!("res {}", day_7_1());
        println!("res {}", day_7_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_8() {
        use crate::day8::{day_8_1, day_8_2};
        println!("res {}", day_8_1());
        println!("res {}", day_8_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_9() {
        use crate::day9::{day_9_1, day_9_2};
        println!("res {}", day_9_1());
        println!("res {}", day_9_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }
    #[test]
    fn day_10() {
        use crate::day10::{day_10_1, day_10_2};
        // println!("res {}", day_10_1());
        // println!("res {}", day_10_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_11() {
        use crate::day11::{day_11_1, day_11_2};
        println!("res {}", day_11_1());
        println!("res {}", day_11_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_12() {
        use crate::day12::*;
        println!("res {}", day_12_1());
        println!("res {}", day_12_2());
        // assert_eq!(day_2_2(), day_2_2_speed_1());
    }

    #[test]
    fn day_13() {
        use crate::day13::*;
        println!("res {}", day_13_1());
        println!("res {}", day_13_2());
    }

    #[test]
    fn day_14() {
        use crate::day14::*;
        println!("res {}", day_14_1());
        println!("res {}", day_14_2());
    }

    #[test]
    fn day_15() {
        use crate::day15::*;
        println!("res {}", day_15_1());
        println!("res {}", day_15_2());
    }

    #[test]
    fn day_16() {
        use crate::day16::*;
        println!("res {}", day_16_1());
        let res2 = day_16_2();
        println!("res {}", res2);
        assert!(res2 == day_16_2_speed_1());
    }

}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day6::*;
use day7::{day_7_1, day_7_2};

use crate::day5::{day_5_1, day_5_2};

pub fn day_1(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_1_1", |b| b.iter(|| black_box(advent::day_1_2())));
    g.bench_function("day_1_2", |b| b.iter(|| black_box(advent::day_1_2())));
    g.bench_function("day_1_2_speed_1", |b| {
        b.iter(|| black_box(advent::day_1_2_speed_1()))
    });
    g.bench_function("day_1_2_speed_2", |b| {
        b.iter(|| black_box(advent::day_1_2_speed_2()))
    });
}

pub fn day_2(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_2_1", |b| b.iter(|| black_box(advent::day_2_2())));
    g.bench_function("day_2_2", |b| b.iter(|| black_box(advent::day_2_2())));
    g.bench_function("day_2_2_speed_1", |b| {
        b.iter(|| black_box(advent::day_2_2_speed_1()))
    });
}

pub fn day_3(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_3_1", |b| {
        b.iter(|| black_box(crate::day3::day3::day_3_1()))
    });
    g.bench_function("day_3_2", |b| {
        b.iter(|| black_box(crate::day3::day3::day_3_2()))
    });
    // g.bench_function("day_2_2_speed_1", |b| {
    //     b.iter(|| black_box(advent::day_2_2_speed_1()))
    // });
}

pub fn day_4(c: &mut Criterion) {
    use crate::day4::*;
    let mut g = c.benchmark_group("day2");
    // g.bench_function("day_4_1", |b| b.iter(|| black_box(day_4_1())));
    g.bench_function("day_4_2", |b| b.iter(|| black_box(day_4_2())));
    g.bench_function("day_4_2_speed_1", |b| {
        b.iter(|| black_box(day_4_2_speed_1()))
    });
}

pub fn day_5(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_5_1", |b| b.iter(|| black_box(day_5_1())));
    g.bench_function("day_5_2", |b| b.iter(|| black_box(day_5_2())));
}

pub fn day_6(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_6_1", |b| b.iter(|| black_box(day_6_1())));
    g.bench_function("day_6_2", |b| b.iter(|| black_box(day_6_2())));
    // g.bench_function("day_6_2_speed_1", |b| b.iter(|| black_box(day_5_6_speed_1())));
}

pub fn day_7(c: &mut Criterion) {
    let mut g = c.benchmark_group("day7");
    g.bench_function("day_7_1", |b| b.iter(|| black_box(day_7_1())));
    g.bench_function("day_7_2", |b| b.iter(|| black_box(day_7_2())));
}

pub fn day_8(c: &mut Criterion) {
    let mut g = c.benchmark_group("day8");
    use crate::day8::*;
    g.bench_function("day_8_1", |b| b.iter(|| black_box(day_8_1())));
    g.bench_function("day_8_2", |b| b.iter(|| black_box(day_8_2())));
}

pub fn day_9(c: &mut Criterion) {
    let mut g = c.benchmark_group("day9");
    use crate::day9::*;
    g.bench_function("day_9_1", |b| b.iter(|| black_box(day_9_1())));
    g.bench_function("day_9_2", |b| b.iter(|| black_box(day_9_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}

pub fn day_10(c: &mut Criterion) {
    let mut g = c.benchmark_group("day10");
    use crate::day10::*;
    g.bench_function("day_10_1", |b| b.iter(|| black_box(day_10_1())));
    g.bench_function("day_10_2", |b| b.iter(|| black_box(day_10_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}

pub fn day_11(c: &mut Criterion) {
    let mut g = c.benchmark_group("day11");
    use crate::day11::*;
    g.bench_function("day_11_1", |b| b.iter(|| black_box(day_11_1())));
    g.bench_function("day_11_2", |b| b.iter(|| black_box(day_11_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}

pub fn day_12(c: &mut Criterion) {
    let mut g = c.benchmark_group("day12");
    use crate::day12::*;
    g.bench_function("day_12_1", |b| b.iter(|| black_box(day_12_1())));
    g.bench_function("day_12_2", |b| b.iter(|| black_box(day_12_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}

pub fn day_13(c: &mut Criterion) {
    let mut g = c.benchmark_group("day13");
    use crate::day13::*;
    g.bench_function("day_13_1", |b| b.iter(|| black_box(day_13_1())));
    g.bench_function("day_13_2", |b| b.iter(|| black_box(day_13_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}

pub fn day_14(c: &mut Criterion) {
    let mut g = c.benchmark_group("day14");
    use crate::day14::*;
    g.bench_function("day_14_1", |b| b.iter(|| black_box(day_14_1())));
    g.bench_function("day_14_2", |b| b.iter(|| black_box(day_14_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}

pub fn day_15(c: &mut Criterion) {
    let mut g = c.benchmark_group("day15");
    use crate::day15::*;
    g.bench_function("day_15_1", |b| b.iter(|| black_box(day_15_1())));
    g.bench_function("day_15_2", |b| b.iter(|| black_box(day_15_2())));
    // g.bench_function("day_9_2_speed_1", |b| b.iter(|| black_box(day_8_2_speed_1())));
}


pub fn day_16(c: &mut Criterion) {
    let mut g = c.benchmark_group("day16");
    use crate::day16::*;
    g.bench_function("day_16_1", |b| b.iter(|| black_box(day_16_1())));
    g.bench_function("day_16_2", |b| b.iter(|| black_box(day_16_2())));
    g.bench_function("day_16_2_speed_1", |b| b.iter(|| black_box(day_16_2_speed_1())));
}


criterion_group!(
    benches, day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16
);
criterion_main!(benches);
