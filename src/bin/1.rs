use aoc23::input::get_input;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            chars[0] * 10 + chars[chars.len() - 1]
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut replaced_input = input.to_string();
    for (i, digit) in DIGITS.iter().enumerate() {
        replaced_input = replaced_input.replace(digit, &format!("{}{}{}", digit, i + 1, digit));
    }
    part_one(&replaced_input)
}

// Second try. Works, but is too complicated.
// pub fn part_two(input: &str) -> u32 {
//     let replaced_input = input
//         .lines()
//         .map(|line| {
//             // Find all matches of digits in the line and save the index and the matched digit
//             let match_indices = DIGITS
//                 .iter()
//                 .enumerate()
//                 .map(|(i, digit)| {
//                     let indices = line
//                         .match_indices(digit)
//                         .map(|(index, _)| index)
//                         .collect::<Vec<_>>();

//                     (i + 1, indices)
//                 })
//                 .collect::<Vec<_>>();

//             let mut new_line = line.to_string();
//             // Place numeric digits in the line
//             for (digit, indices) in match_indices {
//                 for index in indices {
//                     new_line.replace_range(index..index + 1, &digit.to_string());
//                 }
//             }

//             new_line
//         })
//         .collect::<Vec<String>>()
//         .join("\n");
//     part_one(&replaced_input)
// }

// First try. Works, but is too complicated.
// pub fn part_two(input: &str) -> u32 {
//     let replaced_input = input.lines().map(|line| {
//         // Find all matches of digits in the line and save the index and the matched digit
//         let matches = DIGITS
//             .iter()
//             .enumerate()
//             .fold(Vec::new(), |mut acc, (i, d)| {
//                 if let Some(index) = line.find(d) {
//                     acc.push((index, i + 1));
//                 }
//                 if let Some(index) = line.rfind(d) {
//                     acc.push((index, i + 1));
//                 }
//                 acc
//             });

//         // Find the matches with the lowest and highest index
//         let first = matches.iter().min();
//         let last = matches.iter().max();

//         let mut new_line = line.to_string();
//         // Place numeric digits in the line
//         if let Some((i_first, n_first)) = first {
//             new_line.replace_range(i_first..i_first, &n_first.to_string());
//             if let Some((i_last, n_last)) = last {
//                 if i_last != i_first {
//                     let new_i_last = i_last+1;
//                     new_line.replace_range(new_i_last..new_i_last, &n_last.to_string());
//                 }
//             }
//         }
//         new_line

//     });
//     part_one(&replaced_input.collect::<Vec<String>>().join("\n"))
// }

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
            part_one(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
