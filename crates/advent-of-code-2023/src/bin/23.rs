#![expect(clippy::type_complexity, reason = "It's Advent of Code")]
use advent_of_code_2023::shared::grids::grid::Grid;
use advent_of_code_2023::shared::grids::{
    GridIter as _, HorizontalVerticalDirection, Neighbors as _,
};
use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::hash_map::Entry;
use hashbrown::{HashMap, HashSet};

advent_of_code_2023::solution!(490, 6726);

enum Block {
    Open,
    Closed,
    Slope(HorizontalVerticalDirection),
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Block::Open => '.',
            Block::Closed => '#',
            Block::Slope(HorizontalVerticalDirection::Right) => '>',
            Block::Slope(HorizontalVerticalDirection::Left) => '<',
            Block::Slope(HorizontalVerticalDirection::Up) => '^',
            Block::Slope(HorizontalVerticalDirection::Down) => 'v',
        };

        write!(f, "{}", c)
    }
}

impl TryFrom<char> for Block {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Block::Open),
            '#' => Ok(Block::Closed),
            '>' => Ok(Block::Slope(HorizontalVerticalDirection::Right)),
            '<' => Ok(Block::Slope(HorizontalVerticalDirection::Left)),
            '^' => Ok(Block::Slope(HorizontalVerticalDirection::Up)),
            'v' => Ok(Block::Slope(HorizontalVerticalDirection::Down)),
            _ => Err("Invalid block"),
        }
    }
}

fn parse_input(input: &str) -> Grid<Block> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Block::try_from(c).expect("Invalid input"))
                .collect()
        })
        .collect();

    Grid::new(grid)
}

fn find_longest_path(grid: &Grid<Block>) -> Option<usize> {
    let start = (0, 1);

    let end = (grid.get_row_length() - 1, grid.get_column_length() - 2);

    assert!(matches!(grid[end.0][end.1], Block::Open), "Bad input");

    let results = go_forth(
        grid,
        start,
        HorizontalVerticalDirection::Down,
        HashMap::new(),
        0,
        end,
    );

    results.into_iter().max()
}

fn go_forth(
    grid: &Grid<Block>,
    (row_index, column_index): (usize, usize),
    direction: HorizontalVerticalDirection,
    history: HashMap<(usize, usize), HorizontalVerticalDirection>,
    actual_count: usize,
    end: (usize, usize),
) -> Vec<usize> {
    let mut list = vec![((row_index, column_index), direction, history, actual_count)];

    let mut all_paths = vec![];

    while let Some(((row_index, column_index), direction, mut history, actual_count)) = list.pop() {
        // have we been here before?
        if history.contains_key(&(row_index, column_index)) {
            continue;
        }

        let (row_index, column_index) = match grid[row_index][column_index] {
            Block::Open => {
                // good, continue and evaluate neighbors
                // possible optimization: don't consider the position we're coming from
                // as a neighbor, avoiding the need of looking it up
                // in or hv_neighbors filter call
                (row_index, column_index)
            },
            Block::Closed => {
                // non-viable
                continue;
            },
            Block::Slope(ref horizontal_vertical_direction) => {
                let next = match *horizontal_vertical_direction {
                    HorizontalVerticalDirection::Up => (row_index - 1, column_index),
                    HorizontalVerticalDirection::Right => (row_index, column_index + 1),
                    HorizontalVerticalDirection::Down => (row_index + 1, column_index),
                    HorizontalVerticalDirection::Left => (row_index, column_index - 1),
                };

                list.push((
                    next,
                    *horizontal_vertical_direction,
                    history,
                    actual_count + 1,
                ));

                continue;
            },
        };

        if (row_index, column_index) == end {
            all_paths.push(actual_count);

            continue;
        }

        let neighbors = grid.hv_neighbors(row_index, column_index);

        let neighbors = neighbors
            .iter()
            .filter(|&&(ref coordinates, neighbor_direction)| {
                !matches!(grid[coordinates.0][coordinates.1], Block::Closed)
                    && (!direction) != neighbor_direction
            })
            .collect::<Vec<_>>();

        match &*neighbors {
            &[] => {},
            &[&(neighbor_coordinates, direction)] => {
                list.push((neighbor_coordinates, direction, history, actual_count + 1));
            },
            many @ &[..] => {
                // store the intersection in case we come here again
                if history
                    .insert((row_index, column_index), direction)
                    .is_some()
                {
                    panic!("NO");
                }

                for &&(neighbor_coordinates, direction) in many {
                    list.push((
                        neighbor_coordinates,
                        direction,
                        history.clone(),
                        actual_count + 1,
                    ));
                }
            },
        }
    }

    all_paths
}

