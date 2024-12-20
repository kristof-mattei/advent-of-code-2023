use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!();

fn parse_input(input: &str) -> &str {
    input
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let _t = parse_input(input);

        None.into()
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        None.into()
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]

        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]

        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]

        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]

        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
