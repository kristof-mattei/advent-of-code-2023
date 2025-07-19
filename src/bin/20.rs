use std::collections::VecDeque;

use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2023::solution!(763_500_168, 207_652_583_562_007_usize);

const BROADCASTER: &str = "broadcaster";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Kind {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Broadcaster {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    sources: HashMap<String, Pulse>,
}

impl std::hash::Hash for Conjunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for s in &self.sources {
            s.0.hash(state);
            s.1.hash(state);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FlipFlop {
    state: State,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Pulse {
    High,
    Low,
}

trait TModule {
    fn receive(
        &mut self,
        source: &str,
        destinations: &[String],
        pulse: Pulse,
    ) -> Vec<(String, Pulse)>;
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Pulse::High => "high",
            Pulse::Low => "low",
        };

        write!(f, "{}", s)
    }
}

impl TModule for Broadcaster {
    fn receive(
        &mut self,
        _source: &str,
        destinations: &[String],
        pulse: Pulse,
    ) -> Vec<(String, Pulse)> {
        let mut v = vec![];

        for d in destinations {
            v.push((d.clone(), pulse));
        }

        v
    }
}

impl Broadcaster {
    fn new() -> Self {
        Self {}
    }
}

impl Conjunction {
    fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }
}

impl TModule for Conjunction {
    fn receive(
        &mut self,
        source: &str,

        destinations: &[String],
        pulse: Pulse,
    ) -> Vec<(String, Pulse)> {
        *self.sources.get_mut(source).unwrap() = pulse;

        if self.sources.iter().all(|(_, v)| v == &Pulse::High) {
            let mut v = vec![];

            for d in destinations {
                v.push((d.clone(), Pulse::Low));
            }

            v
        } else {
            let mut v = vec![];

            for d in destinations {
                v.push((d.clone(), Pulse::High));
            }

            v
        }
    }
}

impl FlipFlop {
    fn new() -> Self {
        Self { state: State::Off }
    }
}

impl TModule for FlipFlop {
    fn receive(
        &mut self,
        _source: &str,

        destinations: &[String],
        pulse: Pulse,
    ) -> Vec<(String, Pulse)> {
        match pulse {
            Pulse::High => vec![],
            Pulse::Low => match self.state {
                State::Off => {
                    self.state = State::On;

                    let mut v = vec![];

                    for d in destinations {
                        v.push((d.clone(), Pulse::High));
                    }

                    v
                },
                State::On => {
                    self.state = State::Off;
                    let mut v = vec![];

                    for d in destinations {
                        v.push((d.clone(), Pulse::Low));
                    }

                    v
                },
            },
        }
    }
}

fn to_module(
    unparsed_module_name: &str,
    unparsed_destinations: &str,
) -> (String, (Kind, Vec<String>)) {
    let destinations = unparsed_destinations
        .split(", ")
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    if unparsed_module_name == BROADCASTER {
        (
            String::from(BROADCASTER),
            (Kind::Broadcaster(Broadcaster::new()), destinations),
        )
    } else if let Some(stripped) = unparsed_module_name.strip_prefix('%') {
        (
            String::from(stripped),
            (Kind::FlipFlop(FlipFlop::new()), destinations),
        )
    } else if let Some(stripped) = unparsed_module_name.strip_prefix('&') {
        (
            String::from(stripped),
            (Kind::Conjunction(Conjunction::new()), destinations),
        )
    } else {
        panic!("not recognized")
    }
}

