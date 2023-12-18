use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(6956, 455);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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

    fn is_part_of_loop(self) -> bool {
        match self {
            Tile::Vertical(part_of_loop)
            | Tile::Horizontal(part_of_loop)
            | Tile::NorthEast(part_of_loop)
            | Tile::NorthWest(part_of_loop)
            | Tile::SouthWest(part_of_loop)
            | Tile::SouthEast(part_of_loop) => part_of_loop,
            Tile::Ground => false,
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
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn find_start_piece(map: &Vec<Vec<char>>) -> ((usize, usize), Tile) {
    for (i, line) in map.iter().enumerate() {
        for (j, column) in line.iter().enumerate() {
            if column != &'S' {
                continue;
            }

            let left = (j.checked_sub(1))
                .map(|jj| ['-', 'L', 'F'].contains(&line[jj]))
                .unwrap_or_default();

            let right = (j + 1 < line.len())
                .then(|| ['-', 'J', '7'].contains(&line[j + 1]))
                .unwrap_or_default();

            let up = (i.checked_sub(1))
                .map(|ii| ['|', 'F', '7'].contains(&map[ii][j]))
                .unwrap_or_default();

            let down = (i + 1 < map.len())
                .then(|| ['|', 'L', 'J'].contains(&map[i + 1][j]))
                .unwrap_or_default();

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
    match &map[start.0][start.1] {
        Tile::Vertical(true) | Tile::SouthEast(true) => Direction::Up,
        Tile::Horizontal(true) | Tile::SouthWest(true) => Direction::Right,
        Tile::NorthWest(true) => Direction::Down,
        Tile::NorthEast(true) => Direction::Left,
        Tile::Ground => panic!("We never start on ground"),
        _ => panic!("Start tile should be part of the loop"),
    }
}

fn next_direction(map: &[Vec<Tile>], from: &Direction, start: &(usize, usize)) -> Direction {
    let current = &map[start.0][start.1];

    match (from, current) {
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
    next: &Direction,
    (current_row_index, current_column_index): &(usize, usize),
) -> (usize, usize) {
    match next {
        Direction::Up => (current_row_index - 1, *current_column_index),
        Direction::Right => (*current_row_index, current_column_index + 1),
        Direction::Down => (current_row_index + 1, *current_column_index),
        Direction::Left => (*current_row_index, current_column_index - 1),
    }
}

fn mark_coordinates_as_part_of_loop(map: &mut [Vec<Tile>], coordinates: (usize, usize)) {
    #[allow(clippy::match_on_vec_items)]
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

fn mark_loop(map: &mut [Vec<Tile>], start: (usize, usize)) -> usize {
    let mut from = get_any_start_direction(map, &start);

    let mut current = start;

    let mut steps = 1;

    mark_coordinates_as_part_of_loop(map, start);

    loop {
        let next = next_direction(map, &from, &current);

        let next_coordinates = next_coordinates(&next, &current);

        mark_coordinates_as_part_of_loop(map, next_coordinates);

        steps += 1;

        current = next_coordinates;

        from = next;

        if current == start {
            break;
        }
    }

    steps
}

fn count_enclosed(map: &mut [Vec<Tile>], start: (usize, usize)) -> usize {
    let mut count = 0;

    let _ = mark_loop(map, start);

    // 1 .. -1 because items on the edge are never enclosed
    for r in 1..map.len() - 1 {
        for c in 1..map[r].len() - 1 {
            if !map[r][c].is_part_of_loop() {
                count += ray_cast_tile(map, (r, c));
            }
        }
    }
    count
}

fn ray_cast_tile(map: &[Vec<Tile>], (from_row_index, from_column_index): (usize, usize)) -> usize {
    let mut count = 0;

    #[allow(clippy::needless_range_loop)]
    for row_index in 0..from_row_index {
        let tile = &map[row_index][from_column_index];

        if matches!(
            tile,
            Tile::Horizontal(true) | Tile::NorthEast(true) | Tile::SouthEast(true)
        ) {
            count += 1;
        }
    }

    usize::from(count % 2 != 0)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let lines = split_input(input);

        let (start, start_piece) = find_start_piece(&lines);

        let mut map = parse_lines(lines, start_piece);

        ((mark_loop(&mut map, start)) / 2).into()
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
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(6956, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                4,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                4,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                8,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 3))
            );
        }

        #[test]
        fn example_4() {
            assert_eq!(
                8,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 4))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(455, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                4,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 5))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                8,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 6))
            );
        }

        #[test]
        fn example_3() {
            assert_eq!(
                10,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 7))
            );
        }
    }
}