#[derive(Clone)]
struct Graph {
    map: HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn record(&mut self, from: (usize, usize), to: (usize, usize), distance: usize) {
        match self.map.entry(from) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().insert(to, distance);
            },
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert([(to, distance)].into_iter().collect());
            },
        }

        match self.map.entry(to) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().insert(from, distance);
            },
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert([(from, distance)].into_iter().collect());
            },
        }
    }
}

fn brute_force_graph(grid: &Grid<Block>) -> usize {
    let start = (0, 1);
    let end = (grid.get_row_length() - 1, grid.get_column_length() - 2);

    let mut list = vec![((start, start), HorizontalVerticalDirection::Down, 0)];

    let mut graph = Graph::new();

    let mut visited = HashSet::new();

    while let Some(((from, (row_index, column_index)), direction, distance)) = list.pop() {
        if (row_index, column_index) == end {
            graph.record(from, (row_index, column_index), distance);

            continue;
        }

        let neighbors = grid.hv_neighbors(row_index, column_index);

        let neighbors = neighbors
            .iter()
            .filter(|&&(ref coordinates, neighbor_direction)| {
                !matches!(grid[coordinates.0][coordinates.1], Block::Closed)
                    && (!direction) != neighbor_direction
            })
            .collect::<Vec<_>>();

        match &*neighbors {
            &[] => continue,
            &[&(neighbor, neighbor_direction)] => {
                list.push(((from, neighbor), neighbor_direction, distance + 1));
            },
            all @ &[..] => {
                // more than 1 neighbor, we are at an intersection, record distance

                graph.record(from, (row_index, column_index), distance);

                // mark intersection as visited
                if !visited.insert((row_index, column_index)) {
                    continue;
                }

                for &&(neighbor, neighbor_direction) in all {
                    list.push((((row_index, column_index), neighbor), neighbor_direction, 1));
                }
            },
        }
    }

    // convert hashmap of neighbors to vec so we can record
    // which ones we've done without storing the list
    let node_to_neighbors = graph
        .map
        .into_iter()
        .map(|(from, edges)| (from, edges.into_iter().collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();

    let mut longest = 0_usize;
    let mut path_visited = HashSet::new();

    // (node, index into neighbors, accumulated distance)
    let mut stack: Vec<(((usize, usize), usize), usize)> = Vec::new();

    path_visited.insert(start);
    stack.push(((start, 0), 0));

    while let Some(((node, index), accumulated_distance)) = stack.pop() {
        if node == end {
            longest = longest.max(accumulated_distance);
        }

        let Some(neighbors) = node_to_neighbors.get(&node) else {
            // this node has no neighbors, which actually doesn't happen in thise code-base (otherwise how did we gethere?)
            // in a different graph the edges might be unidirectional
            // don't remove the start node
            if node != start {
                // nowhere to go but back
                path_visited.remove(&node);
            }

            continue;
        };

        let Some(&(neighbor, distance_to_neighbor)) = neighbors.get(index) else {
            // no more neighbors to process
            // don't remove the start node
            if node != start {
                // nowhere to go but back
                path_visited.remove(&node);
            }

            continue;
        };

        // queue up the next neighbor
        stack.push(((node, index + 1), accumulated_distance));

        if path_visited.contains(&neighbor) {
            continue;
        }

        // this neighbor works, let's explore from there
        path_visited.insert(neighbor);
        stack.push(((neighbor, 0), accumulated_distance + distance_to_neighbor));
    }

    longest
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let longest_path = find_longest_path(&parsed);

        // -1 because we don't want to count start
        PartSolution::USize(longest_path.unwrap_or_default())
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let longest_path = brute_force_graph(&parsed);

        // -1 because we don't want to count start
        PartSolution::USize(longest_path)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2023::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(2502);
        }

        #[test]
        fn example() {
            test_example_part_1!(94);
        }
    }

    mod part_2 {
        use advent_of_code_2023::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(6726);
        }

        #[test]
        fn example() {
            test_example_part_2!(154);
        }
    }
}
