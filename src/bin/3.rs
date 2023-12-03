use std::collections::HashSet;

use aoc23::input::get_input;

fn get_number_origin_and_value(grid: &[Vec<char>], x: usize, y: usize) -> (usize, usize, u32) {
    let mut y = y;
    let mut number = 0;

    while y > 0 && grid[x][y - 1].is_ascii_digit() {
        y -= 1;
    }

    let y_start = y;
    while y < grid[x].len() && grid[x][y].is_ascii_digit() {
        number += number * 10 + grid[x][y].to_digit(10).unwrap();
        y += 1;
    }
    (x, y_start, number)
}

fn get_adjacent_numbers(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize, u32)> {
    let mut adjacent = HashSet::new();

    for i in x.saturating_sub(1)..=x + 1 {
        for j in y.saturating_sub(1)..=y + 1 {
            if i == x && j == y {
                continue;
            }
            if i < grid.len() && j < grid[i].len() && grid[i][j].is_ascii_digit() {
                let (x, y, n) = get_number_origin_and_value(grid, i, j);
                adjacent.insert((x, y, n));
            }
        }
    }
    Vec::from_iter(adjacent)
}

pub fn part_one(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut numbers = HashSet::new();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] != '.' && !grid[x][y].is_ascii_digit() {
                let adjacent = get_adjacent_numbers(&mut grid, x, y);
                for (x, y, n) in adjacent {
                    numbers.insert((x, y, n));
                }
            }
        }
    }

    numbers.iter().map(|(_, _, n)| *n).sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut gear_ratio_sum = 0;
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == '*' {
                let adjacent = get_adjacent_numbers(&mut grid, x, y);
                if adjacent.len() == 2 {
                    let gear_ratio = adjacent[0].2 * adjacent[1].2;
                    gear_ratio_sum += gear_ratio;
                }
            }
        }
    }
    gear_ratio_sum
}

pub fn main() {
    let input = get_input(3);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(3);
        assert_eq!(part_one(&input), 4361);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(3);
        assert_eq!(part_two(&input), 467835);
    }
}
