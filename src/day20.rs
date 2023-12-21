use std::{cell::RefCell, collections::HashMap};

use num::integer::lcm;

#[derive(PartialEq)]
enum Type {
    Inv,
    Broadcast,
    Flip,
    Dumb,
}
struct Node {
    node_type: Type,
    input: HashMap<String, bool>,
    neighbors: Vec<String>,
    state: bool,
}

pub fn day_20_1() -> u64 {
    let contents = include_str!("../input_20_1");
    let mut nodes = HashMap::new();
    for line in contents.lines() {
        let mut split = line.split(" -> ");
        let from = split.next().unwrap().as_bytes();
        let to = split.next().unwrap();
        let _type = match from[0] {
            b'b' => Type::Broadcast,
            b'%' => Type::Flip,
            b'&' => Type::Inv,
            _ => {
                panic!("Unknown type: {}", from[0] as char);
            }
        };

        let mut name: String = "broadcast".to_string();
        if _type == Type::Inv || _type == Type::Flip {
            name = String::from_utf8(from[1..].to_vec()).unwrap();
        }
        let neighbors = to
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        nodes.insert(
            name,
            Node {
                node_type: _type,
                input: HashMap::new(),
                neighbors: neighbors,
                state: false,
            },
        );
    }
    let mut sum = 0;
    unsafe {
        {
            let mut nodes = (&mut nodes) as *mut HashMap<String, Node>;
            for (name, node) in (*nodes).iter() {
                for neigh in node.neighbors.iter() {
                    if !(*nodes).contains_key(neigh) {
                        (*nodes).insert(
                            neigh.clone(),
                            Node {
                                node_type: Type::Dumb,
                                input: HashMap::new(),
                                neighbors: Vec::new(),
                                state: false,
                            },
                        );
                    }
                    let mut neigh = (*nodes).get_mut(neigh).unwrap() as *mut Node;
                    (*neigh).input.insert(name.clone(), false);
                }
            }
        }

        let mut count_low = 0;
        let mut count_high = 0;
        for i in 0..1000 {
            let mut queue = Vec::new();
            queue.push(("none", "broadcast", false));
            while queue.len() > 0 {
                let (from, name, signal) = queue.pop().unwrap();
                let low = !signal;
                let high = signal;
                count_low += low as u64;
                count_high += high as u64;
                let mut current = nodes.get_mut(name).unwrap() as *mut Node;
                match (*current).node_type {
                    Type::Inv => {
                        *(*current).input.get_mut(from).unwrap() = signal;
                        let set = (*current).input.values().all(|&x| x);
                        for neigh in &(*current).neighbors {
                            queue.insert(0, (name, neigh, !set));
                        }
                    }
                    Type::Broadcast => {
                        for neigh in &(*current).neighbors {
                            queue.insert(0, ("broadcast", neigh.as_str(), signal));
                        }
                    }
                    Type::Flip => {
                        if low {
                            let new_state = !(*current).state;
                            for neighbor in &(*current).neighbors {
                                queue.insert(0, (name, neighbor.as_str(), new_state));
                            }
                            (*current).state = new_state;
                        }
                    }
                    Type::Dumb => {}
                }
            }
        }
        sum = count_high * count_low;
    }
    return sum;
}

pub fn day_20_2() -> u64 {
    let contents = include_str!("../input_20_1");
    let mut nodes = HashMap::new();
    for line in contents.lines() {
        let mut split = line.split(" -> ");
        let from = split.next().unwrap().as_bytes();
        let to = split.next().unwrap();
        let _type = match from[0] {
            b'b' => Type::Broadcast,
            b'%' => Type::Flip,
            b'&' => Type::Inv,
            _ => {
                panic!("Unknown type: {}", from[0] as char);
            }
        };

        let mut name: String = "broadcast".to_string();
        if _type == Type::Inv || _type == Type::Flip {
            name = String::from_utf8(from[1..].to_vec()).unwrap();
        }
        let neighbors = to
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        nodes.insert(
            name,
            Node {
                node_type: _type,
                input: HashMap::new(),
                neighbors: neighbors,
                state: false,
            },
        );
    }
    let mut sum = 0;
    unsafe {
        {
            let mut nodes = (&mut nodes) as *mut HashMap<String, Node>;
            for (name, node) in (*nodes).iter() {
                for neigh in node.neighbors.iter() {
                    if !(*nodes).contains_key(neigh) {
                        (*nodes).insert(
                            neigh.clone(),
                            Node {
                                node_type: Type::Dumb,
                                input: HashMap::new(),
                                neighbors: Vec::new(),
                                state: false,
                            },
                        );
                    }
                    let mut neigh = (*nodes).get_mut(neigh).unwrap() as *mut Node;
                    (*neigh).input.insert(name.clone(), false);
                }
            }
        }

        let mut count_low = 0;
        let mut count_high = 0;
        let mut count = 0;
        let mut counts: HashMap<String, u64> = HashMap::new();
        for node in nodes.iter() {
            if node.0 == "qt" {
                for neigh in node.1.input.iter() {
                    counts.insert(neigh.0.clone(), 0);
                }
            }
        }
        'outter: for i in 0..100000000000000_u64 {
            count += 1;
            let mut queue = Vec::new();
            queue.push(("none", "broadcast", false));
            println!("\nIteration {}", i + 1);
            while queue.len() > 0 {
                if counts.iter().all(|(_, &x)| x > 0) {
                    break 'outter;
                }
                let (from, name, signal) = queue.pop().unwrap();
                // counts.get_mut(name).unwrap().push((i, signal));
                println!("{} -> {} {}", from, signal, name);
                if name == "qt" && signal {
                    let mut c = counts.get_mut(&from.to_string()).unwrap();
                    *c = i + 1 - *c;
                }
                let low = !signal;
                let high = signal;
                count_low += low as u64;
                count_high += high as u64;
                let mut current = nodes.get_mut(name).unwrap() as *mut Node;
                match (*current).node_type {
                    Type::Inv => {
                        *(*current).input.get_mut(from).unwrap() = signal;
                        let set = (*current).input.values().all(|&x| x);
                        for neigh in &(*current).neighbors {
                            queue.insert(0, (name, neigh, !set));
                        }
                    }
                    Type::Broadcast => {
                        for neigh in &(*current).neighbors {
                            queue.insert(0, ("broadcast", neigh.as_str(), signal));
                        }
                    }
                    Type::Flip => {
                        if low {
                            let new_state = !(*current).state;
                            for neighbor in &(*current).neighbors {
                                queue.insert(0, (name, neighbor.as_str(), new_state));
                            }
                            (*current).state = new_state;
                        }
                    }
                    Type::Dumb => {}
                }
            }
        }

        let l = counts.iter().fold(1, |acc, (_, &x)| lcm(acc, x));
        println!("res {} {} c {}", count_low, count_high, l);
        // for (name, vec) in counts.iter() {
        //     println!("{} {:?}", name, vec);
        // }
        sum = count_high * count_low;
    }
    return sum;
}
