use aoc23::input::get_input;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    match start > end {
        true => end..start,
        false => start..end,
    }
}

pub fn solve(input: &str, expansion_factor: usize) -> usize {
    let map = parse_input(input);

    let mut rows_to_expand = Vec::new();
    let mut cols_to_expand = Vec::new();

    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            rows_to_expand.push(i);
        }
    }

    for i in 0..map[0].len() {
        if map.iter().all(|row| row[i] == '.') {
            cols_to_expand.push(i);
        }
    }

    let mut galaxies = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let mut sum = 0;

    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for (x2, y2) in galaxies.iter().skip(i + 1) {
            let rows_that_need_expanding = range(*x1, *x2)
                .filter(|&x| rows_to_expand.contains(&(x as usize)))
                .count();
            let distance_x: usize =
                range(*x1, *x2).count() + rows_that_need_expanding * (expansion_factor - 1);

            let cols_that_need_expanding = range(*y1, *y2)
                .filter(|&y| cols_to_expand.contains(&(y as usize)))
                .count();
            let distance_y: usize =
                range(*y1, *y2).count() + cols_that_need_expanding * (expansion_factor - 1);

            sum += distance_x + distance_y;
        }
    }

    sum
}

pub fn main() {
    let input = get_input(11);
    println!("Part one: {}", solve(&input, 2));
    println!("Part two: {}", solve(&input, 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(11);
        assert_eq!(solve(&input, 2), 374);
    }

    #[test]
    fn test_part_two_10() {
        let input = get_example(11);
        assert_eq!(solve(&input, 10), 1030);
    }

    #[test]
    fn test_part_two_100() {
        let input = get_example(11);
        assert_eq!(solve(&input, 100), 8410);
    }
}
