use crate::shared::{manhattan_distance, Point};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> std::io::Result<i32> {
    return common(1);
}

pub fn part2() -> std::io::Result<i32> {
    return common(2);
}

fn common(part: i32) -> std::io::Result<i32> {
    let file = File::open("input/d3.txt")?;
    let reader = BufReader::new(file);
    let mut cache: HashMap<String, i32> = HashMap::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        let instructions: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        let closest = walk(&instructions, &mut cache, &(index as i32), part);
        if index == 1 {
            return Ok(closest);
        }
    }
    panic!("wtf")
}

fn walk(
    instructions: &Vec<String>,
    cache: &mut HashMap<String, i32>,
    pass: &i32,
    part: i32,
) -> i32 {
    let mut last = Point { x: 0, y: 0 };
    let mut minimum: i32 = i32::max_value();
    let mut step_count = 0;
    for instruction in instructions {
        let (dir, seg_len_str) = instruction.split_at(1);
        let seg_len = seg_len_str.parse::<i32>().unwrap();
        let step: (i32, i32);
        match dir {
            "U" => {
                step = (0, -1);
            }
            "D" => {
                step = (0, 1);
            }
            "L" => {
                step = (-1, 0);
            }
            "R" => {
                step = (1, 0);
            }
            &_ => panic!("wtf"),
        }
        for _ in 1..=seg_len {
            last = Point {
                x: last.x + step.0,
                y: last.y + step.1,
            };
            step_count += 1;
            if *pass == 0 {
                cache.insert(last.to_string(), step_count);
            } else {
                match cache.get(&last.to_string()) {
                    Some(cached_step_count) => {
                        if part == 1 {
                            let dist = manhattan_distance(&Point::origin(), &last);
                            if dist < minimum {
                                minimum = dist;
                            }
                        } else if part == 2 {
                            let sum = cached_step_count + step_count;
                            if sum < minimum {
                                minimum = sum;
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }
    return minimum;
}
