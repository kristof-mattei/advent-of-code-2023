use std::sync::LazyLock;

use advent_of_code_2023::shared::{PartSolution, Parts};
use regex::{Regex, RegexBuilder};

advent_of_code_2023::solution!(40714, 129_849_166_997_110_usize);

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let mut builder =
        RegexBuilder::new(r"(?<direction>[URDL]) (?<count>\d+) \(#(?<color>(?:[0-9A-F]{2}){3})\)");

    builder.case_insensitive(true);

    builder.build().unwrap()
});

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Instruction {
    part1: InstructionPart,
    part2: InstructionPart,
}

struct InstructionPart {
    direction: Direction,
    count: usize,
}

#[derive(Clone, Copy, Debug)]
struct Coordinates {
    row_index: usize,
    column_index: usize,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("Invalid character"),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            0 => Ok(Direction::Right),
            _ => Err("Invalid character"),
        }
    }
}

fn parse_lines(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.lines() {
        let captures = REGEX.captures(line).unwrap();

        let part1_direction: Direction = captures
            .name("direction")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .map(|c| c.try_into().expect("Couldn't convert to direction"))
            .expect("No character found?");

        let part1_count = captures
            .name("count")
            .unwrap()
            .as_str()
            .parse()
            .expect("Invalid count");

        let part2 = captures.name("color").unwrap().as_str();

        let part2_direction: Direction = u8::from_str_radix(&part2[5..], 16)
            .unwrap()
            .try_into()
            .unwrap();

        let part2_count = usize::from_str_radix(&part2[0..5], 16).unwrap();

        // let rgb = (0..part2.len())
        //     .step_by(2)
        //     .map(|i| u8::from_str_radix(&part2[i..i + 2], 16).unwrap())
        //     .collect::<Vec<_>>()
        //     .try_into()
        //     .unwrap();

        instructions.push(Instruction {
            part1: InstructionPart {
                direction: part1_direction,
                count: part1_count,
            },
            part2: InstructionPart {
                direction: part2_direction,
                count: part2_count,
            },
        });
    }

    instructions
}

fn get_lagoon_specs(instructions: &[&InstructionPart]) -> Coordinates {
    let mut rows = 0isize;
    let mut min_row = isize::MAX;
    let mut max_row = isize::MIN;
    let mut columns = 0isize;
    let mut min_column = isize::MAX;
    let mut max_column = isize::MIN;

    for instruction in instructions {
        let count: isize = instruction.count.try_into().unwrap();

        match instruction.direction {
            Direction::Up => rows -= count,
            Direction::Right => columns += count,
            Direction::Down => rows += count,
            Direction::Left => columns -= count,
        }

        min_row = rows.min(min_row);
        max_row = rows.max(max_row);
        min_column = columns.min(min_column);
        max_column = columns.max(max_column);
    }

    Coordinates {
        row_index: min_row.abs().try_into().unwrap(),
        column_index: min_column.abs().try_into().unwrap(),
    }
}

pub(crate) fn build_coordinates(
    start_coordinates: Coordinates,
    instructions: &[&InstructionPart],
) -> (Vec<Coordinates>, usize) {
    let mut coordinates = vec![start_coordinates];

    let mut perimeter = 0;

    let mut current = start_coordinates;

    for instruction in instructions {
        perimeter += instruction.count;
        match instruction.direction {
            Direction::Right => current.column_index += instruction.count,
            Direction::Left => current.column_index -= instruction.count,
            Direction::Up => current.row_index -= instruction.count,
            Direction::Down => current.row_index += instruction.count,
        }

        coordinates.push(current);
    }

    (coordinates, perimeter)
}

pub(crate) fn shoelace(coordinates: &[Coordinates], perimeter: usize) -> usize {
    let mut size: isize = 0;

    for coord in coordinates.windows(2) {
        // [ a b ]
        // [ c d ]

        let a: isize = coord[0].row_index.try_into().unwrap();
        let b: isize = coord[0].column_index.try_into().unwrap();
        let c: isize = coord[1].row_index.try_into().unwrap();
        let d: isize = coord[1].column_index.try_into().unwrap();

        size += (a * d) - (b * c);
    }

    TryInto::<usize>::try_into((size / 2).abs()).unwrap() + (perimeter / 2) + 1
}

fn dig_pool(instructions: &[&InstructionPart]) -> usize {
    let start_coordinates = get_lagoon_specs(instructions);

    let (coordinates, edges) = build_coordinates(start_coordinates, instructions);

    shoelace(&coordinates, edges)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let instructions = parsed.iter().map(|i| &i.part1).collect::<Vec<_>>();

        dig_pool(&instructions).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let instructions = parsed.iter().map(|i| &i.part2).collect::<Vec<_>>();

        dig_pool(&instructions).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::Parts;
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(40714, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                62,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                100,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::Parts;
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                129_849_166_997_110_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                952_408_144_115usize,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 1))
            );
        }
    }
}
