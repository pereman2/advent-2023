use std::{cmp::min, collections::HashMap};

fn solve(i: usize, cur_num: usize, extent: &[u8], nums: &Vec<u64>, dp: &mut HashMap<(usize, usize), u64>) -> u64 {
    if dp.contains_key(&(i, cur_num)) {
        return dp[&(i, cur_num)];
    }
    if cur_num >= nums.len() {
        // if some # were left, it is invalid
        let mut i: usize = i;
        while i < extent.len() {
            if extent[i] == b'#' {
                return 0;
            }
            i += 1;
        }
        return 1;
    }

    if i >= extent.len() {
        return 0;
    }
    if extent[i] == b'.' {
        return solve(i + 1, cur_num, extent, nums, dp);
    }
    let mut counti = 0;
    let mut counth = 0;
    if extent.len() - i < nums[cur_num] as usize {
        return 0;
    }
    for j in i..i + nums[cur_num] as usize {
        if extent[j] == b'?' {
            counti += 1;
        }
        if extent[j] == b'#' {
            counth += 1;
        }
        if extent[i] == b'.' {
            break;
        }
    }
    let total = counth as usize + counti as usize;
    if counti + counth < nums[cur_num] {
        if extent[i] == b'#' {
            // we cannot just miss this #
            return 0;
        }
        return solve(i + 1, cur_num, extent, nums, dp);
    }
    // should be equal
    let end: usize = i + total;
    let mut sol = 0;
    assert!(total == nums[cur_num] as usize);
    if end < extent.len() {
        if extent[end] == b'#' {
            if i < extent.len() && extent[i] == b'#' {
                // we cannot just miss this #
            } else {
                sol = solve(i + 1, cur_num, extent, nums, dp);
            }
        } else {
            sol += solve(end + 1, cur_num + 1, extent, nums, dp);
            if i < extent.len() && extent[i] == b'#' {
                // we cannot just miss this #
            } else {
                // okay we can consider skipping this
                sol += solve(i + 1, cur_num, extent, nums, dp);

            }
        }
    } else {
        sol = solve(end, cur_num + 1, extent, nums, dp);
    }
    // 5454 too low
    // 7515 too high
    dp.insert((i, cur_num), sol);
    return sol;
}

pub fn day_12_1() -> u64 {
    let contents = include_str!("../input_12_1");
    let mut res = 0;
    for line in contents.lines() {
        let mut l = line.split(" ");
        let extent = l.next().unwrap().as_bytes();
        let nums = l
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut dp: HashMap<(usize, usize), u64> = HashMap::new();
        let sol = solve(0, 0, extent, &nums, &mut dp);
        res += sol;
    }
    return res;
}

pub fn day_12_2() -> u64 {
    let contents = include_str!("../input_12_1");
    let mut res = 0;
    for line in contents.lines() {
        let mut l = line.split(" ");
        let extent = l.next().unwrap().as_bytes().to_vec();
        let nums = l
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut mult_extents: Vec<u8> = Vec::new();
        mult_extents.extend_from_slice(&extent);
        mult_extents.push(b'?');
        mult_extents.extend_from_slice(&extent);
        mult_extents.push(b'?');
        mult_extents.extend_from_slice(&extent);
        mult_extents.push(b'?');
        mult_extents.extend_from_slice(&extent);
        mult_extents.push(b'?');
        mult_extents.extend_from_slice(&extent);
        let mut mult_nums = Vec::new();
        mult_nums.extend_from_slice(&nums);
        mult_nums.extend_from_slice(&nums);
        mult_nums.extend_from_slice(&nums);
        mult_nums.extend_from_slice(&nums);
        mult_nums.extend_from_slice(&nums);
        let mut dp: HashMap<(usize, usize), u64> = HashMap::new();
        let sol = solve(0, 0, &mult_extents, &mult_nums, &mut dp);
        res += sol;
    }
    return res;
}
