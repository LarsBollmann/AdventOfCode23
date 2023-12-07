use std::collections::HashMap;

use aoc23::input::get_input;

pub fn map_char_to_value(c: char) -> u32 {
    match c {
        '1'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card value"),
    }
}

pub fn get_hand_value(hand: &str) -> usize {
    let mut counts = hand.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    // Ones are jokers
    let ones = counts.remove(&'1').unwrap_or(0);
    let mut counts: Vec<_> = counts.into_iter().map(|(_, count)| count).collect();

    counts.sort();
    counts.reverse();

    // I am so sorry for this
    let mut hand_value: usize = match counts.get(0) { // Match highest count of any card
        None => 9, // Five jokers
        Some(count) => match count {
            5 => 9, // Five of a kind, no jokers
            4 => match ones {
                // Four of a kind
                1 => 9, // becomes five of a kind with one joker
                _ => 8,
            },
            3 => match ones {
                // Three of a kind
                2 => 9, // becomes five of a kind with two jokers
                1 => 8, // becomes four of a kind with one joker
                _ => match counts.get(1).unwrap() {
                    // three of kind, check for full house
                    2 => 7, // full house
                    _ => 6, // three of a kind
                },
            },
            2 => match ones {
                // Pair
                3 => 9, // becomes five of a kind with three jokers
                2 => 8, // becomes four of a kind with two jokers
                1 => match counts.get(1).unwrap() {
                    // could be full house or three of a kind with one joker
                    2 => 7, // full house
                    _ => 6, // three of a kind
                },
                _ => match counts.get(1).unwrap() {
                    // check for two pair
                    2 => 5, // two pair
                    _ => 4, // pair
                },
            },
            1 => match ones {
                4 => 9, // becomes five of a kind with four jokers
                3 => 8, // becomes four of a kind with three jokers
                2 => match counts.get(1).unwrap() {
                    // could be full house or three of a kind with two jokers
                    2 => 7, // full house
                    _ => 6, // three of a kind
                },
                1 => match counts.get(1).unwrap() {
                    // could be two pair or pair with one joker
                    2 => 5, // two pair
                    _ => 4, // pair
                },
                _ => 3, // high card
            },
            _ => panic!("Invalid hand"),
        },
    };

    for c in hand.chars() {
        hand_value = hand_value * 15 + map_char_to_value(c) as usize;
    }

    hand_value
}

pub fn part_one(input: &str) -> u32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(" ").unwrap();
            let hand_value = get_hand_value(hand_str);
            let bid = bid_str.parse::<u32>().unwrap();
            (hand_value, bid)
        })
        .collect();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as u32 + 1) * bid)
}

pub fn part_two(input: &str) -> u32 {
    part_one(&input.replace('J', "1"))
}

pub fn main() {
    let input = get_input(7);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(7);
        assert_eq!(part_one(&input), 6440);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(7);
        assert_eq!(part_two(&input), 5905);
    }
}
