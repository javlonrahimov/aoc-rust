advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for line in input.split("\n") {
        let mut first = 0;
        let mut last = 0;
        for i in line.chars() {
            if i.is_digit(10) {
                first = i.to_digit(10).unwrap();
                break;
            }
        }
        for i in line.chars().rev() {
            if i.is_digit(10) {
                last = i.to_digit(10).unwrap();
                break;
            }
        }
        result += 10 * first + last;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let er = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let first = re.find(line).unwrap().as_str();
        let reversed = line.chars().rev().collect::<String>();
        let last = er.find(reversed.as_str()).unwrap().as_str();
        result += 10 * get_digit(first) + get_digit(last);
    }
    Some(result)
}

fn get_digit(input: &str) -> u32 {
    match input {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => input.parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_get_digit() {
        assert!(5 == get_digit("5"))
    }
}
