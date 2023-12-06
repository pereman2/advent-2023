pub fn day_6_1() -> u64 {
    let contents = include_str!("../input_6_1");
    let mut lines = contents.lines();
    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    let mut sum = 1;
    for (t ,d) in times.iter().zip(distances.iter()) {
        // (t - x) * x < d
        // -x^2 + tx -d < 0

        let d = *d as f64+0.01;
        let root = (t.pow(2) as f64 - (-4.0*-d)) as f64;
        let root = root.sqrt();
        let mut a = ((t*(-1)) as f64 + root) as f64 / -2 as f64;
        let mut b = ((t*(-1)) as f64 - root) as f64 / -2 as f64;
        if a > b {
            (a, b) = (b, a);
        }
        sum *= (b.floor() - a.ceil()) as u64 + 1;
    }
    return sum;
}



pub fn day_6_2() -> u64 {
    let contents = include_str!("../input_6_1");
    let mut lines = contents.lines();
    let times: Vec<u8> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .as_bytes()
        .iter()
        .filter(|v| (**v).is_ascii_digit())
        .map(|v| *v)
        .collect();
    let distances: Vec<u8> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .as_bytes()
        .iter()
        .filter(|v| (**v).is_ascii_digit())
        .map(|v| *v)
        .collect();
    let mut distance = 0;
    for (i, digit) in distances.iter().rev().enumerate() {
        let i = i as u32;
        distance += 10_i64.pow(i) as i64 * (*digit - b'0') as i64;
    }
    let mut time = 0;
    for (i, digit) in times.iter().rev().enumerate() {
        let i = i as u32;
        time += 10_i64.pow(i) as i64 * (*digit - b'0') as i64;
    }
    let mut sum = 1;
    let t = time;
    let d = distance as f64+0.01;
    let root = (t.pow(2) as f64 - (-4.0*-d)) as f64;
    let root = root.sqrt();
    let mut a = ((t*(-1)) as f64 + root) as f64 / -2 as f64;
    let mut b = ((t*(-1)) as f64 - root) as f64 / -2 as f64;
    if a > b {
        (a, b) = (b, a);
    }
    sum *= (b.floor() - a.ceil()) as u64 + 1;
    return sum;
}
