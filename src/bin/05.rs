use std::convert::Into;
use std::fmt::Debug;
use std::ops::Range;

use advent_of_code_2023::shared::{PartSolution, Parts};

use self::parse::{IndividualSeeds, RangeOfSeeds};

advent_of_code_2023::solution!(309_796_150, 50_716_416);

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn all_the_way(&self, mut seed: u64) -> u64 {
        for map in &self.maps {
            seed = map.map(seed);
        }

        seed
    }

    fn remap_seed_ranges_all_the_way(&self, mut seed_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        for map in &self.maps {
            seed_ranges = map.remap_seed_ranges(seed_ranges);
        }

        seed_ranges
    }
}

impl From<Vec<Vec<(u64, u64, u64)>>> for Almanac {
    fn from(value: Vec<Vec<(u64, u64, u64)>>) -> Self {
        Almanac {
            maps: value.into_iter().map(Into::into).collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AlmanacMap {
    ranges: Vec<DestinationSourceRange>,
}

impl AlmanacMap {
    fn map(&self, seed: u64) -> u64 {
        for range in &self.ranges {
            if let Some(mapped_seed) = range.map(seed) {
                return mapped_seed;
            }
        }

        seed
    }

    fn remap_seed_ranges(&self, seed_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        // run the seed_range through the mapping ONCE, splitting it into processed and unprocessed
        let mut processed = vec![]; // these have been split off and remapped

        let mut todo = seed_ranges;

        while let Some(seed_range) = todo.pop() {
            let mut processed_inner = vec![];
            let mut todo_inner = vec![];

            for dsr in &self.ranges {
                let map_source = &dsr.source;

                if seed_range.end <= map_source.start || seed_range.start >= map_source.end {
                    // no overlap
                } else if seed_range.start < map_source.start {
                    if seed_range.end <= map_source.end {
                        // seed_range: |___________|
                        //        map:       |__________|
                        //             S1    M1    S2    M2
                        // S1..M1
                        // M1..S2 (remap)
                        // or
                        // seed_range: |___________|
                        //        map:       |_____|
                        //             S1    M1    S2,M2
                        // S1..M1
                        todo_inner.push(seed_range.start..map_source.start);
                        // M1..S2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(seed_range.end),
                        );
                    } else {
                        // seed_range: |_________________|
                        //        map:       |_____|
                        //             S1    M1    M2    S2
                        // S1..M1
                        todo_inner.push(seed_range.start..map_source.start);
                        // M1..M2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(map_source.end),
                        );
                        // M2..S2
                        todo_inner.push(map_source.end..seed_range.end);
                    }
                } else if seed_range.start > map_source.start {
                    if seed_range.end <= map_source.end {
                        // seed_range:       |_____|
                        //        map: |_________________|
                        //             M1    S1    S2    M2
                        // S1..S2 (remap)
                        // or
                        // seed_range:       |_____|
                        //        map: |___________|
                        //             M1    S1    S2,M2
                        // S1..S2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(seed_range.start)
                                ..dsr.map_to_destination(seed_range.end),
                        );
                    } else {
                        // seed_range:       |___________|
                        //        map: |___________|
                        //             M1    S1    M2    S2
                        // S1..M2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(seed_range.start)
                                ..dsr.map_to_destination(map_source.end),
                        );
                        // M2..S2
                        todo_inner.push(map_source.end..seed_range.end);
                    }
                } else if seed_range.start == map_source.start {
                    if seed_range.end <= map_source.end {
                        // seed_range: |_____|
                        //        map: |___________|
                        //             S1,M1 S2    M2
                        // S1..S2 (remap)
                        // or
                        // seed_range: |_____|
                        //        map: |_____|
                        //             S1,M1 S2,M2
                        // S1..S2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(seed_range.end),
                        );
                    } else {
                        // seed_range: |___________|
                        //        map: |_____|
                        //             S1,M1 M2    S2
                        // M1..M2 (remap)
                        processed_inner.push(
                            dsr.map_to_destination(map_source.start)
                                ..dsr.map_to_destination(map_source.end),
                        );
                        // M2..S2
                        todo_inner.push(map_source.end..seed_range.end);
                    }
                } else {
                    // nothing happened
                    panic!("Uncovered case");
                }

                if !processed_inner.is_empty() {
                    break;
                }
            }

            if processed_inner.is_empty() {
                // we got through all the maps without processing anything, so the range is fully processed
                processed.push(seed_range);
            } else {
                // we got a hit on this map
                // we bail to avoid duplicate non-mapped pieces

                // processed pieces, retain them
                processed.append(&mut processed_inner);

                // enqueue the cut of pieces for re-processing
                todo.append(&mut todo_inner);
            }
        }

        processed
    }
}

