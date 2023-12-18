use hashbrown::HashMap;

use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(110_677, 90551);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Rock {
    Cube,
    Round,
    None,
}

impl TryFrom<char> for Rock {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Rock::Cube),
            'O' => Ok(Rock::Round),
            '.' => Ok(Rock::None),
            _ => Err("What?"),
        }
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Rock::Cube => '#',
            Rock::Round => 'O',
            Rock::None => '.',
        };

        write!(f, "{}", c)
    }
}

fn parse_input(input: &str) -> Vec<Vec<Rock>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Rock>>()
        })
        .collect::<Vec<_>>()
}

fn roll_north(platform: &mut Vec<Vec<Rock>>) {
    let mut stable = false;
    while !stable {
        stable = true;

        for r in 1..platform.len() {
            for c in 0..platform[r].len() {
                if platform[r][c] == Rock::Round {
                    let mut new_row = r;

                    while new_row > 0 {
                        if platform[new_row - 1][c] == Rock::None {
                            new_row -= 1;
                        } else {
                            break;
                        }
                    }

                    if new_row != r {
                        let (before, after) = platform.split_at_mut(r);

                        std::mem::swap(&mut before[new_row][c], &mut after[0][c]);

                        stable = false;
                    }
                }
            }
        }
    }
}

fn roll_west(platform: &mut Vec<Vec<Rock>>) {
    let mut stable = false;

    while !stable {
        stable = true;

        #[allow(clippy::needless_range_loop)]
        for r in 0..platform.len() {
            for c in 1..platform[r].len() {
                if platform[r][c] == Rock::Round {
                    let mut new_column = c;

                    while new_column > 0 {
                        if platform[r][new_column - 1] == Rock::None {
                            new_column -= 1;
                        } else {
                            break;
                        }
                    }

                    if new_column != c {
                        let (before, after) = platform[r].split_at_mut(c);

                        std::mem::swap(&mut before[new_column], &mut after[0]);

                        stable = false;
                    }
                }
            }
        }
    }
}

fn roll_south(platform: &mut Vec<Vec<Rock>>) {
    let mut stable = false;

    while !stable {
        stable = true;

        for r in (0..platform.len() - 1).rev() {
            for c in 0..platform[r].len() {
                if platform[r][c] == Rock::Round {
                    let mut new_row = r;

                    while new_row < platform.len() - 1 {
                        if platform[new_row + 1][c] == Rock::None {
                            new_row += 1;
                        } else {
                            break;
                        }
                    }

                    if new_row != r {
                        let (before, after) = platform.split_at_mut(new_row);

                        std::mem::swap(&mut before[r][c], &mut after[0][c]);

                        stable = false;
                    }
                }
            }
        }
    }
}

fn roll_east(platform: &mut Vec<Vec<Rock>>) {
    let mut stable = false;

    while !stable {
        stable = true;

        #[allow(clippy::needless_range_loop)]
        for r in 0..platform.len() {
            for c in (0..platform[r].len() - 1).rev() {
                if platform[r][c] == Rock::Round {
                    let mut new_column = c;

                    while new_column < platform[r].len() - 1 {
                        if platform[r][new_column + 1] == Rock::None {
                            new_column += 1;
                        } else {
                            break;
                        }
                    }

                    if new_column != c {
                        let (before, after) = platform[r].split_at_mut(new_column);

                        std::mem::swap(&mut before[c], &mut after[0]);

                        stable = false;
                    }
                }
            }
        }
    }
}

fn count(platform: &[Vec<Rock>]) -> usize {
    let mut total = 0;

    for (row_number, row) in platform
        .iter()
        .enumerate()
        .map(|(i, row)| (platform.len() - i, row))
    {
        for column in row {
            if matches!(column, Rock::Round) {
                total += row_number;
            }
        }
    }

    total
}

fn roll_north_and_count(mut platform: Vec<Vec<Rock>>) -> usize {
    roll_north(&mut platform);

    count(&platform)
}

fn roll_all(platform: &mut Vec<Vec<Rock>>) {
    roll_north(platform);
    roll_west(platform);
    roll_south(platform);
    roll_east(platform);
}

fn as_far_as_we_can(iterations: usize, platform: &mut Vec<Vec<Rock>>) -> usize {
    let mut cache = HashMap::<Vec<Vec<Rock>>, usize>::new();

    for turn in 1..=iterations {
        if let Some(previous_turn) = cache.get(platform) {
            // we saw this configuration at `previous_turn`
            // so we now try to advance as far as possible before exceeding `iterations`

            // how many turns does it take to repeat ourselves
            let period = turn - previous_turn;

            // given we are at turn `turn`, how many times does our `period` fit
            return turn + (((iterations - turn) / period) * period);
        }

        let original = platform.clone();

        roll_all(platform);

        cache.insert(original, turn);
    }

    println!("No period found, meaning we actually ran through all iterations...");

    iterations
}

fn roll_and_count(mut platform: Vec<Vec<Rock>>) -> usize {
    const TOTAL_ITERATIONS: usize = 1_000_000_000;

    // we need to establish the amount of turns it takes to go from form A, apply x rolls, and back to A.
    // we also don't know whether this is at the starting position

    let start = as_far_as_we_can(TOTAL_ITERATIONS, &mut platform);

    for _ in start..=TOTAL_ITERATIONS {
        roll_all(&mut platform);
    }

    count(&platform)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        roll_north_and_count(parsed).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        roll_and_count(parsed).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(110_677, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(136, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(90551, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(64, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
