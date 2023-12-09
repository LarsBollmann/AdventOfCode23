use aoc23::input::get_input;

trait IntoDiff: IntoIterator
where
    Self::Item: std::ops::Sub<Output = Self::Item>,
{
    fn diff(self) -> Diff<Self::IntoIter>;
}

impl<T> IntoDiff for T
where
    T: IntoIterator,
    T::Item: std::ops::Sub<Output = T::Item>,
{
    fn diff(self) -> Diff<<Self as IntoIterator>::IntoIter> {
        Diff {
            iter: self.into_iter(),
            last: None,
        }
    }
}

struct Diff<I>
where
    I: Iterator,
    I::Item: std::ops::Sub<Output = I::Item>,
{
    iter: I,
    last: Option<<I as Iterator>::Item>,
}

impl<I> Iterator for Diff<I>
where
    I: Iterator,
    I::Item: std::ops::Sub<Output = I::Item> + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        let last = self.last.replace(next.clone());

        match last {
            Some(last) => Some(next - last),
            None => self.next(),
        }
    }
}

pub fn get_next_number(numbers: Vec<i32>) -> i32 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }

    let last = *numbers.last().unwrap();

    last + get_next_number(numbers.diff().collect())
}

pub fn get_previous_number(numbers: Vec<i32>) -> i32 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }

    let first = *numbers.first().unwrap();

    first - get_previous_number(numbers.diff().collect())
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect()
    })
}

pub fn part_one(input: &str) -> i32 {
    parse_input(input).map(get_next_number).sum()
}

pub fn part_two(input: &str) -> i32 {
    parse_input(input).map(get_previous_number).sum()
}

pub fn main() {
    let input = get_input(9);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(9);
        assert_eq!(part_one(&input), 114);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(9);
        assert_eq!(part_two(&input), 2);
    }
}
