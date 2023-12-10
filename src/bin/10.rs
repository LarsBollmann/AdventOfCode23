use aoc23::input::get_input;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pipe {
    connections: Vec<Direction>,
}

impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            '|' => Pipe {
                connections: vec![Direction::North, Direction::South],
            },
            '-' => Pipe {
                connections: vec![Direction::East, Direction::West],
            },
            'L' => Pipe {
                connections: vec![Direction::North, Direction::East],
            },
            'J' => Pipe {
                connections: vec![Direction::North, Direction::West],
            },
            '7' => Pipe {
                connections: vec![Direction::South, Direction::West],
            },
            'F' => Pipe {
                connections: vec![Direction::South, Direction::East],
            },
            'S' => Pipe {
                connections: vec![
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ],
            },
            '.' => Pipe {
                connections: vec![],
            },
            _ => panic!("Unknown pipe type: {}", c),
        }
    }

    fn is_start(&self) -> bool {
        self.connections.len() == 4
    }

    fn is_connected(&self, other: &Pipe, direction: Direction) -> bool {
        self.connections.contains(&direction) && other.connections.contains(&direction.opposite())
    }
}

pub fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<Pipe>>) {
    let mut start = (0, 0);
    let map: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    Pipe::from_char(c)
                })
                .collect()
        })
        .collect();

    (start, map)
}

pub fn get_next_connected(
    map: &[Vec<Pipe>],
    x: usize,
    y: usize,
    exclude: Direction,
) -> (usize, usize, Direction) {
    let pipe = &map[y][x];

    if exclude != Direction::North && y > 0 && pipe.is_connected(&map[y - 1][x], Direction::North) {
        return (x, y - 1, Direction::South);
    }
    if exclude != Direction::South
        && y < map.len() - 1
        && pipe.is_connected(&map[y + 1][x], Direction::South)
    {
        return (x, y + 1, Direction::North);
    }
    if exclude != Direction::East
        && x < map[y].len() - 1
        && pipe.is_connected(&map[y][x + 1], Direction::East)
    {
        return (x + 1, y, Direction::West);
    }
    if exclude != Direction::West && x > 0 && pipe.is_connected(&map[y][x - 1], Direction::West) {
        return (x - 1, y, Direction::East);
    }

    panic!("No connected pipe found at ({}, {})", x, y);
}

pub fn part_one(input: &str) -> i32 {
    let ((mut x, mut y), map) = parse_input(input);

    let mut steps = 0;
    let mut current_pipe = &map[y][x];
    let mut coming_from = Direction::South;

    let mut pipe_coordinates = vec![(x, y)];

    loop {
        steps += 1;
        (x, y, coming_from) = get_next_connected(&map, x, y, coming_from);
        current_pipe = &map[y][x];
        pipe_coordinates.push((x, y));

        if current_pipe.is_start() {
            break;
        }
    }

    steps / 2
}

// pub fn part_two(input: &str) -> i32 {
//     1
// }

pub fn main() {
    let input = get_input(10);
    println!("Part one: {:?}", part_one(&input));
    //println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(10);
        assert_eq!(part_one(&input), 8);
    }

    //     #[test]
    //     fn test_part_two() {
    //         let input = "..........
    // .S------7.
    // .|F----7|.
    // .||....||.
    // .||....||.
    // .|L-7F-J|.
    // .|..||..|.
    // .L--JL--J.
    // ..........";
    //         assert_eq!(part_one(&input), 2);
    //     }
}
