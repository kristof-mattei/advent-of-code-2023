use std::rc::Rc;

use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::{hash_map::Entry, HashMap};

advent_of_code_2023::solution!(512_797, 262_454);

fn parse_input(input: &str) -> Vec<Label> {
    let parsed = input.trim().split(',').map(Label::new).collect::<Vec<_>>();

    parsed
}

fn parse_input_split(input: &str) -> Vec<(Label, Option<FocalLength>)> {
    let mut parsed = vec![];

    for piece in input.trim().split(',') {
        if let Some((left, right)) = piece.split_once(|what| what == '=') {
            let lens = right.parse::<usize>().expect("Invalid lens").into();

            parsed.push((Label::new(left), Some(lens)));
        } else {
            let label = piece.strip_suffix('-').unwrap();

            parsed.push((Label::new(label), None));
        }
    }

    parsed
}

fn hash_single(s: &str) -> usize {
    s.chars().map(|c| c as usize).fold(0, |mut acc, curr| {
        acc += curr;
        acc *= 17;
        acc %= 256;

        acc
    })
}

struct Label {
    label: Rc<str>,
    r#box: usize,
}

#[derive(Clone, Copy)]
struct FocalLength(usize);

impl FocalLength {
    fn get_length(self) -> usize {
        self.0
    }
}

impl From<usize> for FocalLength {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Label {
    fn new(label: &str) -> Self {
        Self {
            label: label.into(),
            r#box: hash_single(label),
        }
    }

    fn get_box(&self) -> usize {
        self.r#box
    }

    fn get_label(&self) -> Rc<str> {
        self.label.clone()
    }
}

fn fold_parsed(parsed: &[Label]) -> usize {
    parsed.iter().fold(0, |acc, curr| acc + curr.get_box())
}

fn put_lenses_in_boxes(parsed: &[(Label, Option<FocalLength>)]) -> usize {
    let with_hasher = HashMap::<usize, (HashMap<Rc<str>, (FocalLength, usize)>, usize)>::new();

    let mut boxes = with_hasher;

    for (label, lens) in parsed {
        match (boxes.entry(label.get_box()), lens) {
            (Entry::Occupied(mut o), Some(lens)) => {
                // box exists, and we're adding / overriding a lens
                let (lens_box, last_entry) = o.get_mut();

                if let Some((l, _)) = lens_box.get_mut(&label.get_label()) {
                    *l = *lens;
                } else {
                    lens_box.insert(label.get_label(), (*lens, *last_entry + 1));
                    *last_entry += 1;
                };
            },
            (Entry::Occupied(mut o), None) => {
                // box exist, we're removing the lens with label `label`
                o.get_mut().0.remove(&label.get_label());
            },
            (Entry::Vacant(v), Some(lens)) => {
                // box empty, add lens
                v.insert(([(label.get_label(), (*lens, 0))].into(), 0));
            },
            (Entry::Vacant(_), None) => {},
        }
    }

    let mut total = 0;

    for (box_hash, (lenses, _)) in boxes {
        if lenses.is_empty() {
            continue;
        }

        let box_id = box_hash + 1;

        let mut lenses = lenses
            .iter()
            .map(|(_, (lens, insertion_order))| (*lens, *insertion_order))
            .collect::<Vec<(FocalLength, usize)>>();

        lenses.sort_by_key(|l| l.1);

        for (order, (focal_length, _)) in lenses.iter().enumerate() {
            let slot_id = order + 1;

            total += box_id * slot_id * focal_length.get_length();
        }
    }

    total
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        fold_parsed(&parsed).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input_split(input);

        put_lenses_in_boxes(&parsed).into()
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
            assert_eq!(512_797, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(1320, (Solution {}).part_1(&read_file("examples", &DAY)));
        }

        #[test]
        fn example_one_liner() {
            assert_eq!(52, (Solution {}).part_1("HASH"));
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{hash_single, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(262_454, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(145, (Solution {}).part_2(&read_file("examples", &DAY)));
        }

        #[test]
        fn example_rn() {
            assert_eq!(0, hash_single("rn"));
        }

        #[test]
        fn example_qp() {
            assert_eq!(1, hash_single("qp"));
        }
    }
}
