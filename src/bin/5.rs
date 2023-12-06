use std::iter::Map;

use aoc23::input::get_input;

pub fn apply_map(values: &[u32], map: &str) -> Vec<u32> {
    let maps: Vec<Vec<u32>> = map.lines().skip(1).map(|line| line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>()).collect();

    values.into_iter().map(move |value| {
        for map in &maps {
            let start_dest = map[0];
            let start_src = map[1];
            let length = map[2];

            if *value >= start_src && *value < start_src + length {
                let dest = start_dest + (*value - start_src);
                return dest;
            }
        }
        return *value;
    }).collect()
}


pub fn part_one(input: &str) -> u32 {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let mut values = seeds.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<_>>();


    for map in maps.split("\n\n") {
        values = apply_map(&values, map);
    }

    *values.iter().min().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seed_ranges: Vec<u32> = seeds.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let mut values: Vec<u32> = Vec::new();
    
    for range in seed_ranges.chunks(2) {
        values.extend(range[0]..range[0] + range[1]);
    }

    for map in maps.split("\n\n") {
        values = apply_map(&values, map);
    }

    *values.iter().min().unwrap()
}

pub fn main() {
    let input = get_input(5);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(5);
        assert_eq!(part_one(&input), 35);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(5);
        assert_eq!(part_two(&input), 46);
    }
}