fn parse_input(input: &str) -> HashMap<String, (Kind, Vec<String>)> {
    let mut modules = HashMap::new();

    for line in input.lines() {
        let (unparsed_module_name, unparsed_destinations) = line.split_once(" -> ").unwrap();
        let (module_name, module) = to_module(unparsed_module_name, unparsed_destinations);
        modules.insert(module_name, module);
    }

    // now we need to ensure all our conjunction modules have a record of its inputs

    let mut conjunction_module_names = modules
        .iter()
        .filter_map(|(key, &(ref kind, _))| {
            if matches!(kind, &Kind::Conjunction(_)) {
                Some((key.clone(), vec![]))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    for (source, module) in &modules {
        let &(_, ref destinations) = module;

        for destination in destinations {
            if let Some(d) = conjunction_module_names.get_mut(destination) {
                d.push(source.clone());
            } else {
                // not a conjunction module
            }
        }
    }

    for (conjunction_module_name, sources) in conjunction_module_names {
        let module = modules.get_mut(&conjunction_module_name).unwrap();

        if let &mut (Kind::Conjunction(ref mut conjunction), _) = module {
            for source in sources {
                conjunction.sources.insert(source, Pulse::Low);
            }
        }
    }

    modules
}

fn handle_signals(
    modules: &mut HashMap<String, (Kind, Vec<String>)>,
    source: &str,
    handler_name: &str,
    pulse: Pulse,
) -> Vec<(String, String, Pulse)> {
    let mut signals: Vec<(String, String, Pulse)> = vec![];

    match modules.get_mut(handler_name) {
        Some(&mut (Kind::FlipFlop(ref mut ff), ref destinations)) => {
            for (new_destination, new_pulse) in ff.receive(source, destinations, pulse) {
                signals.push((handler_name.to_owned(), new_destination, new_pulse));
            }
        },
        Some(&mut (Kind::Conjunction(ref mut c), ref destinations)) => {
            for (new_destination, new_pulse) in c.receive(source, destinations, pulse) {
                signals.push((handler_name.to_owned(), new_destination, new_pulse));
            }
        },
        Some(&mut (Kind::Broadcaster(ref mut b), ref destinations)) => {
            for (new_destination, new_pulse) in b.receive(source, destinations, pulse) {
                signals.push((handler_name.to_owned(), new_destination, new_pulse));
            }
        },
        None => {},
    }

    signals
}

fn press_button_1000(mut modules: HashMap<String, (Kind, Vec<String>)>) -> usize {
    let (mut low, mut high) = (0, 0);

    for _ in 1_usize..=1000 {
        let mut signals = VecDeque::from_iter([(
            String::from("button"),
            String::from(BROADCASTER),
            Pulse::Low,
        )]);

        while let Some((source, destination, pulse)) = signals.pop_front() {
            if pulse == Pulse::Low {
                low += 1;
            } else {
                high += 1;
            }

            let new_signals = handle_signals(&mut modules, &source, &destination, pulse);

            for s in new_signals {
                signals.push_back(s);
            }
        }
    }

    low * high
}

fn press_button_forever(mut modules: HashMap<String, (Kind, Vec<String>)>) -> usize {
    let rx_feeder = modules
        .iter()
        .find_map(|(n, &(_, ref d))| {
            if d.contains(&"rx".into()) {
                Some(n.clone())
            } else {
                None
            }
        })
        .unwrap();

    let mut visited = modules
        .iter()
        .filter_map(|(n, &(_, ref d))| {
            if d.contains(&rx_feeder) {
                Some((n.clone(), 0_usize))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let mut lengths = HashMap::new();

    for i in 1.. {
        let mut signals = VecDeque::from_iter([(
            String::from("button"),
            String::from(BROADCASTER),
            Pulse::Low,
        )]);

        while let Some((source, destination, pulse)) = signals.pop_front() {
            if destination == rx_feeder && pulse == Pulse::High {
                *visited.get_mut(&source).unwrap() += 1;

                if !lengths.contains_key(&source) {
                    lengths.insert(source.clone(), i);
                }

                if visited.values().all(|v| v > &0) {
                    let mut product = 1;

                    for l in lengths.values() {
                        product *= l;
                    }

                    return product;
                }
            }

            let new_signals = handle_signals(&mut modules, &source, &destination, pulse);

            for (source, destination, pulse) in new_signals {
                signals.push_back((source, destination, pulse));
            }
        }
    }

    panic!()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let modules = parse_input(input);

        press_button_1000(modules).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let modules = parse_input(input);

        press_button_forever(modules).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::Parts as _;
        use advent_of_code_2023::shared::solution::{read_file, read_file_part};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                763_500_168,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example_1() {
            assert_eq!(
                32_000_000,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                11_687_500,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {

        use advent_of_code_2023::shared::Parts as _;
        use advent_of_code_2023::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                207_652_583_562_007_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }
    }
}
