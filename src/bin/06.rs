advent_of_code::solution!(6);

use regex::Regex;

#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
}

pub fn part_one(input: &str) -> Option<i64> {
    let races = parse(input);
    let mut possibilities = 1;
    for race in races {
        let x1 = (race.time + f64::sqrt((race.time * race.time) - (4.0 * race.distance))) / 2.0;

        let x1 = if x1.fract() == 0.0 {
            x1 as i64 - 1
        } else {
            x1.floor() as i64
        };

        let x2 = (race.time - f64::sqrt((race.time * race.time) - (4.0 * race.distance))) / 2.0;

        let x2 = if x2.fract() == 0.0 {
            x2 as i64 + 1
        } else {
            x2.ceil() as i64
        };

        if x1 != x2 {
            possibilities *= x1 - x2 + 1;
        }
    }
    Some(possibilities)
}

pub fn part_two(input: &str) -> Option<i64> {
    let races = parse(input);
    let (time, distance) = races
        .into_iter()
        .fold((String::new(), String::new()), |acc, race| {
            (
                acc.0 + &race.time.to_string(),
                acc.1 + &race.distance.to_string(),
            )
        });
    let (time, distance) = (
        time.parse::<f64>().unwrap(),
        distance.parse::<f64>().unwrap(),
    );
    let x1 = (time + f64::sqrt((time * time) - (4.0 * distance))) / 2.0;

    let x1 = if x1.fract() == 0.0 {
        x1 as i64 - 1
    } else {
        x1.floor() as i64
    };

    let x2 = (time - f64::sqrt((time * time) - (4.0 * distance))) / 2.0;

    let x2 = if x2.fract() == 0.0 {
        x2 as i64 + 1
    } else {
        x2.ceil() as i64
    };

    Some(x1 - x2 + 1)
}

fn parse(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let number_re = Regex::new(r"\d+").unwrap();
    for (index, line) in input.split("\n").into_iter().enumerate() {
        if index % 2 == 0 {
            for number in number_re.find_iter(line) {
                races.push(Race {
                    time: number.as_str().parse().unwrap(),
                    distance: 0 as f64,
                });
            }
        } else {
            for (index, number) in number_re.find_iter(line).enumerate() {
                races[index] = Race {
                    time: races[index].time,
                    distance: number.as_str().parse().unwrap(),
                };
            }
        }
    }
    races
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(288));
    }
}
