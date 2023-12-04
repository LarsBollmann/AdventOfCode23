use aoc23::input::get_input;

pub fn get_n_winning(winning: &[u32], own: &[u32]) -> usize {
    winning
        .iter()
        .filter(|winning_number| own.contains(winning_number))
        .count()
}

pub fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

pub fn parse_line(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (_, input) = input.split_once(": ").unwrap();
    let (winning_str, own_str) = input.split_once('|').unwrap();

    (parse_numbers(winning_str), parse_numbers(own_str))
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(|(winning, own)| {
            let n_wins = get_n_winning(&winning, &own);

            match n_wins {
                0 => 0,
                n => 1 << (n - 1),
            }
        })
        .sum()
}

#[derive(Debug)]
struct Card {
    count: u32,
    winning: Vec<u32>,
    own: Vec<u32>,
}

pub fn part_two(input: &str) -> u32 {
    let mut cards = input
        .lines()
        .map(parse_line)
        .map(|(winning, own)| Card {
            count: 1,
            winning,
            own,
        })
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        let n_wins = get_n_winning(&cards[i].winning, &cards[i].own);

        for j in i + 1..i + 1 + n_wins {
            cards[j].count += cards[i].count;
        }
    }

    cards.iter().map(|c| c.count).sum()
}

pub fn main() {
    let input = get_input(4);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(4);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(4);
        assert_eq!(part_two(&input), 30);
    }
}
