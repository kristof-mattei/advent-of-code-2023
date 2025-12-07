use std::collections::VecDeque;
use std::num::NonZeroUsize;

use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2023::solution!(3503, 584_211_423_220_706_usize);
enum Tile {
    Rock,
    Garden(bool),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Rock => write!(f, "#"),
            Tile::Garden(false) => write!(f, "."),
            Tile::Garden(true) => write!(f, "O"),
        }
    }
}

fn neighbors(_map: &[Vec<Tile>], r: isize, c: isize) -> Vec<(isize, isize)> {
    vec![(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)]
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let mut map = vec![];

    for line in input.lines() {
        let mut row = vec![];

        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Rock,
                '.' => Tile::Garden(false),
                'S' => Tile::Garden(true),
                _ => panic!("Seriously?"),
            };

            row.push(tile);
        }

        map.push(row);
    }

    map
}

fn bfs(grid: &[Vec<Tile>], start: (isize, isize), steps: NonZeroUsize) -> usize {
    let mut visited = HashSet::new();
    let mut garden_plots = 0;

    let even_odd = steps.get() % 2;

    let mut dq = VecDeque::from_iter([(start, 0_usize)]);

    while let Some((coord, steps_traveled)) = dq.pop_front() {
        if steps_traveled > steps.get() || visited.contains(&coord) {
            continue;
        }

        visited.insert(coord);

        if steps_traveled % 2 == even_odd {
            garden_plots += 1;
        }

        let row_len: isize = grid.len().try_into().expect("Index too long");
        let column_len: isize = grid[0].len().try_into().expect("Index too long");

        for new_coord in neighbors(grid, coord.0, coord.1) {
            if matches!(
                grid[new_coord.0.rem_euclid(row_len).unsigned_abs()]
                    [new_coord.1.rem_euclid(column_len).unsigned_abs()],
                Tile::Garden(_)
            ) {
                dq.push_back((new_coord, steps_traveled + 1));
            }
        }
    }

    println!("Garden plots: {garden_plots}");
    garden_plots
}

fn part_2(garden: &[Vec<Tile>], start: (usize, usize)) -> usize {
    // forgive me, for I am not smart enough to understand this problem
    // this solution is the Rust version of https://github.com/terminalmage/adventofcode/blob/4a52a87f4af8908e4ef6df637680a04770a3a27e/2023/day21.py#L256
    let steps: isize = 26_501_365;

    let columns = garden[0].len();

    let edge = columns - 1 - start.1;

    let [u0, u1, u2]: [usize; 3] = (0..3)
        .map(|x| {
            bfs(
                garden,
                (start.0.try_into().unwrap(), start.1.try_into().unwrap()),
                NonZeroUsize::new(edge + (columns * x)).unwrap(),
            )
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let (u0, u1, u2): (isize, isize, isize) = (
        u0.try_into().unwrap(),
        u1.try_into().unwrap(),
        u2.try_into().unwrap(),
    );

    let c = u0;

    let a = isize::midpoint(u2 - (2 * u1), u0);

    let b = u1 - u0 - a;

    let n: isize = (steps - TryInto::<isize>::try_into(edge).unwrap())
        / TryInto::<isize>::try_into(columns).unwrap();

    ((a * n.pow(2)) + (b * n) + c).unsigned_abs()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let map = parse_input(input);

        let start = map
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter().enumerate().find_map(|(c, t)| {
                    if matches!(*t, Tile::Garden(true)) {
                        Some((r, c))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        bfs(
            &map,
            (start.0.try_into().unwrap(), start.1.try_into().unwrap()),
            NonZeroUsize::new(64).unwrap(),
        )
        .into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let map = parse_input(input);

        let start = map
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter().enumerate().find_map(|(c, t)| {
                    if matches!(*t, Tile::Garden(true)) {
                        Some((r, c))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        part_2(&map, start).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::{test_example_part_1, test_part_1};

        #[test]
        fn outcome() {
            test_part_1!(3503);
        }

        #[test]
        fn example() {
            test_example_part_1!(2665);
        }
    }

    mod part_2 {

        use advent_of_code_2023::test_part_2;

        #[test]
        fn outcome() {
            test_part_2!(584_211_423_220_706_usize);
        }
    }
}
