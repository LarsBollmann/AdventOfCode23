use aoc23::input;

pub fn part_one(input_string: String) -> u64 {
    1
}

pub fn part_two(input_string: String) -> u64 {
    1
}

pub fn main() {
    let input = input::get_input(1);
    println!("Part one answer: {}", part_one(input.clone()));
    println!("Part two answer: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = input::get_example(1);
        assert_eq!(part_one(input), 1);
    }

    #[test]
    fn test_part_two() {
        let input = input::get_example(1);
        assert_eq!(part_two(input), 1);
    }
}