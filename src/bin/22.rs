use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

advent_of_code_2023::solution!(490, 96356);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Piece {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    start: Piece,
    end: Piece,
}

fn parse_piece(piece: &str) -> Piece {
    let piece: [u32; 3] = piece
        .split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    Piece {
        x: piece[0],
        y: piece[1],
        z: piece[2],
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (s, e) = line.split_once('~').unwrap();

            Brick {
                start: parse_piece(s),
                end: parse_piece(e),
            }
        })
        .collect::<Vec<Brick>>()
}

fn get_position_map(input: &[Brick]) -> HashMap<(u32, u32, u32), Brick> {
    let mut position_map = HashMap::new();

    for brick in input {
        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                for z in brick.start.z..=brick.end.z {
                    position_map.insert((x, y, z), *brick);
                }
            }
        }
    }

    position_map
}

fn stabilize_bricks(
    bricks: &mut [Brick],
    position_map: HashMap<(u32, u32, u32), Brick>,
) -> HashMap<(u32, u32, u32), Brick> {
    let mut settled_positions = position_map;

    let mut keep_going = true;

    while keep_going {
        keep_going = false;

        for brick in bricks.iter_mut() {
            let mut fall = true;

            // Check the position below for a different brick
            for x in brick.start.x..=brick.end.x {
                for y in brick.start.y..=brick.end.y {
                    for z in brick.start.z..=brick.end.z {
                        if z - 1 == 0
                            || settled_positions
                                .get(&(x, y, z - 1))
                                .is_some_and(|b| b != brick)
                        {
                            fall = false;
                        }
                    }
                }
            }

            // If the brick can fall, move it down
            if fall {
                keep_going = true;

                // Remove the brick from the position map
                for x in brick.start.x..=brick.end.x {
                    for y in brick.start.y..=brick.end.y {
                        for z in brick.start.z..=brick.end.z {
                            settled_positions.remove(&(x, y, z));
                        }
                    }
                }

                brick.start.z -= 1;
                brick.end.z -= 1;

                // Add the new position
                for x in brick.start.x..=brick.end.x {
                    for y in brick.start.y..=brick.end.y {
                        for z in brick.start.z..=brick.end.z {
                            settled_positions.insert((x, y, z), *brick);
                        }
                    }
                }
            }
        }
    }

    settled_positions
}

fn get_brick_dependencies(
    bricks: &[Brick],
    settled_map: &HashMap<(u32, u32, u32), Brick>,
) -> (
    HashMap<Brick, HashSet<Brick>>,
    HashMap<Brick, HashSet<Brick>>,
) {
    // Map bricks above and below each other
    let mut above: HashMap<Brick, HashSet<Brick>> = HashMap::new();
    let mut below: HashMap<Brick, HashSet<Brick>> = HashMap::new();

    for brick in bricks {
        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                for z in brick.start.z..=brick.end.z {
                    if let Some(other_brick) = settled_map.get(&(x, y, z + 1)) {
                        if other_brick != brick {
                            above
                                .entry(*brick)
                                .and_modify(|v| {
                                    v.insert(*other_brick);
                                })
                                .or_insert_with(|| {
                                    ([*other_brick]).into_iter().collect::<HashSet<_>>()
                                });

                            below
                                .entry(*other_brick)
                                .and_modify(|v| {
                                    v.insert(*brick);
                                })
                                .or_insert_with(|| ([*brick]).into_iter().collect::<HashSet<_>>());
                        }
                    }
                }
            }
        }
    }

    (above, below)
}

fn count_safe_to_remove(input: &str) -> PartSolution {
    let mut bricks = parse_input(input);

    let position_map = get_position_map(&bricks);

    let settled_map = stabilize_bricks(&mut bricks, position_map);

    let (above, below) = get_brick_dependencies(&bricks, &settled_map);

    let mut safe_to_remove = 0;

    // check brick dependencies for which ones are safe to remove
    for brick in bricks {
        let mut can_remove = true;

        // Check for bricks above this one
        for brick_above in above.get(&brick).unwrap_or(&HashSet::new()) {
            // If there is only one brick below then it's not safe to remove
            if below.get(brick_above).map_or(0, hashbrown::HashSet::len) == 1 {
                can_remove = false;
            }
        }

        if can_remove {
            safe_to_remove += 1;
        }
    }

    safe_to_remove.into()
}

fn count_total_bricks_disintegrated(input: &str) -> PartSolution {
    let mut bricks = parse_input(input);

    let position_map = get_position_map(&bricks);

    let settled_map = stabilize_bricks(&mut bricks, position_map);

    let (above, below) = get_brick_dependencies(&bricks, &settled_map);

    let mut total_bricks_disintegrated = 0;

    for brick in bricks {
        let mut bricks_disintegrated = HashSet::new();
        bricks_disintegrated.insert(brick);

        loop {
            let mut new_bricks_disintegrating = HashSet::new();

            for disintegrated_brick in &bricks_disintegrated {
                // For each brick above this one
                for brick_above in above.get(disintegrated_brick).unwrap_or(&HashSet::new()) {
                    if bricks_disintegrated.get(brick_above) .is_none() &&
                        // All the bricks below it have disintegrated
                        below.get(brick_above).is_some_and(|b| b.is_subset(&bricks_disintegrated))
                    {
                        new_bricks_disintegrating.insert(*brick_above);
                    }
                }
            }

            if new_bricks_disintegrating.is_empty() {
                break;
            }

            for new_brick in new_bricks_disintegrating {
                bricks_disintegrated.insert(new_brick);
            }
        }

        // Remove 1 from the total as we don't count the brick that started the chain reaction
        total_bricks_disintegrated += bricks_disintegrated.len() - 1;
    }

    total_bricks_disintegrated.into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_safe_to_remove(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_total_bricks_disintegrated(input)
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
            assert_eq!(490, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(5, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(96356, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(7, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
