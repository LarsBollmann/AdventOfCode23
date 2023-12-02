use aoc23::input::get_input;

pub fn solution(input: &str) -> (u32, u32) {
    let mut possible_sum = 0;
    let mut power_sum = 0;

    for game in input.lines() {
        let (id, sets) = game.split_once(':').unwrap();
        let mut minimum: [u32; 3] = [0; 3];

        for set in sets.split(';') {
            for group in set.split(',') {
                let (n, color) = group.trim().split_once(' ').unwrap();
                let n = n.parse::<u32>().unwrap();
                match color {
                    "red" => minimum[0] = n.max(minimum[0]),
                    "green" => minimum[1] = n.max(minimum[1]),
                    "blue" => minimum[2] = n.max(minimum[2]),
                    _ => panic!("Invalid color"),
                };
            }
        }
        
        if minimum[0] <= 12 && minimum[1] <= 13 && minimum[2] <= 14 {
            possible_sum += id
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
        power_sum += minimum[0] * minimum[1] * minimum[2];
    }
    (possible_sum, power_sum)
}

pub fn main() {
    let input = get_input(2);
    println!("Part one and two: {:?}", solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(2);
        assert_eq!(solution(&input).0, 8);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(2);
        assert_eq!(solution(&input).1, 2286);
    }
}
