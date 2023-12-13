advent_of_code::solution!(5);

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Range {
    d_start: u64,
    s_start: u64,
    length: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse(input);
    let seeds = result.0;
    let convertor = result.1;

    let loc = seeds
        .iter()
        .map(|seed| {
            let mut result: u64 = *seed;
            for i in convertor.iter() {
                for j in i {
                    if result >= j.s_start && result < (j.s_start + j.length) {
                        result = j.d_start + (result - j.s_start);
                        break;
                    }
                }
            }
            return result;
        })
        .min()
        .unwrap();
    Some(loc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = parse(input);
    let seeds = result.0;
    let convertor = result.1;
    let mut lowest_loc = 0;
    for loc in 0u64.. {
        let mut seed: u64 = loc;
        for i in convertor.iter().rev() {
            for j in i {
                if seed >= j.d_start && seed < (j.d_start + j.length) {
                    seed = j.s_start + (seed - j.d_start);
                    break;
                }
            }
        }
        for (index, s) in seeds.iter().enumerate() {
            if index % 2 == 0 {
                if seed >= *s && seed < (s + seeds[index + 1]) {
                    lowest_loc = loc;
                    return Some(lowest_loc);
                }
            }
        }
    }
    Some(lowest_loc)
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Range>>) {
    let mut seeds: Vec<u64> = vec![];
    let mut convertor: Vec<Vec<Range>> = vec![];
    let number_re = Regex::new(r"\d+").unwrap();
    let mut range_vector: Vec<Range> = vec![];

    for (row, line) in input.split("\n").enumerate() {
        if row == 0 {
            number_re
                .find_iter(line)
                .for_each(|x| seeds.push(x.as_str().parse().unwrap()));
            continue;
        }

        if line.is_empty() {
            if !range_vector.is_empty() {
                convertor.push(range_vector.to_vec());
            }
            range_vector.clear();
            continue;
        }

        if line.chars().next().unwrap().is_alphabetic() {
            continue;
        }

        let numbers: Vec<u64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        range_vector.push(Range {
            d_start: numbers[0],
            s_start: numbers[1],
            length: numbers[2],
        });
    }

    (seeds, convertor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
