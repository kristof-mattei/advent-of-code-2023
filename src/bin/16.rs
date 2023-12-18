use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(6902, 7697);

#[derive(Clone, Copy)]
enum Traveling {
    Up,
    Right,
    Down,
    Left,
}

struct Tile {
    traveled_from: [bool; 4],
    mirror: Mirror,
}

enum Mirror {
    Dash,
    Pipe,
    Slash,
    BackwardsSlash,
    Ground,
}

impl std::fmt::Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Mirror::Dash => '-',
            Mirror::Pipe => '|',
            Mirror::Slash => '/',
            Mirror::BackwardsSlash => '\\',
            Mirror::Ground => '.',
        };

        write!(f, "{}", c)
    }
}
enum Ummm {
    Split((Traveling, Traveling)),
    Single(Traveling),
}

fn apply_direction_to_coordinates(
    field: &[Vec<Tile>],
    row_index: usize,
    column_index: usize,
    direction: Traveling,
) -> Option<(usize, usize)> {
    let left = column_index.checked_sub(1);

    let right = {
        let next_column_index = column_index + 1;
        (next_column_index < field[row_index].len()).then_some(next_column_index)
    };

    let up = row_index.checked_sub(1);

    let down = {
        let next_row_index = row_index + 1;
        (next_row_index < field.len()).then_some(next_row_index)
    };

    match direction {
        Traveling::Up => up.map(|u| (u, column_index)),
        Traveling::Right => right.map(|r| (row_index, r)),
        Traveling::Down => down.map(|d| (d, column_index)),
        Traveling::Left => left.map(|l| (row_index, l)),
    }
}

impl Tile {
    fn get_next_direction(&self, energy_is_traveling: Traveling) -> Ummm {
        match (energy_is_traveling, &self.mirror) {
            (Traveling::Up, Mirror::Slash)
            | (Traveling::Right, Mirror::Dash | Mirror::Ground)
            | (Traveling::Down, Mirror::BackwardsSlash) => Ummm::Single(Traveling::Right),
            (Traveling::Left, Mirror::Dash | Mirror::Ground)
            | (Traveling::Down, Mirror::Slash)
            | (Traveling::Up, Mirror::BackwardsSlash) => Ummm::Single(Traveling::Left),
            (Traveling::Up, Mirror::Pipe | Mirror::Ground)
            | (Traveling::Right, Mirror::Slash)
            | (Traveling::Left, Mirror::BackwardsSlash) => Ummm::Single(Traveling::Up),
            (Traveling::Left | Traveling::Right, Mirror::Pipe) => {
                Ummm::Split((Traveling::Up, Traveling::Down))
            },
            (Traveling::Up | Traveling::Down, Mirror::Dash) => {
                Ummm::Split((Traveling::Left, Traveling::Right))
            },
            (Traveling::Down, Mirror::Pipe | Mirror::Ground)
            | (Traveling::Left, Mirror::Slash)
            | (Traveling::Right, Mirror::BackwardsSlash) => Ummm::Single(Traveling::Down),
        }
    }

    fn set_seen_or_set(&mut self, from: Traveling) -> bool {
        // for Dash & Pipe we can reduce the work by treating opposite directions as the same
        // as they have the same outcome
        let from = match (&self.mirror, from) {
            (Mirror::Dash, Traveling::Up | Traveling::Down) => Traveling::Up,
            (Mirror::Pipe, Traveling::Left | Traveling::Right) => Traveling::Left,
            (_, t) => t,
        };

        if self.traveled_from[from as usize] {
            true
        } else {
            self.traveled_from[from as usize] = true;
            false
        }
    }

    fn reset(&mut self) {
        self.traveled_from = [false; 4];
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.try_into().map(|m| Self {
            traveled_from: [false; 4],
            mirror: m,
        })
    }
}

impl TryFrom<char> for Mirror {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Mirror::Pipe),
            '-' => Ok(Mirror::Dash),
            '/' => Ok(Mirror::Slash),
            '\\' => Ok(Mirror::BackwardsSlash),
            '.' => Ok(Mirror::Ground),
            _ => Err("Invalid character"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn send_light(
    parsed: &mut [Vec<Tile>],
    mut row_index: usize,
    mut column_index: usize,
    mut direction: Traveling,
) {
    loop {
        if parsed[row_index][column_index].set_seen_or_set(direction) {
            break;
        }

        match parsed[row_index][column_index].get_next_direction(direction) {
            Ummm::Split((d1, d2)) => {
                if let Some(row_column_index) =
                    apply_direction_to_coordinates(parsed, row_index, column_index, d1)
                {
                    send_light(parsed, row_column_index.0, row_column_index.1, d1);
                }

                if let Some(row_column_index) =
                    apply_direction_to_coordinates(parsed, row_index, column_index, d2)
                {
                    send_light(parsed, row_column_index.0, row_column_index.1, d2);
                }

                break;
            },
            Ummm::Single(d) => {
                direction = d;
            },
        }

        if let Some(row_column_index) =
            apply_direction_to_coordinates(parsed, row_index, column_index, direction)
        {
            (row_index, column_index) = row_column_index;
        } else {
            break;
        }
    }
}

fn count_energized(parsed: &[Vec<Tile>]) -> usize {
    let mut count: usize = 0;
    for row in parsed {
        for tile in row {
            if tile.traveled_from.iter().any(|a| *a) {
                count += 1;
            }
        }
    }
    count
}

fn reset(parsed: &mut Vec<Vec<Tile>>) {
    for row in parsed {
        for tile in row {
            tile.reset();
        }
    }
}

fn find_highest_entrypoint(mut parsed: Vec<Vec<Tile>>) -> usize {
    let mut highest = usize::MIN;

    {
        for row_index in 0..parsed.len() {
            // now we do column 0 going right

            // we start from (0,-1), going down, to (0,0)
            send_light(&mut parsed, row_index, 0, Traveling::Right);
            highest = usize::max(highest, count_energized(&parsed));

            reset(&mut parsed);
        }
    }

    {
        let last_column_index = parsed[0].len() - 1;
        for row_index in 0..parsed.len() {
            // now we do column 0 going right

            // we start from (0,-1), going down, to (0,0)
            send_light(&mut parsed, row_index, last_column_index, Traveling::Left);
            highest = usize::max(highest, count_energized(&parsed));

            reset(&mut parsed);
        }
    }

    {
        for column_index in 0..parsed[0].len() {
            // now we do column 0 going right

            // we start from (0,-1), going down, to (0,0)
            send_light(&mut parsed, 0, column_index, Traveling::Down);
            highest = usize::max(highest, count_energized(&parsed));

            reset(&mut parsed);
        }
    }

    {
        let last_row_index = parsed[0].len() - 1;
        for column_index in 0..parsed[last_row_index].len() {
            // now we do column 0 going right

            // we start from (0,-1), going down, to (0,0)
            send_light(&mut parsed, last_row_index, column_index, Traveling::Up);
            highest = usize::max(highest, count_energized(&parsed));

            reset(&mut parsed);
        }
    }

    highest
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut parsed = parse_input(input);

        // we start from (0,-1), going right, to (0,0)
        send_light(&mut parsed, 0, 0, Traveling::Right);

        count_energized(&parsed).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        find_highest_entrypoint(parsed).into()
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
            assert_eq!(6902, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(46, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(7697, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(51, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
