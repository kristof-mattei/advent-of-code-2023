use std::str::FromStr;

use advent_of_code_2023::shared::{PartSolution, Parts};
use z3::ast::Int;
use z3::{SatResult, Solver};

advent_of_code_2023::solution!(490, 6726);

struct Hailstone {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

impl FromStr for Hailstone {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = line.split_once('@').ok_or("Missing @ separator")?;

        let [px, py, pz] = parse_vector(position)?;
        let [vx, vy, vz] = parse_vector(velocity)?;

        Ok(Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        })
    }
}

fn parse_vector(text: &str) -> Result<[i128; 3], &'static str> {
    let components = text
        .split(',')
        .map(|component| component.trim().parse())
        .collect::<Result<Vec<i128>, _>>()
        .map_err(|_| "could not parse component")?;

    components
        .try_into()
        .map_err(|_| "invalid component length")
}

fn parse_input(input: &str) -> Result<Vec<Hailstone>, &'static str> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::parse)
        .collect()
}

/// Counts the pairs whose future XY paths cross inside `min..=max` on both axes.
fn count_crossings(hailstones: &[Hailstone], min: i128, max: i128) -> usize {
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(index, first)| {
            hailstones[index + 1..]
                .iter()
                .map(move |second| (first, second))
        })
        .filter(|&(first, second)| paths_cross_within(first, second, min, max))
        .count()
}

fn paths_cross_within(first: &Hailstone, second: &Hailstone, min: i128, max: i128) -> bool {
    // Cramer's rule on `first.px + first.vx * first_time = second.px + second.vx * second_time` and its y counterpart
    let determinant = second.vx * first.vy - first.vx * second.vy;

    if determinant == 0 {
        // parallel paths
        return false;
    }

    let offset_x = second.px - first.px;
    let offset_y = second.py - first.py;

    // a positive `determinant` gives each time the sign of its numerator
    let determinant_sign = determinant.signum();
    let determinant = determinant.abs();

    // each time equals its numerator divided by `determinant`, and that division is never performed
    let first_time_numerator = (second.vx * offset_y - second.vy * offset_x) * determinant_sign;
    let second_time_numerator = (first.vx * offset_y - first.vy * offset_x) * determinant_sign;

    if first_time_numerator < 0 || second_time_numerator < 0 {
        // the crossing lies in the past of at least one hailstone
        return false;
    }

    // the area and the crossing are both scaled by `determinant`, which keeps the comparison in integers
    let scaled_area = min * determinant..=max * determinant;
    let scaled_crossing_x = first.px * determinant + first.vx * first_time_numerator;
    let scaled_crossing_y = first.py * determinant + first.vy * first_time_numerator;

    scaled_area.contains(&scaled_crossing_x) && scaled_area.contains(&scaled_crossing_y)
}

/// Finds the integer rock throw that hits every hailstone and returns the sum of its starting coordinates.
fn rock_coordinate_sum(hailstones: &[Hailstone]) -> Option<i64> {
    #![expect(clippy::similar_names, reason = "")]

    let solver = Solver::new();

    let [rock_px, rock_py, rock_pz, rock_vx, rock_vy, rock_vz] = [
        "rock_px", "rock_py", "rock_pz", "rock_vx", "rock_vy", "rock_vz",
    ]
    .map(Int::new_const);

    // three hailstones with pairwise skew paths determine the six unknowns
    for (index, hailstone) in hailstones.iter().take(3).enumerate() {
        let time = Int::new_const(format!("time_{}", index));
        solver.assert(time.ge(0));

        let axes = [
            (&rock_px, &rock_vx, hailstone.px, hailstone.vx),
            (&rock_py, &rock_vy, hailstone.py, hailstone.vy),
            (&rock_pz, &rock_vz, hailstone.pz, hailstone.vz),
        ];

        for (rock_position, rock_velocity, hailstone_position, hailstone_velocity) in axes {
            let rock_at_time = rock_position + rock_velocity * &time;
            let hailstone_at_time = &time * hailstone_velocity + hailstone_position;

            solver.assert(rock_at_time.eq(hailstone_at_time));
        }
    }

    if !matches!(solver.check(), SatResult::Sat) {
        return None;
    }

    let model = solver.get_model()?;

    model
        .eval(&(&rock_px + &rock_py + &rock_pz), true)?
        .as_i64()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        const TEST_AREA_MIN: i128 = 200_000_000_000_000;
        const TEST_AREA_MAX: i128 = 400_000_000_000_000;

        let hailstones = parse_input(input).unwrap();

        let collisions = count_crossings(&hailstones, TEST_AREA_MIN, TEST_AREA_MAX);

        PartSolution::USize(collisions)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let hailstones = parse_input(input).unwrap();

        let result = rock_coordinate_sum(&hailstones).unwrap();

        PartSolution::I64(result)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::test_part_1;
        use pretty_assertions::assert_eq;

        use crate::{DAY, count_crossings, parse_input};

        #[test]
        fn outcome() {
            test_part_1!(21679);
        }

        #[test]
        fn example() {
            let input = read_file("examples", &DAY);

            let hailstones = parse_input(&input).unwrap();

            let collisions = count_crossings(&hailstones, 7, 27);

            assert_eq!(collisions, 2);
        }
    }

    mod part_2 {
        use advent_of_code_2023::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(566_914_635_762_564_i64);
        }

        #[test]
        fn example() {
            test_example_part_2!(47);
        }
    }
}
