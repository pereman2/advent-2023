mod day1;
mod day2;

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
    fn _read_file(path: String) -> String {
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
        let contents = _read_file("input_1_1".to_string());
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
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_1(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_1_1", |b| b.iter(|| black_box(advent::day_1_2())));
    g.bench_function("day_1_2", |b| b.iter(|| black_box(advent::day_1_2())));
    g.bench_function("day_1_2_speed_1", |b| b.iter(|| black_box(advent::day_1_2_speed_1())));
    g.bench_function("day_1_2_speed_2", |b| b.iter(|| black_box(advent::day_1_2_speed_2())));
}

pub fn day_2(c: &mut Criterion) {
    let mut g = c.benchmark_group("day2");
    g.bench_function("day_2_1", |b| b.iter(|| black_box(advent::day_2_2())));
    g.bench_function("day_2_2", |b| b.iter(|| black_box(advent::day_2_2())));
    g.bench_function("day_2_2_speed_1", |b| b.iter(|| black_box(advent::day_2_2_speed_1())));
}

criterion_group!(benches, day_1, day_2);
criterion_main!(benches);
