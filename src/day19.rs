use std::{cmp::{min, max}, collections::HashMap};

#[derive(Debug)]
struct Rule {
    cat: u8,
    less: bool,
    n: u64,
    jump: &'static str,
}
#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

pub fn day_19_1() -> u64 {
    let contents = include_str!("../input_19_1");
    let mut chunks = contents.split("\n\n");
    let first = chunks.next().unwrap();
    let second = chunks.next().unwrap();

    let mut rules: HashMap<&str, Vec<Rule>> = HashMap::new();
    rules.insert("R", Vec::new());
    rules.insert("A", Vec::new());

    let mut machines = Vec::new();
    for line in first.lines() {
        let mut split = line.split('{');
        let name = split.next().unwrap();
        rules.insert(name, Vec::new());
        let mut workflow = rules.get_mut(name).unwrap();
        let rules = split.next().unwrap();
        let rules = rules[0..rules.len() - 1].split(',').collect::<Vec<&str>>();
        for rule in rules.iter().take(rules.len() - 1) {
            let mut rule_it = rule.split(':');
            let rule_first_part = rule_it.next().unwrap().as_bytes();
            let rule_second_part = rule_it.next().unwrap();

            let less = rule_first_part[1] == b'<';
            let rule = Rule {
                cat: rule_first_part[0],
                less: less,
                n: rule_first_part[2..rule_first_part.len()]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as u64),
                jump: rule_second_part,
            };
            workflow.push(rule);
        }
        let last = rules.last().unwrap();
        workflow.push(Rule {
            cat: b'z',
            less: false,
            n: 0,
            jump: last,
        });
    }

    for machine in second.lines() {
        let mut machine_it = machine[1..machine.len() - 1].split(',');
        let x = machine_it
            .next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let m = machine_it
            .next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let a = machine_it
            .next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let s = machine_it
            .next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let part = Part { x, m, a, s };
        machines.push(part);
    }

    for (key, workflow) in rules.iter() {
        print!("{}: ", key);
        for rule in workflow {
            print!("{:?} ", rule);
        }
        println!();
    }

    let mut sum = 0;
    for machine in machines.iter() {
        let mut current_workflow_name = "in";
        let mut current_workflow = rules.get("in").unwrap();
        let mut accepted = false;
        let mut current_workflow_index = 0;
        loop {
            if current_workflow_name == "R" {
                break;
            }
            if current_workflow_name == "A" {
                accepted = true;
                break;
            }
            let current_rule = current_workflow.get(current_workflow_index).unwrap();
            let end = current_rule.cat == b'z';
            if end {
                current_workflow_name = current_rule.jump;
                current_workflow = rules.get(current_workflow_name).unwrap();
                current_workflow_index = 0;
                continue;
            }

            match current_rule.cat {
                b'x' => {
                    if current_rule.less {
                        if machine.x < current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    } else {
                        if machine.x > current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    }
                }
                b'm' => {
                    if current_rule.less {
                        if machine.m < current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    } else {
                        if machine.m > current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    }
                }
                b'a' => {
                    if current_rule.less {
                        if machine.a < current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    } else {
                        if machine.a > current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    }
                }
                b's' => {
                    if current_rule.less {
                        if machine.s < current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    } else {
                        if machine.s > current_rule.n {
                            current_workflow_index = 0;
                            current_workflow = rules.get(current_rule.jump).unwrap();
                            current_workflow_name = current_rule.jump;
                        } else {
                            current_workflow_index += 1;
                        }
                    }
                }
                _ => panic!("Unknown cat: {}", current_rule.cat),
            }
        }
        println!("{} {:?}", accepted, machine);
        if accepted {
            sum += machine.a + machine.s + machine.x + machine.m;
        }
    }

    return sum;
}

struct State<'a> {
    state: Vec<(u64, u64)>,
    current_workflow_index: usize,
    current_workflow: &'a Vec<Rule>,
    current_workflow_name: &'a str,
}

