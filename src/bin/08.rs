use std::array::TryFromSliceError;
use std::collections::BTreeMap;

use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(19667, 19_185_263_738_117usize);

enum Direction {
    Left = 0,
    Right = 1,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!("Couldn't convert '{}' into Direction", value)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node([char; 3]);

impl Node {
    fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }

    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }
}

impl TryFrom<&[char]> for Node {
    type Error = TryFromSliceError;

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        let x: [char; 3] = value.try_into()?;

        Ok(Node(x))
    }
}

struct Network {
    directions: Vec<Direction>,
    nodes: BTreeMap<Node, (Node, Node)>,
}

fn parse_node_line(node_line: &str) -> (Node, (Node, Node)) {
    let as_chars = node_line.chars().collect::<Vec<char>>();
    // AAA = (BBB, CCC)
    let from_node: Node = as_chars[0..3].try_into().expect("Invalid node");

    let to_node_left: Node = as_chars[7..10].try_into().expect("Invalid node");
    let to_node_right: Node = as_chars[12..15].try_into().expect("Invalid node");

    (from_node, (to_node_left, to_node_right))
}

fn parse_lines(input: &str) -> Network {
    let (unparsed_directions, unparsed_node_lines) =
        input.split_once("\n\n").expect("Invalid network");

    Network {
        directions: unparsed_directions
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect(),
        nodes: unparsed_node_lines.lines().map(parse_node_line).collect(),
    }
}

fn follow_directions(network: &Network) -> usize {
    const START: Node = Node(['A', 'A', 'A']);
    const END: Node = Node(['Z', 'Z', 'Z']);

    let mut steps = 0;
    let mut current: &Node = &START;

    for direction in network.directions.iter().cycle() {
        if current == &END {
            break;
        }
        steps += 1;

        let next = network.nodes.get(current).expect("No next?");

        match direction {
            Direction::Left => current = &next.0,
            Direction::Right => current = &next.1,
        }
    }

    steps
}

fn count_steps<'node, 'network>(mut current: &'node Node, network: &'network Network) -> usize
where
    'network: 'node,
{
    let mut steps = 0;

    while !current.is_end() {
        let next = network.nodes.get(current).expect("No next?");

        match network.directions.get(steps % network.directions.len()) {
            Some(Direction::Left) => current = &next.0,
            Some(Direction::Right) => current = &next.1,
            None => panic!("Not found"),
        }

        steps += 1;
    }

    steps
}

fn gcd(left: usize, right: usize) -> usize {
    if right == 0 {
        left
    } else {
        gcd(right, left % right)
    }
}

fn lcm(left: usize, right: usize) -> usize {
    (left * right) / gcd(left, right)
}

fn follow_directions_from_multiple(network: &Network, starts: Vec<&Node>) -> usize {
    starts
        .into_iter()
        .map(|node| count_steps(node, network))
        .reduce(lcm)
        .unwrap()
}

fn find_starts(nodes: &BTreeMap<Node, (Node, Node)>) -> Vec<&Node> {
    nodes
        .iter()
        .filter_map(|(f, _)| if f.is_start() { Some(f) } else { None })
        .collect::<Vec<&Node>>()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let network = parse_lines(input);

        follow_directions(&network).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let network = parse_lines(input);

        let starts = find_starts(&network.nodes);

        follow_directions_from_multiple(&network, starts).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(19667, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                2,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                6,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                19_185_263_738_117usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                6,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 3))
            );
        }
    }
}
