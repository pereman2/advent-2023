pub fn day_13_1() -> u64 {
    let contents = include_str!("../input_13_1");
    let mut res = 0;
    for input in contents.split("\n\n") {
        let by_rows = input.lines().collect::<Vec<&str>>();
        let mut by_cols: Vec<Vec<u8>> = Vec::new();
        for i in 0..by_rows[0].len() {
            let mut col = Vec::new();
            for j in 0..by_rows.len() {
                col.push(by_rows[j].as_bytes()[i]);
            }
            by_cols.push(col);
        }
        let by_rows: Vec<Vec<u8>> = by_rows
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();
        let count = |kind: &Vec<Vec<u8>>| {
            let mut rows = 0;
            let mut good = false;
            'outter: for (i, pairs) in kind.windows(2).enumerate() {
                good = pairs[0].iter().zip(pairs[1].iter()).all(|(a, b)| a == b);
                let j = i + 1;
                if good {
                    let i_start = i;
                    let mut j = i + 2;
                    let mut i: i64 = i as i64 - 1;
                    while i >= 0 && j < kind.len() {
                        good = kind[i as usize]
                            .iter()
                            .zip(kind[j].iter())
                            .all(|(a, b)| a == b);
                        if !good {
                            continue 'outter;
                        }
                        i -= 1;
                        j += 1;
                    }
                    return i_start + 1;
                }
            }
            return rows;
        };
        let cols = count(&by_cols);
        let rows = count(&by_rows);
        res += cols + (rows * 100);
    }
    return res as u64;
}

pub fn day_13_2() -> u64 {
    let contents = include_str!("../input_13_1");
    let mut res = 0;
    for input in contents.split("\n\n") {
        let by_rows = input.lines().collect::<Vec<&str>>();
        let mut by_cols: Vec<Vec<u8>> = Vec::new();
        for i in 0..by_rows[0].len() {
            let mut col = Vec::new();
            for j in 0..by_rows.len() {
                col.push(by_rows[j].as_bytes()[i]);
            }
            by_cols.push(col);
        }
        let by_rows: Vec<Vec<u8>> = by_rows
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let count = |kind: &Vec<Vec<u8>>| {
            let mut rows = 0;
            let mut good = 0;
            'outter: for (i, pairs) in kind.windows(2).enumerate() {
                good = pairs[0]
                    .iter()
                    .zip(pairs[1].iter())
                    .fold(0, |acc, (a, b)| acc + (a != b) as u64);
                let j = i + 1;
                let mut chance = true;
                if good == 1 && chance {
                    chance = false;
                }
                if good <= 1 {
                    let i_start = i;
                    let mut j = i + 2;
                    let mut i: i64 = i as i64 - 1;
                    while i >= 0 && j < kind.len() {
                        good = kind[i as usize]
                            .iter()
                            .zip(kind[j].iter())
                            .fold(0, |acc, (a, b)| acc + (a != b) as u64);
                        if good == 1 && !chance {
                            continue 'outter;
                        }
                        if good == 1 && chance {
                            chance = false;
                        }
                        if good > 1 {
                            continue 'outter;
                        }
                        i -= 1;
                        j += 1;
                    }
                    if chance {
                        // we didn't fix anything
                        continue 'outter;
                    }
                    rows = i_start + 1;
                    return rows;
                }
            }
            return rows;
        };
        let cols = count(&by_cols);
        let rows = count(&by_rows);
        res += cols + (rows * 100);
    }
    return res as u64;
}