impl From<Vec<(u64, u64, u64)>> for AlmanacMap {
    fn from(value: Vec<(u64, u64, u64)>) -> Self {
        AlmanacMap {
            ranges: value.into_iter().map(Into::into).collect::<Vec<_>>(),
        }
    }
}

#[derive(PartialEq, Eq)]
struct DestinationSourceRange {
    source: Range<u64>,
    destination: u64,
}

impl DestinationSourceRange {
    fn map(&self, seed: u64) -> Option<u64> {
        if (self.source).contains(&seed) {
            Some(self.map_to_destination(seed))
        } else {
            None
        }
    }

    fn map_to_destination(&self, seed: u64) -> u64 {
        seed + self.destination - self.source.start
    }
}

impl From<(u64, u64, u64)> for DestinationSourceRange {
    fn from(value: (u64, u64, u64)) -> Self {
        DestinationSourceRange {
            source: value.1..value.1 + value.2,
            destination: value.0,
        }
    }
}

impl std::fmt::Debug for DestinationSourceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestinationSourceRange")
            .field("Destination", &(self.destination))
            .field("Source", &(self.source))
            .finish()
    }
}

mod parse {
    use std::ops::Range;

    use super::{Almanac, AlmanacMap};

    pub(super) trait IntoSeed {
        fn into(raw_seeds: Vec<u64>) -> Self;
    }

