use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(54159, 53866);

fn first_09_digit(line: &str) -> Option<(usize, u32)> {
    line.chars()
        .enumerate()
        .find_map(|(index, c)| c.to_digit(10).map(|d| (index, d)))
}

fn last_09_digit(line: &str) -> Option<(usize, u32)> {
    line.chars()
        .rev()
        .enumerate()
        .find_map(|(index, c)| c.to_digit(10).map(|d| (line.len() - index - 1, d)))
}

fn calculate_total_calibration_value_part_1(lines: &str) -> u32 {
    let mut total = 0;
    for line in lines.lines() {
        let (_, first_number) = first_09_digit(line).expect("No number found");
        let (_, last_number) = last_09_digit(line).expect("No number found");

        total += first_number * 10 + last_number;
    }

    total
}

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_first_last(
    left: Option<(usize, u32)>,
    right: Option<(usize, u32)>,
    cmp: fn(usize, usize) -> bool,
) -> Option<u32> {
    match (left, right) {
        (None, None) => None,
        (None, Some((_, right))) => Some(right),
        (Some((_, left)), None) => Some(left),
        (Some((left_index, left_number)), Some((right_index, right_number))) => {
            if cmp(left_index, right_index) {
                Some(left_number)
            } else {
                Some(right_number)
            }
        },
    }
}

fn calculate_total_calibration_value_part_2(lines: &str) -> u32 {
    let mut total = 0;
    for line in lines.lines() {
        let first_09_digit = first_09_digit(line);
        let last_09_digit = last_09_digit(line);

        let first_word_digit =
            first_word_digit(line, first_09_digit.map_or(line.len(), |(index, _)| index));
        let last_word_digit = last_word_digit(line, last_09_digit.map_or(0, |(index, _)| index));

        let first =
            get_first_last(first_09_digit, first_word_digit, |l, r| l < r).expect("No first found");
        let last =
            get_first_last(last_09_digit, last_word_digit, |l, r| l > r).expect("No last found");

        total += last;
        total += first * 10;
    }
    total
}

fn first_word_digit(line: &str, first_09_digit_index: usize) -> Option<(usize, u32)> {
    for i in 0..first_09_digit_index {
        for (word_index, word) in NUMBER_WORDS.iter().enumerate() {
            if line[i..].starts_with(word) {
                return Some((
                    i,
                    u32::try_from(word_index).expect("index doesn't fit u32") + 1,
                ));
            }
        }
    }

    None
}

fn last_word_digit(line: &str, last_09_digit_index: usize) -> Option<(usize, u32)> {
    for i in (last_09_digit_index..line.len()).rev() {
        for (word_index, word) in NUMBER_WORDS.iter().enumerate() {
            if line[i..].starts_with(word) {
                return Some((
                    i,
                    u32::try_from(word_index).expect("index doesn't fit u32") + 1,
                ));
            }
        }
    }

    None
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        calculate_total_calibration_value_part_1(input).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        calculate_total_calibration_value_part_2(input).into()
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
            assert_eq!(54159, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(
                142,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};
        use advent_of_code_2023::shared::Parts;

        use crate::{first_word_digit, last_word_digit, Solution, DAY};

        #[test]
        fn example() {
            assert_eq!(
                281,
                Solution {}.part_2(&read_file_part("examples", &DAY, 2))
            );
        }

        #[test]
        fn outcome() {
            assert_eq!(53866, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn first_word_with_index() {
            let line = "one two three four five six seven eight nine";
            let (index, number) = first_word_digit(line, line.len()).unwrap();

            assert_eq!(index, 0);
            assert_eq!(number, 1);
        }

        #[test]
        fn last_word_with_index() {
            let line = "one two three four five six seven eight nine";
            let (index, number) = last_word_digit(line, 0).unwrap();

            assert_eq!(index, 40);
            assert_eq!(number, 9);
        }
    }
}
