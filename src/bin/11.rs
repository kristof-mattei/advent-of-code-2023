use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(10_276_166, 598_693_078_798_usize);

#[derive(PartialEq, Eq)]
enum What {
    Void,
    Galaxy,
}

struct Galaxy {
    map: Vec<Vec<What>>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

fn parse_input(input: &str) -> Galaxy {
    let mut map = vec![];
    for line in input.lines() {
        let mut row = vec![];

        for c in line.chars() {
            let whats_here = match c {
                '.' => What::Void,
                '#' => What::Galaxy,
                _ => panic!("Unrecognized object. UFO?"),
            };

            row.push(whats_here);
        }

        map.push(row);
    }

    let rows = map.len();
    let columns = map[0].len();

    let rows_with_galaxies: Vec<usize> = (0..rows)
        .fold(vec![false; map.len()], |mut acc, row_index| {
            for column_index in 0..columns {
                if map[row_index][column_index] == What::Galaxy {
                    acc[row_index] = true;
                    break;
                }
            }

            acc
        })
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, has_galaxies)| {
                if has_galaxies {
                    None
                } else {
                    Some(index)
                }
            },
        )
        .collect();

    let columns_with_galaxies: Vec<usize> = (0..columns)
        .fold(vec![false; map.len()], |mut acc, column_index| {
            for row in map.iter().take(rows) {
                if row[column_index] == What::Galaxy {
                    acc[column_index] = true;
                    break;
                }
            }

            acc
        })
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, has_galaxies)| {
                if has_galaxies {
                    None
                } else {
                    Some(index)
                }
            },
        )
        .collect();

    Galaxy {
        map,
        empty_rows: rows_with_galaxies,
        empty_columns: columns_with_galaxies,
    }
}

fn sum_lengths_between_galaxies(galaxy: &Galaxy, multiplier: usize) -> usize {
    let mut galaxies = galaxy
        .map
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(column_index, column)| {
                    (column == &What::Galaxy).then_some((row_index, column_index))
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();

    let mut combinations: Vec<((usize, usize), (usize, usize))> = vec![];

    while let Some(galaxy) = galaxies.pop() {
        for g in &galaxies {
            combinations.push((galaxy, *g));
        }
    }

    let mut sum_of_distances = 0;

    for ((r1, c1), (r2, c2)) in combinations {
        let min_row = r1.min(r2);
        let max_row = r1.max(r2);

        let min_column = c1.min(c2);
        let max_column = c1.max(c2);

        let row_range = min_row..max_row;
        let column_range = min_column..max_column;

        let rows_to_add = galaxy
            .empty_rows
            .iter()
            .filter(|r| row_range.contains(r))
            .count();
        let columns_to_add = galaxy
            .empty_columns
            .iter()
            .filter(|r| column_range.contains(r))
            .count();

        let distance = ((max_row - min_row) - rows_to_add + (rows_to_add * multiplier))
            + ((max_column - min_column) - columns_to_add + (columns_to_add * multiplier));

        sum_of_distances += distance;
    }

    sum_of_distances
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let galaxy: Galaxy = parse_input(input);

        sum_lengths_between_galaxies(&galaxy, 2).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let galaxy: Galaxy = parse_input(input);

        sum_lengths_between_galaxies(&galaxy, 1_000_000).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{parse_input, Galaxy, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(10_276_166, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_empty_rows_columns() {
            let input = read_file("examples", &DAY);

            let galaxy: Galaxy = parse_input(&input);

            assert_eq!(vec![3, 7], galaxy.empty_rows);
            assert_eq!(vec![2, 5, 8], galaxy.empty_columns);
        }

        #[test]
        fn example() {
            assert_eq!(374, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{parse_input, sum_lengths_between_galaxies, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                598_693_078_798_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_1() {
            let galaxy = parse_input(&read_file("examples", &DAY));

            assert_eq!(1030, sum_lengths_between_galaxies(&galaxy, 10));
        }

        #[test]
        fn example_2() {
            let galaxy = parse_input(&read_file("examples", &DAY));

            assert_eq!(8410, sum_lengths_between_galaxies(&galaxy, 100));
        }
    }
}
