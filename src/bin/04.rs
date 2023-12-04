advent_of_code::solution!(4);

struct Card {
    id: u16,
    w_numbers: Vec<i16>,
    p_numbers: Vec<i16>,
}

impl Card {
    fn point(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            u32::pow(2, (matches - 1) as _)
        }
    }

    fn matches(&self) -> u16 {
        let mut matches = 0;
        for number in self.p_numbers.clone() {
            if self.w_numbers.contains(&number) {
                matches += 1;
            }
        }
        matches
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let cards = parse(input);

    for card in cards {
        sum += card.point();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = vec![];

    for (row, line) in input.split("\n").into_iter().enumerate() {
        let mut w_numbers: Vec<i16> = vec![];
        let mut p_numbers: Vec<i16> = vec![];
        let mut passed_w_n = false;
        let mut passwed_id = false;

        let re = regex::Regex::new(r"\d+|\||").unwrap();

        let numbers = re.find_iter(line);

        for ch in numbers {
            if ch.as_str().is_empty() {
                continue;
            }

            if !passwed_id {
                passwed_id = true;
                continue;
            }

            if ch.as_str() == "|" {
                passed_w_n = true;
                continue;
            }
            if passed_w_n {
                p_numbers.push(ch.as_str().parse().unwrap())
            } else {
                w_numbers.push(ch.as_str().parse().unwrap());
            }
        }

        cards.push(Card {
            id: row as _,
            w_numbers: w_numbers,
            p_numbers: p_numbers,
        });
    }

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    }
}
