advent_of_code::solution!(2);

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for game in parse(input) {
        let mut is_valid = true;
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            sum += game.id;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for game in parse(input) {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in game.sets {
            if min_red < set.red {
                min_red = set.red;
            }
            if min_green < set.green {
                min_green = set.green;
            }
            if min_blue < set.blue {
                min_blue = set.blue;
            }
        }
        sum += min_red * min_green * min_blue;
    }

    Some(sum)
}

fn parse(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.split("\n") {
        let mut id = 0;
        let mut set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };
        let mut sets: Vec<Set> = vec![];
        let mut current_number = String::new();

        for i in line.chars() {
            if i.is_digit(10) {
                current_number.push(i);
            }
            if i == ':' {
                id = current_number.parse().unwrap();
                current_number.clear();
            }
            if i == ';' {
                sets.push(set);
                set = Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
            }
            if i.is_alphabetic() && !current_number.is_empty() {
                match i {
                    'r' => {
                        set.red = current_number.parse().unwrap();
                        current_number.clear();
                    }
                    'g' => {
                        set.green = current_number.parse().unwrap();
                        current_number.clear();
                    }
                    'b' => {
                        set.blue = current_number.parse().unwrap();
                        current_number.clear();
                    }
                    _ => current_number.clear(),
                }
            }
        }
        sets.push(set);
        let game = Game { id, sets };
        games.push(game);
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
