advent_of_code::solution!(3);

use hashbrown::HashMap;

struct Coords {
    x: i16,
    y: i16,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut current_number = String::new();
    let mut number_start = Coords { x: 0, y: 0 };
    let mut number_end = Coords { x: 0, y: 0 };
    let matrix = parse(input);
    for (y, row) in matrix.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch.is_digit(10) {
                if current_number.is_empty() {
                    number_start.x = x as _;
                    number_start.y = y as _;
                }
                current_number.push(*ch);
                if x == row.len() - 1 {
                    number_end.x = (x) as _;
                    number_end.y = y as _;
                    if check_symbols(&number_start, &number_end, &matrix) {
                        sum += current_number.parse::<u32>().unwrap();
                    }
                    current_number.clear();
                }
            } else {
                if !current_number.is_empty() {
                    number_end.x = (x - 1) as _;
                    number_end.y = y as _;
                    if check_symbols(&number_start, &number_end, &matrix) {
                        sum += current_number.parse::<u32>().unwrap();
                    }
                    current_number.clear();
                }
            }
        }
        current_number.clear();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut symbols = HashMap::new();
    for (r, l) in lines.iter().enumerate() {
        let mut c = 0;
        while c < l.len() {
            let (start, mut symbol) = (c, None);
            while c < l.len() && l[c].is_ascii_digit() {
                symbol = symbol.or_else(|| find_symbol(&lines, r, c));
                c += 1;
            }
            if let Some(symbol) = symbol {
                let num = l[start..c]
                    .iter()
                    .fold(0, |n, c| n * 10 + (c - b'0') as usize);
                symbols.entry(symbol).or_insert(Vec::new()).push(num);
            }
            c += 1;
        }
    }

    Some(
        symbols
            .iter()
            .filter(|(&(_, _, s), v)| s == '*' && v.len() == 2)
            .map(|(_, v)| v[0] as u32 * v[1] as u32)
            .sum(),
    )
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.split("\n") {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        if row.is_empty() {
            continue;
        }
        matrix.push(row);
    }
    matrix
}

fn find_symbol(lines: &[&[u8]], r: usize, c: usize) -> Option<(usize, usize, char)> {
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (rr, cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
        let Some(&s) = lines.get(rr).and_then(|l| l.get(cc)) else {
            continue;
        };
        if s != b'.' && !s.is_ascii_digit() {
            return Some((cc, rr, s as char));
        }
    }
    None
}

fn check_symbols(start: &Coords, end: &Coords, matrix: &Vec<Vec<char>>) -> bool {
    for x in (start.x - 1)..=(end.x + 1) {
        for y in (start.y - 1)..=(end.y + 1) {
            if x < 0 || y < 0 || x > (matrix.len() - 1) as _ || y > (matrix[0].len() - 1) as _ {
                continue;
            }
            let ch: char = matrix[y as usize][x as usize];
            if !ch.is_digit(10) && ch != '.' {
                return true;
            }
        }
    }
    false
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
