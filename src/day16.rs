use std::cmp::max;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Beam {
    pos: (i64, i64),
    dx: (i64, i64),
}

pub fn day_16_1() -> u64 {
    let contents = include_str!("../input_16_1");
    let grid = contents
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut beams: Vec<Beam> = Vec::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    beams.push(Beam {
        pos: (0, -1),
        dx: (0, 1),
    });

    while beams.len() > 0 {
        let beam = beams.pop().unwrap();
        if seen.contains(&beam) {
            continue;
        }
        seen.insert(beam.clone());
        let new_x = beam.pos.0 + beam.dx.0;
        let new_y = beam.pos.1 + beam.dx.1;
        if new_x >= grid.len() as i64
            || new_x < 0 as i64
            || new_y >= grid[0].len() as i64
            || new_y < 0 as i64
        {
            continue;
        }

        match grid[new_x as usize][new_y as usize] {
            b'.' => {
                beams.push(Beam {
                    pos: (new_x, new_y),
                    dx: beam.dx,
                });
            }
            b'/' => match beam.dx {
                (0, 1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (-1, 0),
                    });
                }
                (0, -1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (1, 0),
                    });
                }
                (1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, -1),
                    });
                }
                (-1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, 1),
                    });
                }
                _ => panic!("Unknown dx: {:?}", beam.dx),
            },
            b'\\' => match beam.dx {
                (0, 1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (1, 0),
                    });
                }
                (0, -1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (-1, 0),
                    });
                }
                (1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, 1),
                    });
                }
                (-1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, -1),
                    });
                }
                _ => panic!("Unknown dx: {:?}", beam.dx),
            },
            b'|' => match beam.dx {
                (0, 1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (1, 0),
                    });
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (-1, 0),
                    });
                }
                (0, -1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (1, 0),
                    });
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (-1, 0),
                    });
                }
                (1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: beam.dx,
                    });
                }
                (-1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: beam.dx,
                    });
                }
                _ => panic!("Unknown dx: {:?}", beam.dx),
            },
            b'-' => match beam.dx {
                (0, 1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: beam.dx,
                    });
                }
                (0, -1) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: beam.dx,
                    });
                }
                (1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, 1),
                    });
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, -1),
                    });
                }
                (-1, 0) => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, 1),
                    });
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: (0, -1),
                    });
                }
                _ => panic!("Unknown dx: {:?}", beam.dx),
            },
            _ => panic!("Unknown char: {}", grid[new_x as usize][new_y as usize]),
        }
    }

    let mut unique = HashSet::new();
    for v in seen {
        unique.insert(v.pos);
    }
    return unique.len() as u64 - 1; // include -1
}

pub fn day_16_2() -> u64 {
    let contents = include_str!("../input_16_1");
    let grid = contents
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut start_positions = Vec::new();
    let mut _max = 0;
    let n = grid.len();
    let m = grid.len();
    for i in 0..n {
        start_positions.push(Beam {
            pos: (i as i64, -1),
            dx: (0, 1),
        });
        start_positions.push(Beam {
            pos: (i as i64, m as i64),
            dx: (0, -1),
        });
    }
    for i in 0..m {
        start_positions.push(Beam {
            pos: (-1, i as i64),
            dx: (1, 0),
        });
        start_positions.push(Beam {
            pos: (n as i64, i as i64),
            dx: (-1, 0),
        });
    }
    for beam_start in start_positions {
        let mut beams: Vec<Beam> = Vec::new();
        let mut seen: HashSet<Beam> = HashSet::new();

        beams.push(beam_start);

        while beams.len() > 0 {
            let beam = beams.pop().unwrap();
            if seen.contains(&beam) {
                continue;
            }
            seen.insert(beam.clone());
            let new_x = beam.pos.0 + beam.dx.0;
            let new_y = beam.pos.1 + beam.dx.1;
            if new_x >= grid.len() as i64
                || new_x < 0 as i64
                || new_y >= grid[0].len() as i64
                || new_y < 0 as i64
            {
                continue;
            }

            match grid[new_x as usize][new_y as usize] {
                b'.' => {
                    beams.push(Beam {
                        pos: (new_x, new_y),
                        dx: beam.dx,
                    });
                }
                b'/' => match beam.dx {
                    (0, 1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (-1, 0),
                        });
                    }
                    (0, -1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (1, 0),
                        });
                    }
                    (1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, -1),
                        });
                    }
                    (-1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, 1),
                        });
                    }
                    _ => panic!("Unknown dx: {:?}", beam.dx),
                },
                b'\\' => match beam.dx {
                    (0, 1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (1, 0),
                        });
                    }
                    (0, -1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (-1, 0),
                        });
                    }
                    (1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, 1),
                        });
                    }
                    (-1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, -1),
                        });
                    }
                    _ => panic!("Unknown dx: {:?}", beam.dx),
                },
                b'|' => match beam.dx {
                    (0, 1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (1, 0),
                        });
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (-1, 0),
                        });
                    }
                    (0, -1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (1, 0),
                        });
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (-1, 0),
                        });
                    }
                    (1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: beam.dx,
                        });
                    }
                    (-1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: beam.dx,
                        });
                    }
                    _ => panic!("Unknown dx: {:?}", beam.dx),
                },
                b'-' => match beam.dx {
                    (0, 1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: beam.dx,
                        });
                    }
                    (0, -1) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: beam.dx,
                        });
                    }
                    (1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, 1),
                        });
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, -1),
                        });
                    }
                    (-1, 0) => {
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, 1),
                        });
                        beams.push(Beam {
                            pos: (new_x, new_y),
                            dx: (0, -1),
                        });
                    }
                    _ => panic!("Unknown dx: {:?}", beam.dx),
                },
                _ => panic!("Unknown char: {}", grid[new_x as usize][new_y as usize]),
            }
        }

        let mut unique = HashSet::new();
        for v in seen {
            unique.insert(v.pos);
        }
        _max = max(unique.len() as u64 - 1, _max); // include -1
    }
    return _max;
}

