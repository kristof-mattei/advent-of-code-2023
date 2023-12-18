use std::convert::Into;

use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(345_015, 42_588_603);

fn parse_lines(input: &str) -> Vec<(usize, usize)> {
    let mut time_and_distances = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();

    let (_, times) = lines[0].split_once(':').expect("Bad input");
    let (_, distances) = lines[1].split_once(':').expect("Bad input");

    for (time, distance) in times
        .split_ascii_whitespace()
        .zip(distances.split_ascii_whitespace())
    {
        time_and_distances.push((
            time.parse().expect("Bad time"),
            distance.parse().expect("Bad distance"),
        ));
    }

    time_and_distances
}

fn parse_lines_with_bad_kerning(input: &str) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<&str>>();

    let (_, times) = lines[0].split_once(':').expect("Bad input");
    let (_, distances) = lines[1].split_once(':').expect("Bad input");

    (
        times.replace(' ', "").parse().expect("Bad time"),
        distances.replace(' ', "").parse().expect("Bad distance"),
    )
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let time_and_distances = parse_lines(input);

        let mut possibilities = 1;

        for (time, distance) in time_and_distances {
            possibilities *= calculate_possibilities(time, distance);
        }

        possibilities.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (time, distance) = parse_lines_with_bad_kerning(input);

        calculate_possibilities(time, distance).into()
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn calculate_possibilities(time: usize, distance: usize) -> usize {
    // solve the quadratic formula of

    // hold = hold time
    // time = time
    // distance = distance (record)

    // to beat the record we need to find a `hold` where
    // hold * (time - hold) > distance

    // to tie the record (that's where we'll start) we need to simplify this:
    // distribute `hold`:
    // (hold * time) - (hold * hold) = distance
    // (swap)
    // - (hold * hold) + (hold * time) = distance
    // (simplify)
    // -(hold ^ 2) + time * hold = distance
    // (simplify)
    // -(hold ^ 2) + time * hold - distance = 0
    // a x    ^ 2  + b      x    + c        = 0

    // The distinct points between the 2 roots are the values where we will have a further distance
    // so we get the discriminant
    // D = b ^ 2 - 4 a c
    // in our case
    // b = time
    // a = -1
    // c = -distance
    // D = (time * time) - (4 * (-1 * -distance))
    // (simplify)
    // D = time * time - 4 * distance

    // roots
    // r1 = ( - b - sqrt(D) ) / 2a
    // r2 = ( - b + sqrt(D) ) / 2a

    // but we want the first whole number above root 1 -> ceil
    let r1 = ((time as f64 + (((time * time) - 4 * distance) as f64).sqrt()) / 2.0).ceil() as usize;

    // and the last whole number beneight root 2 -> floor
    let r2 =
        ((time as f64 - (((time * time) - 4 * distance) as f64).sqrt()) / 2.0).floor() as usize;

    // now we  take the diff of root 1 and root 2, substracting 1 because we need to break the record, not match it
    r1 - r2 - 1
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(345_015, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(288, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(42_588_603, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(71503, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
