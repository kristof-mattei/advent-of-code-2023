use std::cell::Cell;
use std::collections::BinaryHeap;
use std::ops::RangeInclusive;

use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2023::solution!(928, 1104);

struct Block {
    value: u32,
    visited: Cell<bool>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coordinates {
    row_index: usize,
    column_index: usize,
}

impl From<(usize, usize)> for Coordinates {
    fn from((row_index, column_index): (usize, usize)) -> Self {
        Coordinates {
            row_index,
            column_index,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply<T>(self, city_map: &[Vec<T>], row_column_index: &Coordinates) -> Option<Coordinates> {
        let row_limit = city_map.len();
        let column_limit = city_map[0].len();

        let Coordinates {
            row_index,
            column_index,
        } = row_column_index;

        match self {
            Direction::Up => (row_index.checked_sub(1)).map(|up| (up, *column_index).into()),
            Direction::Right => {
                let right = column_index + 1;
                if (right) < column_limit {
                    Some((*row_index, right).into())
                } else {
                    None
                }
            },
            Direction::Down => {
                let down = row_index + 1;

                if (down) < row_limit {
                    Some((down, *column_index).into())
                } else {
                    None
                }
            },
            Direction::Left => (column_index.checked_sub(1)).map(|left| (*row_index, left).into()),
        }
    }
}

fn get_neighbor_directions<T>(city_map: &[Vec<T>], node: &Node) -> Vec<(Coordinates, Direction)> {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Down,
        Direction::Up,
        Direction::Right,
        Direction::Left,
    ];

    let mut neighbors = Vec::with_capacity(3);

    for directions in DIRECTIONS {
        if let Some(new_row_column_index) = directions.apply(city_map, &node.row_column_index) {
            if new_row_column_index != node.previous {
                neighbors.push((new_row_column_index, directions));
            }
        }
    }

    neighbors
}

fn parse_lines(lines: &[&str]) -> Vec<Vec<Block>> {
    let mut map = Vec::new();

    for line in lines {
        map.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .map(|x| Block {
                    value: x,
                    visited: Cell::new(false),
                })
                .collect(),
        );
    }

    map
}

fn reconstruct_path(
    map: &[Vec<Block>],
    mut came_from: HashMap<Node, Node>,
    mut current: Node,
) -> Vec<Coordinates> {
    let mut total_path = vec![current.row_column_index];

    while let Some(next) = came_from.remove(&current) {
        total_path.push(next.row_column_index);

        current = next;
    }

    for p in &total_path {
        map[p.row_index][p.column_index].visited.set(true);
    }

    total_path.reverse();
    total_path
}

fn distance(map: &[Vec<Block>], _current: Coordinates, neighbor: Coordinates) -> u32 {
    map[neighbor.row_index][neighbor.column_index].value
}

fn heuristic(map: &[Vec<Block>], current: Coordinates) -> u32 {
    #![allow(clippy::cast_possible_truncation)]
    (map.len() - 1 - current.row_index + map[0].len() - 1 - current.column_index) as u32
}

#[derive(Clone)]
struct Node {
    row_column_index: Coordinates,
    previous_direction: Direction,
    previous: Coordinates,
    count_of_direction: u32,
    f_score: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.row_column_index == other.row_column_index
            && self.previous_direction == other.previous_direction
            && self.count_of_direction == other.count_of_direction
    }
}

impl Eq for Node {}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.row_column_index.hash(state);
        self.previous_direction.hash(state);
        self.count_of_direction.hash(state);
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R: {}, C: {}", self.row_index, self.column_index)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // forward to Ord
        Some(self.cmp(other))
    }
}

fn a_star(
    map: &mut [Vec<Block>],
    start: Coordinates,
    goal: Coordinates,
    min_max: RangeInclusive<u32>,
) -> Vec<Coordinates> {
    let start = Node {
        row_column_index: start,
        previous_direction: Direction::Up,
        previous: start,
        count_of_direction: *min_max.start(),
        f_score: heuristic(map, start),
    };

    let mut came_from = HashMap::<Node, Node>::new();

    let mut g_score: HashMap<Node, u32> = HashMap::from([(start.clone(), 0)]);

    // we don't keep an f_score as it is incoorporated in the Node

    let mut open_set = BinaryHeap::from([start]);

    while let Some(node) = open_set.pop() {
        if node.row_column_index == goal {
            return reconstruct_path(map, came_from, node);
        }

        let neighbors = get_neighbor_directions(map, &node);

        for (neighbor, direction) in neighbors {
            let count = if direction == node.previous_direction {
                if &node.count_of_direction < min_max.end() {
                    node.count_of_direction + 1
                } else {
                    continue;
                }
            } else if &node.count_of_direction >= min_max.start() {
                // different direction, allowed as we're beyond min
                1
            } else {
                continue;
            };

            let tentative_g_score =
                g_score.get(&node).unwrap() + distance(map, node.row_column_index, neighbor);

            let f_score = tentative_g_score + heuristic(map, neighbor);

            let neighbor_node = Node {
                row_column_index: neighbor,
                previous_direction: direction,
                previous: node.row_column_index,
                count_of_direction: count,
                f_score,
            };

            if tentative_g_score < *g_score.get(&neighbor_node).unwrap_or(&u32::MAX) {
                g_score.insert(neighbor_node.clone(), tentative_g_score);
                came_from.insert(neighbor_node.clone(), node.clone());
                open_set.push(neighbor_node);
            }
        }
    }

    panic!("No solution found")
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let lines: Vec<&str> = input.lines().collect();

        let mut parsed = parse_lines(&lines);

        let max_row = parsed.len() - 1;
        let max_col = parsed[0].len() - 1;

        let cheapest = a_star(&mut parsed, (0, 0).into(), (max_row, max_col).into(), 0..=3);

        dump_map(&parsed);

        cheapest
            .iter()
            .skip(1)
            .map(
                |Coordinates {
                     row_index: r,
                     column_index: c,
                 }| (parsed[*r][*c]).value,
            )
            .sum::<u32>()
            .into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let lines: Vec<&str> = input.lines().collect();

        let mut parsed = parse_lines(&lines);

        let max_row = parsed.len() - 1;
        let max_col = parsed[0].len() - 1;

        let cheapest = a_star(
            &mut parsed,
            (0, 0).into(),
            (max_row, max_col).into(),
            4..=10,
        );

        dump_map(&parsed);

        cheapest
            .iter()
            .skip(1)
            .map(
                |Coordinates {
                     row_index: r,
                     column_index: c,
                 }| (parsed[*r][*c]).value,
            )
            .sum::<u32>()
            .into()
    }
}

fn dump_map(map: &[Vec<Block>]) {
    for r in map {
        for c in r {
            let color: u32 = if c.visited.get() { 31 } else { 0 };
            print!("\x1b[{}m{}\x1b[0m", color, c.value);
        }

        println!();
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::Parts;
        use advent_of_code_2023::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(928, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(102, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::Parts;
        use advent_of_code_2023::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(1104, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(94, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