    #[derive(Debug, PartialEq, Eq)]
    pub(super) struct IndividualSeeds(pub(super) Vec<u64>);
    impl IntoSeed for IndividualSeeds {
        fn into(raw_seed: Vec<u64>) -> Self {
            Self(raw_seed)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub(super) struct RangeOfSeeds(pub(super) Vec<Range<u64>>);

    impl IntoSeed for RangeOfSeeds {
        fn into(raw_seeds: Vec<u64>) -> Self {
            Self(parse_seed_range(&raw_seeds))
        }
    }

    fn parse_seed_range(seeds_with_range: &[u64]) -> Vec<Range<u64>> {
        seeds_with_range
            .chunks(2)
            .map(|s| s[0]..s[0] + s[1])
            .collect::<Vec<Range<u64>>>()
    }

    pub(super) fn parse_lines<S: IntoSeed>(lines: &str) -> (Almanac, S) {
        let mut seeds = None;
        let mut seed_to_soil_map = None;
        let mut soil_to_fertilizer_map = None;
        let mut fertilizer_to_water_map = None;
        let mut water_to_light_map = None;
        let mut light_to_temperature_map = None;
        let mut temperature_to_humidity_map = None;
        let mut humidity_to_location_map = None;

        for group in lines
            .lines()
            .collect::<Vec<&str>>()
            .split(|line| line.is_empty())
        {
            assert!(!group.is_empty());

            match group[0] {
                "seed-to-soil map:" => seed_to_soil_map = process_map(&group[1..]).into(),
                "soil-to-fertilizer map:" => {
                    soil_to_fertilizer_map = process_map(&group[1..]).into();
                },
                "fertilizer-to-water map:" => {
                    fertilizer_to_water_map = process_map(&group[1..]).into();
                },
                "water-to-light map:" => water_to_light_map = process_map(&group[1..]).into(),
                "light-to-temperature map:" => {
                    light_to_temperature_map = process_map(&group[1..]).into();
                },
                "temperature-to-humidity map:" => {
                    temperature_to_humidity_map = process_map(&group[1..]).into();
                },
                "humidity-to-location map:" => {
                    humidity_to_location_map = process_map(&group[1..]).into();
                },
                s if s.starts_with("seeds:") => {
                    let values = group[0]
                        .split(' ')
                        .skip(1)
                        .map(|s| s.parse::<u64>().expect("Invalid seed"))
                        .collect();

                    seeds = Some(S::into(values));
                },
                _ => panic!("Invalid group"),
            }
        }

        (
            Almanac {
                maps: vec![
                    seed_to_soil_map.expect("Missing map"),
                    soil_to_fertilizer_map.expect("Missing map"),
                    fertilizer_to_water_map.expect("Missing map"),
                    water_to_light_map.expect("Missing map"),
                    light_to_temperature_map.expect("Missing map"),
                    temperature_to_humidity_map.expect("Missing map"),
                    humidity_to_location_map.expect("Missing map"),
                ],
            },
            seeds.expect("Missing seeds"),
        )
    }

    fn process_map(lines: &[&str]) -> AlmanacMap {
        let mut almanac_map = Vec::new();
        for line in lines {
            let x = line
                .split(' ')
                .map(|s| s.parse().expect("Invalid number"))
                .collect::<Vec<u64>>();

            assert!(x.len() == 3);

            almanac_map.push((x[0], x[1], x[2]));
        }

        almanac_map.into()
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let (almanac, seeds) = parse::parse_lines::<IndividualSeeds>(input);

        let min_location = seeds
            .0
            .into_iter()
            .map(|s| almanac.all_the_way(s))
            .min()
            .expect("No min found");

        (min_location).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (almanac, mut range_of_seeds) = parse::parse_lines::<RangeOfSeeds>(input);

        let mut result_seed_ranges =
            almanac.remap_seed_ranges_all_the_way(range_of_seeds.0.clone());

        range_of_seeds.0.sort_by_key(|r| r.start);
        result_seed_ranges.sort_by_key(|r| r.start);

        let mut min = u64::MAX;

        'outer: for seed_range in result_seed_ranges {
            for original_seed_range in &range_of_seeds.0 {
                if original_seed_range.contains(&(seed_range.start - 1)) {
                    min = seed_range.start;
                    break 'outer;
                }

                if original_seed_range.contains(&(seed_range.end - 1)) {
                    min = original_seed_range.start;
                    break 'outer;
                }
            }
        }

        min.into()
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::parse::{parse_lines, IndividualSeeds};
        use crate::{Almanac, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(309_796_150, Solution {}.part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            let lines = read_file("examples", &DAY);

            let (almanac, seeds) = parse_lines::<IndividualSeeds>(&lines);

            assert_eq!(IndividualSeeds(vec![79, 14, 55, 13]), seeds);

            assert_eq!(
                almanac,
                Almanac::from(vec![
                    vec![(50, 98, 2), (52, 50, 48)],
                    vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
                    vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
                    vec![(88, 18, 7), (18, 25, 70)],
                    vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
                    vec![(0, 69, 1), (1, 0, 69)],
                    vec![(60, 56, 37), (56, 93, 4)]
                ])
            );
        }

        #[test]
        fn example_seed_to_soil() {
            let lines = read_file("examples", &DAY);

            let (almanac, _) = parse_lines::<IndividualSeeds>(&lines);

            assert_eq!(81, almanac.maps[0].map(79));
            assert_eq!(14, almanac.maps[0].map(14));
            assert_eq!(57, almanac.maps[0].map(55));
            assert_eq!(13, almanac.maps[0].map(13));
        }

        #[test]
        fn example_seed_to_location() {
            let lines = read_file("examples", &DAY);

            let (almanac, _) = parse_lines::<IndividualSeeds>(&lines);

            assert_eq!(81, almanac.maps[0].map(79));
            assert_eq!(81, almanac.maps[1].map(81));
            assert_eq!(81, almanac.maps[2].map(81));
            assert_eq!(74, almanac.maps[3].map(81));
            assert_eq!(78, almanac.maps[4].map(74));
            assert_eq!(78, almanac.maps[5].map(78));
            assert_eq!(82, almanac.maps[6].map(78));
        }

        #[test]
        fn seed_maps() {
            let lines = read_file("examples", &DAY);

            let (almanac, seeds) = parse_lines::<IndividualSeeds>(&lines);

            let locations = seeds
                .0
                .into_iter()
                .map(|s| almanac.all_the_way(s))
                .collect::<Vec<u64>>();

            assert_eq!(vec![82, 43, 86, 35], locations);
        }
    }

    mod part_2 {
        use std::ops::Range;

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use super::super::Solution;
        use crate::parse::{parse_lines, RangeOfSeeds};
        use crate::{AlmanacMap, DAY};

        #[test]
        fn outcome() {
            assert_eq!(50_716_416, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_test_ranges() {
            let lines = read_file("examples", &DAY);

            let (_, range_of_seeds) = parse_lines::<RangeOfSeeds>(&lines);

            assert_eq!(vec![79..93, 55..68], range_of_seeds.0);
        }

        #[test]
        fn example_test_min_ranges_naive() {
            let lines = read_file("examples", &DAY);

            let (almanac, range_of_seeds) = parse_lines::<RangeOfSeeds>(&lines);

            let location = range_of_seeds
                .0
                .into_iter()
                .filter_map(|s| s.clone().map(|s| almanac.all_the_way(s)).min())
                .min()
                .expect("No min found");

            assert_eq!(location, 46);
        }

        #[test]
        fn example_test_overlap() {
            let map: AlmanacMap = (vec![(0, 10, 5)]).into();

            let remapped = map.remap_seed_ranges(vec![Range { start: 5, end: 15 }]);

            assert_eq!(remapped, vec![0..5, 5..10]);
        }

        #[test]
        fn example_test_overlap_multiple() {
            let map: AlmanacMap = (vec![(0, 10, 5), (20, 5, 5)]).into();

            let remapped = map.remap_seed_ranges(vec![Range { start: 5, end: 15 }]);

            assert_eq!(remapped, vec![0..5, 20..25]);
        }

        #[test]
        fn example_lowest_in_original() {
            assert_eq!(55, Solution {}.part_2(&read_file("examples", &DAY)));
        }
    }
}