pub fn day_16_2_speed_1() -> u64 {
    let contents = include_str!("../input_16_1");
    let grid = contents
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut start_positions = Vec::new();
    let mut _max: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let n = grid.len();
    let m = grid.len();
    for i in 0..n {
        start_positions.push(Beam {
            pos: (i as i64, -1),
            dx: (0, 1),
        });
        start_positions.push(Beam {
            pos: (i as i64, m as i64),
            dx: (0, -1),
        });
    }
    for i in 0..m {
        start_positions.push(Beam {
            pos: (-1, i as i64),
            dx: (1, 0),
        });
        start_positions.push(Beam {
            pos: (n as i64, i as i64),
            dx: (-1, 0),
        });
    }
    std::thread::scope(|s| {
        for beam_start in start_positions {
            s.spawn(|| {
                let mut beams: Vec<Beam> = Vec::new();
                let mut seen: HashSet<Beam> = HashSet::new();

                beams.push(beam_start);

                while beams.len() > 0 {
                    let beam = beams.pop().unwrap();
                    if seen.contains(&beam) {
                        continue;
                    }
                    seen.insert(beam.clone());
                    let new_x = beam.pos.0 + beam.dx.0;
                    let new_y = beam.pos.1 + beam.dx.1;
                    if new_x >= grid.len() as i64
                        || new_x < 0 as i64
                        || new_y >= grid[0].len() as i64
                        || new_y < 0 as i64
                    {
                        continue;
                    }

                    match grid[new_x as usize][new_y as usize] {
                        b'.' => {
                            beams.push(Beam {
                                pos: (new_x, new_y),
                                dx: beam.dx,
                            });
                        }
                        b'/' => match beam.dx {
                            (0, 1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (-1, 0),
                                });
                            }
                            (0, -1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (1, 0),
                                });
                            }
                            (1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, -1),
                                });
                            }
                            (-1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, 1),
                                });
                            }
                            _ => panic!("Unknown dx: {:?}", beam.dx),
                        },
                        b'\\' => match beam.dx {
                            (0, 1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (1, 0),
                                });
                            }
                            (0, -1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (-1, 0),
                                });
                            }
                            (1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, 1),
                                });
                            }
                            (-1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, -1),
                                });
                            }
                            _ => panic!("Unknown dx: {:?}", beam.dx),
                        },
                        b'|' => match beam.dx {
                            (0, 1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (1, 0),
                                });
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (-1, 0),
                                });
                            }
                            (0, -1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (1, 0),
                                });
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (-1, 0),
                                });
                            }
                            (1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: beam.dx,
                                });
                            }
                            (-1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: beam.dx,
                                });
                            }
                            _ => panic!("Unknown dx: {:?}", beam.dx),
                        },
                        b'-' => match beam.dx {
                            (0, 1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: beam.dx,
                                });
                            }
                            (0, -1) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: beam.dx,
                                });
                            }
                            (1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, 1),
                                });
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, -1),
                                });
                            }
                            (-1, 0) => {
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, 1),
                                });
                                beams.push(Beam {
                                    pos: (new_x, new_y),
                                    dx: (0, -1),
                                });
                            }
                            _ => panic!("Unknown dx: {:?}", beam.dx),
                        },
                        _ => panic!("Unknown char: {}", grid[new_x as usize][new_y as usize]),
                    }
                }

                let mut unique = HashSet::new();
                for v in seen {
                    unique.insert(v.pos);
                }
                let mut m = _max.lock().unwrap();
                *m = max(unique.len() as u64 - 1, *m); // include -1
            });
        }
    });
    return *_max.lock().unwrap();
}
