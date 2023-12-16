use std::collections::HashMap;

use aoc23::input;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

pub fn push_check(
    stack: &mut Vec<(usize, usize, Direction)>,
    x: usize,
    y: usize,
    x_len: usize,
    y_len: usize,
    direction: Direction,
) {
    match direction {
        Direction::North => {
            if y > 0 {
                stack.push((x, y - 1, direction))
            }
        }
        Direction::South => {
            if y < y_len - 1 {
                stack.push((x, y + 1, direction))
            }
        }
        Direction::East => {
            if x < x_len - 1 {
                stack.push((x + 1, y, direction))
            }
        }
        Direction::West => {
            if x > 0 {
                stack.push((x - 1, y, direction))
            }
        }
    }
}

pub fn get_energized_tiles(
    start_x: usize,
    start_y: usize,
    start_direction: Direction,
    map: &[Vec<char>],
) -> usize {
    let len_x = map[0].len();
    let len_y = map.len();

    let mut tiles_visited = HashMap::new();

    let mut stack: Vec<(usize, usize, Direction)> = Vec::new();
    stack.push((start_x, start_y, start_direction));

    while let Some((x, y, direction)) = stack.pop() {
        if let Some(m) = tiles_visited.insert((x, y), vec![direction]) {
            if m.contains(&direction) {
                // We've already been here with this direction
                continue;
            }
        }

        match map[y][x] {
            '.' => push_check(&mut stack, x, y, len_x, len_y, direction),
            '/' => match direction {
                Direction::North => push_check(&mut stack, x, y, len_x, len_y, Direction::East),
                Direction::South => push_check(&mut stack, x, y, len_x, len_y, Direction::West),
                Direction::East => push_check(&mut stack, x, y, len_x, len_y, Direction::North),
                Direction::West => push_check(&mut stack, x, y, len_x, len_y, Direction::South),
            },
            '\\' => match direction {
                Direction::North => push_check(&mut stack, x, y, len_x, len_y, Direction::West),
                Direction::South => push_check(&mut stack, x, y, len_x, len_y, Direction::East),
                Direction::East => push_check(&mut stack, x, y, len_x, len_y, Direction::South),
                Direction::West => push_check(&mut stack, x, y, len_x, len_y, Direction::North),
            },
            '|' => match direction {
                Direction::East | Direction::West => {
                    push_check(&mut stack, x, y, len_x, len_y, Direction::North);
                    push_check(&mut stack, x, y, len_x, len_y, Direction::South);
                }
                _ => push_check(&mut stack, x, y, len_x, len_y, direction),
            },
            '-' => match direction {
                Direction::North | Direction::South => {
                    push_check(&mut stack, x, y, len_x, len_y, Direction::East);
                    push_check(&mut stack, x, y, len_x, len_y, Direction::West);
                }
                _ => push_check(&mut stack, x, y, len_x, len_y, direction),
            },
            _ => panic!("Invalid character"),
        }
    }
    tiles_visited.len()
}

pub fn part_one(input: &str) -> usize {
    let map = parse_input(input);
    get_energized_tiles(0, 0, Direction::East, &map)
}

pub fn part_two(input: &str) -> usize {
    let mut values = Vec::new();
    let map = parse_input(input);
    for x in 0..map[0].len() {
        values.push(get_energized_tiles(x, 0, Direction::South, &map));
        values.push(get_energized_tiles(
            x,
            map.len() - 1,
            Direction::North,
            &map,
        ));
    }

    for y in 0..map.len() {
        values.push(get_energized_tiles(0, y, Direction::East, &map));
        values.push(get_energized_tiles(
            map[0].len() - 1,
            y,
            Direction::West,
            &map,
        ));
    }

    values.into_iter().max().unwrap()
}

pub fn main() {
    let input = input::get_input(16);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(16);
        assert_eq!(part_one(&input), 46);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(16);
        assert_eq!(part_two(&input), 51);
    }
}
