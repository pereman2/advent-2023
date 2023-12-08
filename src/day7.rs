use std::{cmp::max, collections::HashMap};

pub fn day_7_1() -> u64 {
    let mut seen: Vec<u8> = vec![0; 255];
    let mut t: Vec<Vec<(&str, u64, u32)>> = vec![Vec::new(); 7];
    let contents = include_str!("../input_7_1");
    let mut values: HashMap<u8, u8> = HashMap::new();
    values.insert('A' as u8, 18); 
    values.insert('K' as u8, 17); 
    values.insert('Q' as u8, 16); 
    values.insert('J' as u8, 15); 
    values.insert('T' as u8, 14); 
    values.insert('9' as u8, 13); 
    values.insert('8' as u8, 12); 
    values.insert('7' as u8, 10); 
    values.insert('6' as u8, 9); 
    values.insert('5' as u8, 8); 
    values.insert('4' as u8, 7); 
    values.insert('3' as u8, 6); 
    values.insert('2' as u8, 5); 

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let hand = line.next().unwrap();
        let bid = line.next().unwrap().parse::<u64>().unwrap();
        let mut pair = false;
        let mut double_pair = false;
        let mut three = false;
        let mut house = false;
        let mut four = false;
        let mut five = false;
        let mut high = 0_u32;
        seen = vec![0; 255];
        for c in hand.as_bytes() {
            high = max(high, *values.get(c).unwrap() as u32);
            seen[*c as usize] += 1;
            if seen[*c as usize] == 2 {
                if pair {
                    double_pair = true;
                }
                pair = true;
            } else if seen[*c as usize] == 3 {
                if double_pair {
                    double_pair = false;
                } else {
                    pair = false;
                }
                three = true;
            } else if seen[*c as usize] == 4 {
                four = true;
                three = false;
            } else if seen[*c as usize] == 5 {
                five = true;
            }
            if three && pair {
                house = true;
            }
        }
        if five {
            t[6].push((hand, bid, high));
        } else if four {
            t[5].push((hand, bid, high));
        } else if house {
            t[4].push((hand, bid, high));
        } else if three {
            t[3].push((hand, bid, high));
        } else if double_pair {
            t[2].push((hand, bid, high));
        } else if pair {
            t[1].push((hand, bid, high));
        } else {
            t[0].push((hand, bid, high));
        }
    }

    let mut rank = 1;
    let mut res: u64 = 0;
    for (i, kind) in t.iter_mut().enumerate() {
        kind.sort_by(|a, b| {
            for i in 0..5 {
                let aa = values.get(&a.0.as_bytes()[i as usize]).unwrap();
                let bb = values.get(&b.0.as_bytes()[i as usize]).unwrap();
                match aa.cmp(&bb) {
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {}
                }
            }
            return std::cmp::Ordering::Greater;
        });

        for hand in kind {
            res += (rank * hand.1);
            rank += 1;
        }
    }
    return res;
}

pub fn day_7_2() -> u64 {
    let mut seen: Vec<u8> = vec![0; 255];
    let mut t: Vec<Vec<(&str, u64, u32)>> = vec![Vec::new(); 7];
    let contents = include_str!("../input_7_1");
    let mut values: HashMap<u8, u8> = HashMap::new();
    values.insert('A' as u8, 18); 
    values.insert('K' as u8, 17); 
    values.insert('Q' as u8, 16); 
    values.insert('T' as u8, 14); 
    values.insert('9' as u8, 13); 
    values.insert('8' as u8, 12); 
    values.insert('7' as u8, 10); 
    values.insert('6' as u8, 9); 
    values.insert('5' as u8, 8); 
    values.insert('4' as u8, 7); 
    values.insert('3' as u8, 6); 
    values.insert('2' as u8, 5); 
    values.insert('J' as u8, 1); 

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let hand = line.next().unwrap();
        let bid = line.next().unwrap().parse::<u64>().unwrap();
        let mut pair = false;
        let mut double_pair = false;
        let mut three = false;
        let mut house = false;
        let mut four = false;
        let mut five = false;
        let mut high = 0_u32;
        seen = vec![0; 255];
        for c in hand.as_bytes() {
            high = max(high, *values.get(c).unwrap() as u32);
            seen[*c as usize] += 1;
            if *c == 'J' as u8 {
                continue;
            }
            if seen[*c as usize] == 2 {
                if pair {
                    double_pair = true;
                    pair = false;
                } else {
                    pair = true;
                }
            } else if seen[*c as usize] == 3 {
                if double_pair {
                    double_pair = false;
                    pair = true;
                } else {
                    pair = false;
                }
                three = true;
            } else if seen[*c as usize] == 4 {
                four = true;
                three = false;
            } else if seen[*c as usize] == 5 {
                five = true;
                four = false;
            }
        }
        if three && pair {
            house = true;
            three = false;
            pair = false;
        }
        let mut w = seen['J' as usize];
        while w > 0 {
            if pair {
                three = true;
                pair = false;
            } else if double_pair {
                house = true;
            } else if three {
                four = true;
                three = false;
            } else if house {
                four = true;
                three = false;
            } else if four {
                five = true;
            } else {
                match w {
                    1 => pair = true,
                    2 => three = true,
                    3 => four = true,
                    4 => five = true,
                    5 => five = true,
                    _ => {}
                }
                break;
            }
            w -= 1;
        }
        if five {
            t[6].push((hand, bid, high));
        } else if four {
            t[5].push((hand, bid, high));
        } else if house {
            t[4].push((hand, bid, high));
        } else if three {
            t[3].push((hand, bid, high));
        } else if double_pair {
            t[2].push((hand, bid, high));
        } else if pair {
            t[1].push((hand, bid, high));
        } else {
            t[0].push((hand, bid, high));
        }
    }

    let mut rank = 1;
    let mut res: u64 = 0;
    for (i, kind) in t.iter_mut().enumerate() {
        kind.sort_by(|a, b| {
            for i in 0..5 {
                let aa = values.get(&a.0.as_bytes()[i as usize]).unwrap();
                let bb = values.get(&b.0.as_bytes()[i as usize]).unwrap();
                match aa.cmp(&bb) {
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {}
                }
            }
            return std::cmp::Ordering::Greater;
        });

        for hand in kind {
            res += (rank * hand.1);
            rank += 1;
        }
    }
    return res;
}