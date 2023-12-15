advent_of_code::solution!(8);

use std::cmp::{max, min};
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Node {
    value: Vec<char>,
    left: Vec<char>,
    right: Vec<char>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cycles = 0;
    let (instructions, nodes) = parse(input);
    let mut current_node = nodes.get("AAA").unwrap();
    'outer: loop {
        for ch in instructions.chars() {
            cycles += 1;
            if ch == 'L' {
                current_node = nodes
                    .get(&current_node.left.iter().collect::<String>())
                    .unwrap();
            } else {
                current_node = nodes
                    .get(&current_node.right.iter().collect::<String>())
                    .unwrap();
            }
            if current_node.value == vec!['Z', 'Z', 'Z'] {
                break 'outer;
            }
        }
    }
    Some(cycles)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cycles = 0;
    let (instructions, nodes) = parse(input);
    let mut current_nodes: Vec<Node> = nodes
        .values()
        .filter(|x| x.value[2] == 'Z')
        .map(|x| Node {
            value: x.value.clone(),
            left: x.left.clone(),
            right: x.right.clone(),
        })
        .collect();

    let mut individual_steps: HashMap<Vec<char>, u64> = HashMap::new();

    'outer: loop {
        for ch in instructions.chars() {
            cycles += 1;
            if ch == 'L' {
                current_nodes = current_nodes
                    .iter()
                    .map(|x| {
                        let data = nodes.get(&x.left.iter().collect::<String>()).unwrap();
                        if individual_steps.get(&x.value).is_some() {
                            Node {
                                value: x.value.clone(),
                                left: x.left.clone(),
                                right: x.right.clone(),
                            }
                        } else {
                            Node {
                                value: data.value.clone(),
                                left: data.left.clone(),
                                right: data.right.clone(),
                            }
                        }
                    })
                    .collect();
            } else {
                current_nodes = current_nodes
                    .iter()
                    .map(|x| {
                        let data = nodes.get(&x.right.iter().collect::<String>()).unwrap();

                        if individual_steps.get(&x.value).is_some() {
                            Node {
                                value: x.value.clone(),
                                left: x.left.clone(),
                                right: x.right.clone(),
                            }
                        } else {
                            Node {
                                value: data.value.clone(),
                                left: data.left.clone(),
                                right: data.right.clone(),
                            }
                        }
                    })
                    .collect();
            }
            current_nodes.iter().for_each(|x| {
                if x.value[2] == 'Z' {
                    individual_steps.entry(x.value.clone()).or_insert(cycles);
                }
            });
            if current_nodes.iter().all(|x| x.value[2] == 'Z') {
                break 'outer;
            }
        }
    }

    Some(individual_steps.into_values().fold(1, |mut l: usize, x| {
        l = lcm(l, x as usize);
        l
    }) as u64)
}

fn parse(input: &str) -> (String, HashMap<String, Node>) {
    let mut instructions = String::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut not_yet = false;
    for line in input.split("\n") {
        if line.is_empty() {
            not_yet = true;
            continue;
        }
        if !not_yet {
            instructions.push_str(line);
            continue;
        }

        let re = Regex::new(r"([A-Z0-9]){3}").unwrap();
        let params: Vec<&str> = re.find_iter(line).map(|x| x.as_str()).collect();

        nodes.insert(
            String::from(params[0]),
            Node {
                value: params[0].chars().collect(),
                left: params[1].chars().collect(),
                right: params[2].chars().collect(),
            },
        );
    }
    (instructions, nodes)
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