fn solve(
    rules: &HashMap<&str, Vec<Rule>>,
    state: Vec<(u64, u64)>,
    current_workflow_index: usize,
    current_workflow: &Vec<Rule>,
    current_workflow_name: &str,
    IDX: &HashMap<u8, u64>,
) -> u64 {
    let mut stack = Vec::new();
    stack.push(State {
        state: state.clone(),
        current_workflow_index,
        current_workflow,
        current_workflow_name,
    });
    println!("{}: {}:", current_workflow_name, current_workflow_index);
    let mut sum = 0;
    let intersection = |a: (u64, u64), b: (u64, u64)| {
        let _min = max(a.0, b.0);
        let _max = min(a.1, b.1);
        if _min < _max {
            return Some((_min, _max));
        }
        return None;
    };
    while stack.len() > 0 {
        let cur_state = stack.pop().unwrap();
        let state = cur_state.state;
        let current_workflow_index = cur_state.current_workflow_index;
        let current_workflow = cur_state.current_workflow;
        let current_workflow_name = cur_state.current_workflow_name;
        for (i, x) in state.iter().enumerate() {
            print!("{}: {:?} ", i, x);
        }
        println!();

        match current_workflow_name {
            "R" => {
                continue;
            }
            "A" => {
                sum += state.iter().fold(1, |acc, x| acc * (x.1 - x.0));
                continue;
            }
            _ => {}
        }

        let end = current_workflow[current_workflow_index].cat == b'z';
        if end {
            stack.push(State {
                state: state.clone(),
                current_workflow_index: 0,
                current_workflow: rules
                    .get(current_workflow[current_workflow_index].jump)
                    .unwrap(),
                current_workflow_name: current_workflow[current_workflow_index].jump,
            });
            continue;
        }
        let current_rule = current_workflow.get(current_workflow_index).unwrap();
        let cat = *IDX.get(&current_rule.cat).unwrap() as usize;
        let mut state_false = state.clone();
        let mut state_true = state.clone();
        println!("{:?}", current_rule);
        match current_rule.less {
            true => {
                if let Some(range) = intersection(state_false[cat], (current_rule.n, 4001)) {
                    state_false[cat] = range;
                    stack.push(State {
                        state: state_false.clone(),
                        current_workflow_index: current_workflow_index + 1,
                        current_workflow: current_workflow,
                        current_workflow_name: current_workflow_name,
                    });

                }
                if let Some(range) = intersection(state_true[cat], (1, current_rule.n)) {
                    state_true[cat] = range;
                    stack.push(State {
                        state: state_true.clone(),
                        current_workflow_index: 0,
                        current_workflow: rules
                            .get(current_workflow[current_workflow_index].jump)
                            .unwrap(),
                        current_workflow_name: current_workflow[current_workflow_index].jump,
                    });

                }
            }
            false => {
                if let Some(range) = intersection(state_false[cat], (1, current_rule.n+1)) {
                    state_false[cat] = range;
                    stack.push(State {
                        state: state_false.clone(),
                        current_workflow_index: current_workflow_index + 1,
                        current_workflow: current_workflow,
                        current_workflow_name: current_workflow_name,
                    });

                }
                if let Some(range) = intersection(state_true[cat], (current_rule.n+1, 4001)) {
                    state_true[cat] = range;
                    stack.push(State {
                        state: state_true.clone(),
                        current_workflow_index: 0,
                        current_workflow: rules
                            .get(current_workflow[current_workflow_index].jump)
                            .unwrap(),
                        current_workflow_name: current_workflow[current_workflow_index].jump,
                    });

                }

            }
        }
    }
    return sum;
}

pub fn day_19_2() -> u64 {
    let contents = include_str!("../input_19_1");
    let mut chunks = contents.split("\n\n");
    let first = chunks.next().unwrap();
    let second = chunks.next().unwrap();

    let mut rules: HashMap<&str, Vec<Rule>> = HashMap::new();
    rules.insert("R", Vec::new());
    rules.insert("A", Vec::new());

    for line in first.lines() {
        let mut split = line.split('{');
        let name = split.next().unwrap();
        rules.insert(name, Vec::new());
        let mut workflow = rules.get_mut(name).unwrap();
        let rules = split.next().unwrap();
        let rules = rules[0..rules.len() - 1].split(',').collect::<Vec<&str>>();
        for rule in rules.iter().take(rules.len() - 1) {
            let mut rule_it = rule.split(':');
            let rule_first_part = rule_it.next().unwrap().as_bytes();
            let rule_second_part = rule_it.next().unwrap();

            let less = rule_first_part[1] == b'<';
            let rule = Rule {
                cat: rule_first_part[0],
                less: less,
                n: rule_first_part[2..rule_first_part.len()]
                    .iter()
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as u64),
                jump: rule_second_part,
            };
            workflow.push(rule);
        }
        let last = rules.last().unwrap();
        workflow.push(Rule {
            cat: b'z',
            less: false,
            n: 0,
            jump: last,
        });
    }

    for (key, workflow) in rules.iter() {
        print!("{}: ", key);
        for rule in workflow {
            print!("{:?} ", rule);
        }
        println!();
    }

    let mut IDX = HashMap::new();
    IDX.insert(b'x', 0);
    IDX.insert(b'm', 1);
    IDX.insert(b'a', 2);
    IDX.insert(b's', 3);
    let state = vec![(1, 4001); 4];
    let sum = solve(&rules, state, 0, rules.get("in").unwrap(), "in", &IDX);
    return sum;
}
