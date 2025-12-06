use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(6956, 455);

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
enum Tile {
    Vertical(bool),
    Horizontal(bool),
    NorthEast(bool),
    NorthWest(bool),
    SouthWest(bool),
    SouthEast(bool),
    Ground,
}

impl Tile {
    fn from_board_value(value: char, start_piece: Tile) -> Result<Tile, &'static str> {
        match value {
            '|' => Ok(Tile::Vertical(false)),
            '-' => Ok(Tile::Horizontal(false)),
            'L' => Ok(Tile::NorthEast(false)),
            'J' => Ok(Tile::NorthWest(false)),
            '7' => Ok(Tile::SouthWest(false)),
            'F' => Ok(Tile::SouthEast(false)),
            '.' => Ok(Tile::Ground),
            'S' => Ok(start_piece),
            _ => Err("Invalid character"),
        }
    }
}

fn split_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<_>>>()
}

fn parse_lines(lines: Vec<Vec<char>>, start_piece: Tile) -> Vec<Vec<Tile>> {
    lines
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|c| Tile::from_board_value(c, start_piece).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_start_piece(map: &[Vec<char>]) -> ((usize, usize), Tile) {
    for (i, line) in map.iter().enumerate() {
        for (j, column) in line.iter().enumerate() {
            if column != &'S' {
                continue;
            }

            let left = (j.checked_sub(1)).is_some_and(|jj| ['-', 'L', 'F'].contains(&line[jj]));

            let right = if j + 1 < line.len() {
                ['-', 'J', '7'].contains(&line[j + 1])
            } else {
                false
            };

            let up = (i.checked_sub(1)).is_some_and(|ii| ['|', 'F', '7'].contains(&map[ii][j]));

            let down = if i + 1 < map.len() {
                ['|', 'L', 'J'].contains(&map[i + 1][j])
            } else {
                false
            };

            let start = match (up, down, left, right) {
                // coming from top to bottom
                (true, true, false, false) => Tile::Vertical(true),
                // coming from top to left
                (true, false, true, false) => Tile::NorthWest(true),
                // top to right
                (true, false, false, true) => Tile::NorthEast(true),
                // coming from below to left
                (false, true, true, false) => Tile::SouthWest(true),
                // coming from below to right
                (false, true, false, true) => Tile::SouthEast(true),
                // coming from left to right
                (false, false, true, true) => Tile::Horizontal(true),
                _ => panic!("Invalid S"),
            };

            return ((i, j), start);
        }
    }

    panic!("Start not found");
}

fn get_any_start_direction(map: &[Vec<Tile>], start: &(usize, usize)) -> Direction {
    match map[start.0][start.1] {
        Tile::Vertical(true) | Tile::SouthEast(true) => Direction::Up,
        Tile::Horizontal(true) | Tile::SouthWest(true) => Direction::Right,
        Tile::NorthWest(true) => Direction::Down,
        Tile::NorthEast(true) => Direction::Left,
        Tile::Ground => panic!("We never start on ground"),
        Tile::Vertical(_)
        | Tile::Horizontal(_)
        | Tile::NorthEast(_)
        | Tile::NorthWest(_)
        | Tile::SouthWest(_)
        | Tile::SouthEast(_) => panic!("Start tile should be part of the loop"),
    }
}

fn next_direction(map: &[Vec<Tile>], from: Direction, start: &(usize, usize)) -> Direction {
    let current = &map[start.0][start.1];

    match (from, *current) {
        (Direction::Up, Tile::Vertical(true))
        | (Direction::Right, Tile::NorthWest(true))
        | (Direction::Left, Tile::NorthEast(true)) => Direction::Up,
        (Direction::Up, Tile::SouthEast(true))
        | (Direction::Right, Tile::Horizontal(true))
        | (Direction::Down, Tile::NorthEast(true)) => Direction::Right,
        (Direction::Up, Tile::SouthWest(true))
        | (Direction::Down, Tile::NorthWest(true))
        | (Direction::Left, Tile::Horizontal(true)) => Direction::Left,
        (Direction::Right, Tile::SouthWest(true))
        | (Direction::Down, Tile::Vertical(true))
        | (Direction::Left, Tile::SouthEast(true)) => Direction::Down,
        _ => panic!("Invalid directions"),
    }
}

fn next_coordinates(
    next: Direction,
    &(current_row_index, current_column_index): &(usize, usize),
) -> (usize, usize) {
    match next {
        Direction::Up => (current_row_index - 1, current_column_index),
        Direction::Right => (current_row_index, current_column_index + 1),
        Direction::Down => (current_row_index + 1, current_column_index),
        Direction::Left => (current_row_index, current_column_index - 1),
    }
}

fn mark_coordinates_as_part_of_loop(map: &mut [Vec<Tile>], coordinates: (usize, usize)) {
    let new_tile = match map[coordinates.0][coordinates.1] {
        Tile::Vertical(_) => Tile::Vertical(true),
        Tile::Horizontal(_) => Tile::Horizontal(true),
        Tile::NorthEast(_) => Tile::NorthEast(true),
        Tile::NorthWest(_) => Tile::NorthWest(true),
        Tile::SouthWest(_) => Tile::SouthWest(true),
        Tile::SouthEast(_) => Tile::SouthEast(true),
        Tile::Ground => panic!("Ground cannot be part of loop"),
    };

    map[coordinates.0][coordinates.1] = new_tile;
}

fn mark_loop(map: &mut [Vec<Tile>], start: (usize, usize)) -> (Vec<(usize, usize)>, usize) {
    let mut from = get_any_start_direction(map, &start);

    let mut current = start;

    let mut steps = 1;

    let mut coordinates = vec![];

    mark_coordinates_as_part_of_loop(map, start);

    loop {
        let next = next_direction(map, from, &current);

        if next != from {
            coordinates.push(current);
        }

        let next_coordinates = next_coordinates(next, &current);

        mark_coordinates_as_part_of_loop(map, next_coordinates);

        steps += 1;

        current = next_coordinates;

        from = next;

        if current == start {
            coordinates.push(current);
            break;
        }
    }

    (coordinates, steps)
}

pub(crate) fn shoelace(coordinates: &[(usize, usize)], perimeter: usize) -> usize {
    let mut size: isize = 0;

    for coord in coordinates.windows(2) {
        // [ a b ]
        // [ c d ]

        let a: isize = coord[0].0.try_into().unwrap();
        let b: isize = coord[0].1.try_into().unwrap();
        let c: isize = coord[1].0.try_into().unwrap();
        let d: isize = coord[1].1.try_into().unwrap();

        size += (a * d) - (b * c);
    }

    TryInto::<usize>::try_into((size / 2).abs()).unwrap() - (perimeter / 2) + 1
}

fn count_enclosed(map: &mut [Vec<Tile>], start: (usize, usize)) -> usize {
    let (coordinates, perimeter) = mark_loop(map, start);

    shoelace(&coordinates, perimeter)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let lines = split_input(input);

        let (start, start_piece) = find_start_piece(&lines);

        let mut map = parse_lines(lines, start_piece);

        let (_, steps) = mark_loop(&mut map, start);

        ((steps) / 2).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let lines = split_input(input);

        let (start, start_piece) = find_start_piece(&lines);

        let mut map = parse_lines(lines, start_piece);

        count_enclosed(&mut map, start).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {

        use advent_of_code_2023::{test_example_part_1, test_part_1};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_1!(6956);
        }

        #[test]
        fn example_1() {
            test_example_part_1!(4, 1);
        }

        #[test]
        fn example_2() {
            test_example_part_1!(4, 2);
        }

        #[test]
        fn example_3() {
            test_example_part_1!(8, 3);
        }

        #[test]
        fn example_4() {
            test_example_part_1!(8, 4);
        }
    }

    mod part_2 {

        use advent_of_code_2023::{test_example_part_2, test_part_2};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_2!(455);
        }

        #[test]
        fn example_1() {
            test_example_part_2!(4, 5);
        }

        #[test]
        fn example_2() {
            test_example_part_2!(8, 6);
        }

        #[test]
        fn example_3() {
            test_example_part_2!(10, 7);
        }
    }
}
