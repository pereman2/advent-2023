use num::integer::lcm;

pub fn day_8_1() -> u64 {
    let contents = include_str!("../input_8_1");
    let mut steps = 0;
    let mut contents = contents.split("\n\n");
    let input = contents.next().unwrap();
    let nodes_str = contents.next().unwrap();
    let mut nodes: Vec<(u32, u32)> = vec![(0, 0); 16777216];
    for line in nodes_str.lines() {
        let mut line = line.split(" = ");
        let t = |a: &str| {
            let mut res: u32 = 0;
            for c in a.as_bytes() {
                res = (res << 8) | *c as u32;
            }
            return res;
        };
        let node = t(line.next().unwrap());
        let mut child = line.next().unwrap().split(", ");
        let left = t(&child.next().unwrap()[1..=3]);
        let right = t(&child.next().unwrap()[0..=2]);
        nodes[node as usize] = (left, right);
    }
    let mut current: usize = 4276545; // AAA
    const END: usize = 5921370; // ZZZ
    let mut steps = 0;
    'outter: loop {
        for action in input.as_bytes() {
            current = match action {
                b'L' => nodes[current].0 as usize,
                b'R' => nodes[current].1 as usize,
                _ => 0,
            };

            steps += 1;
            if current == END {
                break 'outter;
            }
        }
    }
    return steps;
}

pub fn day_8_2() -> u64 {
    let contents = include_str!("../input_8_2");
    let mut steps = 0;
    let mut contents = contents.split("\n\n");
    let input = contents.next().unwrap();
    let nodes_str = contents.next().unwrap();
    let mut nodes: Vec<(u32, u32)> = vec![(0, 0); 16777216];
    let mut currents: Vec<u32> = Vec::new();
    const MASK: u32 = (1 << 9) - 1;
    for line in nodes_str.lines() {
        let mut line = line.split(" = ");
        let t = |a: &str| {
            let mut res: u32 = 0;
            for c in a.as_bytes() {
                res = (res << 8) | *c as u32;
            }
            return res;
        };
        let node = t(line.next().unwrap());
        if (node & MASK) as u8 == b'A' {
            currents.push(node);
        }
        let mut child = line.next().unwrap().split(", ");
        let left = t(&child.next().unwrap()[1..=3]);
        let right = t(&child.next().unwrap()[0..=2]);
        nodes[node as usize] = (left, right);
    }
    let mut steps: u64 = 0;
    let mut steps_per_cycle: Vec<u64> = vec![0; currents.len()];
    let mut dones = currents.len();
    'outter: loop {
        for action in input.as_bytes() {
            for (i, current) in currents.iter_mut().enumerate() {
                *current = match action {
                    b'L' => nodes[*current as usize].0,
                    b'R' => nodes[*current as usize].1,
                    _ => 0,
                };

                if (*current & MASK) as u8 == b'Z' {
                    if steps_per_cycle[i] == 0 {
                        steps_per_cycle[i] = steps + 1;
                        dones -= 1;
                    }
                }
            }
            steps += 1;
            if dones == 0 {
                break 'outter;
            }
        }
    }
    let mut g = steps_per_cycle[0];
    for step in steps_per_cycle.iter().skip(1) {
        g = lcm(g, *step);
    }
    return steps;
}
