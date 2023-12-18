use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(1_772_145_754, 867);

fn parse_lines(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn calculate_future_r(history: &[i32]) -> i32 {
    let mut differences = Vec::with_capacity(history.len() - 1);

    for window in history.windows(2) {
        differences.push(window[1] - window[0]);
    }

    let last = history.last().unwrap();

    if differences.iter().all(|d| d == &0) {
        *last
    } else {
        *last + calculate_future_r(&differences)
    }
}

fn calculate_sum_of_futures(histories: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for history in histories {
        sum += calculate_future_r(history);
    }
    sum
}

fn calculate_history_r(history: &[i32]) -> i32 {
    let mut differences = Vec::with_capacity(history.len() - 1);

    for window in history.windows(2) {
        differences.push(window[1] - window[0]);
    }

    let first = history.first().unwrap();

    if differences.iter().all(|d| d == &0) {
        *first
    } else {
        *first - calculate_history_r(&differences)
    }
}

fn calculate_sum_of_histories(histories: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for history in histories {
        sum += calculate_history_r(history);
    }
    sum
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        calculate_sum_of_futures(&parsed).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        calculate_sum_of_histories(&parsed).into()
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
            assert_eq!(
                1_772_145_754,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(114, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(867, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(2, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
