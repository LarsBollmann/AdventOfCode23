use aoc23::input;

pub fn hash_string(input: &str) -> u32 {
    input.chars().fold(0, |acc, c| {
        ((acc + c as u32) * 17) % 256
    })
}

pub fn part_one(input: &str) -> u32 {
    input.split(',').map(hash_string).sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut hashmap: [Vec<(String, u32)>; 256] = std::array::from_fn(|_| Vec::new());

    for instruction in input.split(',') {
        let n = instruction.find(|c| c == '=' || c == '-').unwrap();
        let (label, rest) = instruction.split_at(n);
        let bucket = &mut hashmap[hash_string(label) as usize];

        match rest.split_at(1) {
            ("=", focal_length) => {
                let focal_length = focal_length.parse().unwrap();
                match bucket.iter_mut().find(|(l, _)| l == label) {
                    Some((_, f)) => *f = focal_length,
                    None => bucket.push((label.to_string(), focal_length)),
                }
            }
            ("-", _) => {
                if bucket.iter_mut().any(|(l, _f)| l == label) {
                    bucket.retain(|(l, _f)| l != label);
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }

    hashmap.into_iter().enumerate().fold(0, |acc, (n_b, b)| {
        acc + b
            .into_iter()
            .enumerate()
            .fold(0, |acc, (n_a, (_, focal_length))| {
                acc + (n_b as u32 + 1) * (n_a as u32 + 1) * focal_length
            })
    })
}

pub fn main() {
    let input = input::get_input(15);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(15);
        assert_eq!(part_one(&input), 1320);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(15);
        assert_eq!(part_two(&input), 145);
    }
}
