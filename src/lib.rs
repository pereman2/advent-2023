mod tests {
    use std::{
        fs::read_to_string,
        path::Path,
        sync::Arc,
        thread::{self, JoinHandle, ScopedJoinHandle}, collections::HashMap,
    };
    fn _read_file(path: String) -> String {
        let path = Path::new(&path);
        return read_to_string(&path).unwrap();
    }
    #[test]
    pub fn day_1_1() {
        let contents = _read_file("input_1_1".to_string());
        let mut sum = 0;
        for line in contents.split('\n') {
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
        println!("{}", sum);
    }
    #[inline(never)]
    #[test]
    pub fn day_1_2() {
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
        println!("{}", sum);
    }

    #[inline(never)]
    #[test]
    pub fn day_1_2_speed_1() {
        let contents = _read_file("input_1_1".to_string());
        let mut sum = 0;
        for line in contents.split('\n') {
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
        println!("{}", sum);
    }

    #[inline(never)]
    #[test]
    pub fn day_1_2_speed_2() {
        let contents = _read_file("input_1_1".to_string());
        let mut sum = 0;
        for line in contents.split('\n') {
            let mut first = 0;
            let mut last = 0;
            let mut i: usize = 0;
            while i < line.len() {
                let rest = &line[i..line.len()];
                // let rest = line[i..line.len()]; RETURNS A COPY WTF!
                // REMOVE BRANCHING
                first = first | (rest.starts_with("one") as u32 * 1);
                first = first | (rest.starts_with("two") as u32 * 2);
                first = first | (rest.starts_with("three") as u32 * 3);
                first = first | (rest.starts_with("four") as u32 * 4);
                first = first | (rest.starts_with("five") as u32 * 5);
                first = first | (rest.starts_with("six") as u32 * 6);
                first = first | (rest.starts_with("seven") as u32 * 7);
                first = first | (rest.starts_with("eight") as u32 * 8);
                first = first | (rest.starts_with("nine") as u32 * 9);
                let chr = line.as_bytes().get(i).unwrap(); // CHARS TOO WTF
                if chr.is_ascii_digit() {
                    last = (chr - b'0') as u32;
                }
                if first != 0 {
                    break;
                }
                i += 1;
            }
            i = line.len() - 1;
            while i >= 0 {
                let rest = &line[i..line.len()];
                last = last | (rest.starts_with("one") as u32 * 1);
                last = last | (rest.starts_with("two") as u32 * 2);
                last = last | (rest.starts_with("three") as u32 * 3);
                last = last | (rest.starts_with("four") as u32 * 4);
                last = last | (rest.starts_with("five") as u32 * 5);
                last = last | (rest.starts_with("six") as u32 * 6);
                last = last | (rest.starts_with("seven") as u32 * 7);
                last = last | (rest.starts_with("eight") as u32 * 8);
                last = last | (rest.starts_with("nine") as u32 * 9);
                let chr = line.as_bytes().get(i).unwrap();
                if chr.is_ascii_digit() {
                    last = (chr - b'0') as u32;
                }
                if last != 0 {
                    break;
                }
                i -= 1;
            }
            sum += (first * 10) + last;
        }
        println!("{}", sum);
    }

}
