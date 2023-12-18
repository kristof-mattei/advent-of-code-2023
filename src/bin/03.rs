use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(527_364, 79_026_871);

enum Cell {
    Number(u32),
    Symbol(char),
    Empty(),
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty()),
            '0'..='9' => {
                let n = u32::try_from(c).unwrap() - 48;

                Ok(Cell::Number(n))
            },
            _ => Ok(Cell::Symbol(c)),
        }
    }
}

fn parse_lines(lines: &str) -> Schematic {
    let mut parsed = Vec::new();

    for line in lines.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(Cell::try_from(c).unwrap());
        }

        parsed.push(row);
    }

    parsed
}

type Schematic = Vec<Vec<Cell>>;

fn find_number_at(row: &[Cell], column_index: usize) -> Option<u32> {
    // first we check if we are at the start of our number
    // on the left it must either be Empty or Symbol, just not a number
    // this avoids cases like
    // 123
    //  *
    // 456
    // and we landed on the (0,1). We should then ignore the number as it will be covered by the left-top check

    // are we on a number? If not, return
    if !matches!(row[column_index], Cell::Number(_)) {
        return None;
    }

    // we're on a number, let's make sure we're on the start on the number
    let mut start = column_index;

    while start > 0 && matches!(row[start - 1], Cell::Number(_)) {
        start -= 1;
    }

    // okay we're at the start of a number
    let mut number = 0;

    for c in &row[start..] {
        if let Cell::Number(n) = c {
            number = number * 10 + n;
        } else {
            break;
        }
    }

    Some(number)
}

fn get_numbers_around(schematic: &Schematic, row_index: usize, column_index: usize) -> Vec<u32> {
    let mut numbers_around = vec![];

    let row_above = row_index.checked_sub(1).and_then(|ri| schematic.get(ri));
    let row_below = schematic.get(row_index + 1);

    let column_index_left = column_index.checked_sub(1);
    let column_index_right =
        (column_index + 1 < schematic[row_index].len()).then_some(column_index + 1);

    // left
    if let Some(left_index) = column_index_left {
        if let Some(n) = find_number_at(&schematic[row_index], left_index) {
            numbers_around.push(n);
        }
    }

    // right
    if let Some(right_index) = column_index_right {
        if let Some(n) = find_number_at(&schematic[row_index], right_index) {
            numbers_around.push(n);
        }
    }

    if let Some(row) = row_above {
        // if we have a number right above us then that is the only number that can touch us
        // as we go as far left/right as we can
        if let Some(n) = find_number_at(row, column_index) {
            numbers_around.push(n);
        } else {
            // left-top
            if let Some(left_index) = column_index_left {
                if let Some(n) = find_number_at(row, left_index) {
                    numbers_around.push(n);
                }
            }

            // right-top
            if let Some(right_index) = column_index_right {
                if let Some(n) = find_number_at(row, right_index) {
                    numbers_around.push(n);
                }
            }
        }
    }

    if let Some(row) = row_below {
        if let Some(n) = find_number_at(row, column_index) {
            numbers_around.push(n);
        } else {
            // left-bottom
            if let Some(left_index) = column_index_left {
                if let Some(n) = find_number_at(row, left_index) {
                    numbers_around.push(n);
                }
            }

            // right-bottom
            if let Some(right_index) = column_index_right {
                if let Some(n) = find_number_at(row, right_index) {
                    numbers_around.push(n);
                }
            }
        }
    }

    numbers_around
}

fn multiply_gear_numbers(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for (row_index, row) in schematic.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if matches!(column, Cell::Symbol('*')) {
                let gear_ratios = get_numbers_around(schematic, row_index, column_index);

                if gear_ratios.len() > 1 {
                    sum += gear_ratios.iter().product::<u32>();
                }
            }
        }
    }

    sum
}

fn sum_all_part_numbers(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for (row_index, row) in schematic.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if matches!(column, Cell::Symbol(_)) {
                sum += get_numbers_around(schematic, row_index, column_index)
                    .iter()
                    .sum::<u32>();
            }
        }
    }

    sum
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        sum_all_part_numbers(&parsed).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        multiply_gear_numbers(&parsed).into()
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
            assert_eq!(527_364, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(4361, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(79_026_871, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(467_835, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
