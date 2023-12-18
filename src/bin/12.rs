use hashbrown::HashMap;
use std::fmt::Debug;

use advent_of_code_2023::shared::{PartSolution, Parts};

advent_of_code_2023::solution!(7191, 6_512_849_198_636u64);

#[derive(Clone, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damaged => write!(f, "#"),
            Self::Operational => write!(f, "."),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for State {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(State::Damaged),
            '.' => Ok(State::Operational),
            '?' => Ok(State::Unknown),
            _ => Err("Unknown state"),
        }
    }
}

struct Conditions {
    rows: Vec<(Vec<State>, Vec<usize>)>,
}

fn parse_input(input: &str) -> Conditions {
    let rows = input
        .lines()
        .map(|l| {
            let (unparsed_springs, unparsed_contiguous_springs) =
                l.split_once(' ').expect("Invalid line");

            let springs: Vec<State> = unparsed_springs
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<State>>();

            let contiguous_springs: Vec<usize> = unparsed_contiguous_springs
                .split(',')
                .map(|v| v.parse().expect("Invalid number"))
                .collect();

            (springs, contiguous_springs)
        })
        .collect();

    Conditions { rows }
}

fn arrangements(data: &[State], groups: &[usize]) -> usize {
    let mut cache = HashMap::default();

    dfs(&mut cache, data, groups, 0)
}

fn dfs(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    springs: &[State],
    damaged_groups: &[usize],
    damaged_group_progress: usize,
) -> usize {
    let Some(current_spring) = springs.first() else {
        // out of springs
        match damaged_groups {
            [] => {
                // and we're no longer expecting damaged springs
                return 1;
            },
            &[size] if size == damaged_group_progress => {
                // previous spring was a damaged one, but the group is full
                return 1;
            },
            _ => {
                // we're still expecting damaged springs, either in a new group
                // or in the current group
                // but we have no more springs to check
                return 0;
            },
        }
    };

    let Some(damaged_group_size) = damaged_groups.first().copied() else {
        if springs.iter().any(|x| matches!(x, State::Damaged)) {
            return 0;
        }

        return 1;
    };

    match current_spring {
        State::Operational => {
            // we're not chasing down a broken spring, just move on to the next
            if damaged_group_progress == 0 {
                return dfs(cache, &springs[1..], damaged_groups, 0);
            }

            // we are chasing down damaged springs (as current_group_progress > 0)
            // check if we have hit enough damaged / unknowns
            if damaged_group_progress < damaged_group_size {
                // no, we need more damaged, unknowns, so this is invalid
                return 0;
            }

            // previous spring was damaged, current one is operational, and we've hit the damaged_group_progress
            // move to next group & reset progress
            dfs(cache, &springs[1..], &damaged_groups[1..], 0)
        },
        State::Damaged => {
            // we already found x damaged springs for the damaged group
            // now we have one too many, so this one is bad
            if damaged_group_progress == damaged_group_size {
                return 0;
            }

            // we're still expecting damaged springs, increment progress and go
            dfs(
                cache,
                &springs[1..],
                damaged_groups,
                damaged_group_progress + 1,
            )
        },

        State::Unknown => {
            if let Some(&answer) =
                cache.get(&(springs.len(), damaged_groups.len(), damaged_group_progress))
            {
                return answer;
            }

            let mut ways = 0;

            // here we split

            // we're currently not chasing down damaged cells, so we
            // skip this one and go on our marry way
            // notice that in the next block we do try to treat this one as damaged
            if damaged_group_progress == 0 {
                ways += dfs(cache, &springs[1..], damaged_groups, 0);
            }

            // 0 is included here
            // so eitehr we're chasing down a damaged collection
            // or not
            // either way, consider this one damaged
            if damaged_group_progress < damaged_group_size {
                ways += dfs(
                    cache,
                    &springs[1..],
                    damaged_groups,
                    damaged_group_progress + 1,
                );
            }

            // this is the same as in the Damaged section
            // check to see if we hit our target
            // if we did, then this one MUST be Operational (spacing needed)
            // which means we can move on to the next spring and group
            if damaged_group_progress == damaged_group_size {
                ways += dfs(cache, &springs[1..], &damaged_groups[1..], 0);
            }

            cache.insert(
                (springs.len(), damaged_groups.len(), damaged_group_progress),
                ways,
            );

            ways
        },
    }
}

fn sum_all_possibilities(conditions: &Conditions) -> usize {
    let mut sum = 0;

    for (places, groups) in &conditions.rows {
        sum += arrangements(places, groups);
    }

    sum
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let conditions = parse_input(input);

        sum_all_possibilities(&conditions).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut conditions = parse_input(input);

        for (states, damaged) in &mut conditions.rows {
            let mut states_repeated = states.clone();

            for _ in 1..5 {
                states_repeated.push(State::Unknown);

                states_repeated.append(&mut (states.clone()));
            }

            *states = states_repeated;

            let mut damaged_repeated = vec![];
            for _ in 0..5 {
                damaged_repeated.append(&mut (damaged.clone()));
            }

            *damaged = damaged_repeated;
        }

        sum_all_possibilities(&conditions).into()
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
            assert_eq!(7191, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_single_1() {
            assert_eq!(1, (Solution {}).part_1("???.### 1,1,3"));
        }

        #[test]
        fn example_single_2() {
            assert_eq!(4, (Solution {}).part_1(".??..??...?##. 1,1,3"));
        }

        #[test]
        fn example_single_3() {
            assert_eq!(1, (Solution {}).part_1("?#?#?#?#?#?#?#? 1,3,1,6"));
        }

        #[test]
        fn example_single_4() {
            assert_eq!(1, (Solution {}).part_1("????.#...#... 4,1,1"));
        }

        #[test]
        fn example_single_5() {
            assert_eq!(4, (Solution {}).part_1("????.######..#####. 1,6,5"));
        }

        #[test]
        fn example_single_6() {
            assert_eq!(10, (Solution {}).part_1("?###???????? 3,2,1"));
        }

        #[test]
        fn example() {
            assert_eq!(21, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                6_512_849_198_636u64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(525_152, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
