use std::{env, fs};

use super::day::Day;

/// Helper function that reads a text file to a string.
/// # Panics
///
/// if the file does not exist or cannot be read
#[must_use]
pub fn read_file(folder: &str, day: &Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{}.txt", day));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
/// # Panics
///
/// if the file does not exist or cannot be read
#[must_use]
pub fn read_file_part(folder: &str, day: &Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{}-{}.txt", day, part));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

#[macro_export]
macro_rules! solution {
    () => {
        $crate::solution!(PartSolution::None, PartSolution::None);
    };
    ($solution_1:expr) => {
        $crate::solution!($solution_1, PartSolution::None);
    };
    ($solution_1:expr, $solution_2:expr) => {
        /// The current day.
        static DAY: once_cell::sync::Lazy<$crate::shared::day::Day> =
            once_cell::sync::Lazy::new(|| {
                use std::path::Path;

                let path = Path::new(file!());
                let file_stem = path
                    .file_stem()
                    .expect("No stem found")
                    .to_str()
                    .expect("Invalid str");

                std::str::FromStr::from_str(file_stem).expect("Could not convert input to Day")
            });

        fn main() {
            // use advent_of_code::template::runner::*;
            let input = $crate::shared::solution::read_file("inputs", &DAY);

            let part_1_expected_solution: PartSolution = PartSolution::from($solution_1);

            let s = Solution {};

            assert_eq!(part_1_expected_solution, s.part_1(&input));

            let part_2_expected_solution: PartSolution = PartSolution::from($solution_2);

            assert_eq!(part_2_expected_solution, s.part_2(&input));
            // run_part(part_one, &input, &DAY, 1);
            // run_part(part_two, &input, &DAY, 2);
        }

        pub struct Solution {}
    };
}
