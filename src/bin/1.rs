use aoc23::input::get_input;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            chars[0] * 10
                + chars[chars.len() - 1]
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    1
}

pub fn main() {
    let input = get_input(1);
    println!("Part one answer: {}", part_one(&input));
    println!("Part two answer: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
        part_one("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"), 281);
    }
}
