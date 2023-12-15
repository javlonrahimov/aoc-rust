advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let history = parse(input);

    Some(
        history
            .into_iter()
            .map(|mut h| {
                let mut result = h[h.len() - 1];
                loop {
                    h = diff(h);
                    if h.iter().all(|x| *x == 0) {
                        break;
                    }
                    result += h[h.len() - 1];
                }
                result
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let history = parse(input);

    Some(
        history
            .into_iter()
            .map(|mut h| {
                h = h.into_iter().rev().collect();
                let mut result = h[h.len() - 1];
                loop {
                    h = diff(h);
                    if h.iter().all(|x| *x == 0) {
                        break;
                    }
                    result += h[h.len() - 1];
                }
                result
            })
            .sum(),
    )
}

fn diff(input: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut index = 0;
    loop {
        if index >= input.len() - 1 {
            break;
        }
        result.push(input[index + 1] - input[index]);
        index += 1;
    }
    result
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut history: Vec<Vec<i32>> = vec![];
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        history.push(line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
    }
    history
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
