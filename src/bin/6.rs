use aoc23::input::get_input;

// This analytically calculates and counts all integer solutions to the equation (t-x) * x > d
// where t is the time and d is the distance.
pub fn get_number_of_solutions(time: usize, distance: usize) -> usize {
    let time = time as f64;
    let distance = distance as f64;

    // No solutions exist
    if distance < 0.0 || time <= 2.0 * distance.sqrt() {
        return 0;
    }

    let win_time_min = match 0.5 * (time - (time.powi(2) - 4.0 * distance).sqrt()) {
        i if i.fract() == 0.0 => i as usize + 1,
        i => i.ceil() as usize,
    };
    let win_time_max = match 0.5 * ((time.powi(2) - 4.0 * distance).sqrt() + time) {
        i if i.fract() == 0.0 => i as usize - 1,
        i => i.floor() as usize,
    };

    win_time_max - win_time_min + 1
}


pub fn part_one(input: &str) -> usize {
    let races = {
        let (line1, line2) = input.split_once("\n").unwrap();

        let times = line1.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<_>>();
        let distances = line2.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    
        times.into_iter().zip(distances.into_iter())
    };

    races.map(|(time, distance)| get_number_of_solutions(time, distance)).reduce(|a, b| a * b).unwrap()
}

pub fn part_two(input: &str) -> usize {
    let races = {
        let (line1, line2) = input.split_once("\n").unwrap();

        let times: usize = line1.split_once(":").unwrap().1.replace(" ", "").parse().unwrap();
        let distances: usize = line2.split_once(":").unwrap().1.replace(" ", "").trim().parse().unwrap();
    
        (times, distances)
    };

    get_number_of_solutions(races.0, races.1)
}

pub fn main() {
    let input = get_input(6);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(6);
        assert_eq!(part_one(&input), 288);
    }

    #[test]
    fn test_part_two() {
        let input = get_example(6);
        assert_eq!(part_two(&input), 71503);
    }
}
