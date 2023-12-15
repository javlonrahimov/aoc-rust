advent_of_code::solution!(7);

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<char>,
    bid: u64,
    typ: u8,
}

impl Hand {
    pub fn new(cards: &str, bid: u64, use_wildcard: bool) -> Hand {
        let mut typ = 8;
        if use_wildcard {
            "AKQT98765432".chars().into_iter().for_each(|candidate| {
                let mut temp: String = String::new();
                for x in cards.chars() {
                    if x == 'J' {
                        temp.push(candidate)
                    } else {
                        temp.push(x)
                    }
                }
                typ = get_hand_type(&temp).min(typ);
            })
        }
        Hand {
            cards: cards.chars().collect(),
            bid: bid,
            typ: if use_wildcard {
                typ
            } else {
                get_hand_type(cards)
            },
        }
    }

    pub fn compare(&self, other: &Self, use_wildcard: bool) -> Ordering {
        if self.typ > other.typ {
            return Ordering::Greater;
        } else if self.typ < other.typ {
            return Ordering::Less;
        }
        for i in 0..self.cards.len() {
            let self_strength = get_card_strength(&self.cards[i], use_wildcard);
            let other_strength = get_card_strength(&other.cards[i], use_wildcard);
            if self_strength == other_strength {
                continue;
            }
            if self_strength > other_strength {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut hands = parse(input, false);
    hands.sort_by(|a, b| b.compare(a, false));
    for (index, hand) in hands.iter().enumerate() {
        result += (index as u64 + 1) * hand.bid;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut hands = parse(input, true);
    hands.sort_by(|a, b| b.compare(a, true));
    for (index, hand) in hands.iter().enumerate() {
        result += (index as u64 + 1) * hand.bid;
    }
    Some(result)
}

fn parse(input: &str, use_wildcard: bool) -> Vec<Hand> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let vector: Vec<&str> = line.split(" ").collect();
            Hand::new(vector[0], vector[1].parse().unwrap(), use_wildcard)
        })
        .collect()
}

fn get_hand_type(cards: &str) -> u8 {
    let mut cards: Vec<char> = cards.chars().collect();
    cards.sort();
    let mut min_count = cards.len();
    let mut max_count = 0;

    for card in cards.iter() {
        let mut count = 0;
        for x in cards.iter() {
            if x == card {
                count += 1;
            }
        }
        max_count = max_count.max(count);
        min_count = min_count.min(count);
    }
    cards.dedup();
    let label_count = cards.len();

    match (label_count, min_count, max_count) {
        (1, _, _) => 1,
        (2, _, 4) => 2,
        (2, 2, 3) => 3,
        (3, 1, 3) => 4,
        (3, 1, 2) => 5,
        (4, _, _) => 6,
        (5, _, _) => 7,
        _ => panic!("Invalid card"),
    }
}

fn get_card_strength(card: &char, use_wildcard: bool) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if use_wildcard {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
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
        assert_eq!(result, Some(5905));
    }
}
