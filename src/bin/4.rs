use aoc23::input::get_input;

pub fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

pub fn parse_line(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (_, input) = input.split_once(": ").unwrap();
    let (winning, own) = input.split_once('|').unwrap();
    let winning = parse_numbers(winning);
    let own = parse_numbers(own);
    (winning, own)
}

pub fn part_one(input: &str) -> u32 {
    input.lines().map(parse_line).map(|(winning, own)| {
        let n_wins = winning.iter().filter(|n| own.contains(n)).count();
        println!("{:?} {:?} {}", winning, own, n_wins);
       match n_wins {
            0 => 0,
            n => 1 << (n - 1),
       }
    }).sum()
}

pub fn part_two(input: &str) -> u32 {

    1
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
        assert_eq!(part_two(&input), 467835);
    }
}
