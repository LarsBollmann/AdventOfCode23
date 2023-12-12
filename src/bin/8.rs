use std::collections::HashMap;

use aoc23::input::get_input;

type Map<'a> = HashMap<&'a str, Node<'a>>;

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_input<'a>(input: &'a str) -> (Map<'a>, impl Iterator<Item = char> + 'a) {
    let mut map = Map::new();

    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().cycle();

    for line in nodes.lines() {
        let (name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();
        map.insert(name, Node { left, right });
    }

    (map, instructions)
}

fn part_one(input: &str) -> u32 {
    let (map, instructions) = parse_input(input);

    let mut current_node = "AAA";
    let mut steps = 0;

    for instruction in instructions {
        let node = map.get(current_node).unwrap();
        match instruction {
            'L' => current_node = node.left,
            'R' => current_node = node.right,
            _ => panic!("Unknown instruction: {}", instruction),
        }
        steps += 1;
        if current_node == "ZZZ" {
            break;
        }
    }

    steps
}

fn part_two(input: &str) -> u32 {
    let (map, instructions) = parse_input(input);

    let mut current_nodes: Vec<_> = map.keys().filter(|k| k.ends_with('A')).copied().collect();
    let mut steps = 0;

    for instruction in instructions {
        let mut all_done = true;
        for current_node in 0..current_nodes.len() {
            let node = map.get(current_nodes[current_node]).unwrap();
            match instruction {
                'L' => current_nodes[current_node] = node.left,
                'R' => current_nodes[current_node] = node.right,
                _ => panic!("Unknown instruction: {}", instruction),
            }
            if !current_nodes[current_node].ends_with('Z') {
                all_done = false;
            }
        }
        steps += 1;
        if all_done {
            break;
        }
    }
    steps
}

fn main() {
    let input = get_input(8);
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc23::input::get_example;

    #[test]
    fn test_part_one() {
        let input = get_example(8);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part_two(&input), 6);
    }
}
