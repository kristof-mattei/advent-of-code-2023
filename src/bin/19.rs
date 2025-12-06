use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::HashMap;
use regex::Regex;

advent_of_code_2023::solution!(420_739, 130_251_901_420_382_usize);

use std::sync::LazyLock;

static PART_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap());

static WORKFLOW_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?<name>[a-z]+)\{(?<pieces>(.*),?)\}").unwrap());

static WORKFLOW_REGEX_RULE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("(?<property>[xmas])(?<cmp>[<>])(?<value>[0-9]*):(?<target>[AR]|[a-z]*)").unwrap()
});

struct Part {
    // x: Extremely cool looking
    // m: Musical (it makes a noise when you hit it)
    // a: Aerodynamic
    // s: Shiny
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> Next {
        for rule in &self.rules {
            if let Some(n) = rule.process(part) {
                return n;
            }
        }

        panic!("Finished workflow without next")
    }
}

enum Rule {
    Lt(Property, usize, Next),
    Gt(Property, usize, Next),
    Next(Next),
}

impl Rule {
    fn process(&self, part: &Part) -> Option<Next> {
        match *self {
            Rule::Lt(ref p, v, ref next) => {
                if p.value(part) < v {
                    Some(next.clone())
                } else {
                    None
                }
            },
            Rule::Gt(ref p, v, ref next) => {
                if p.value(part) > v {
                    Some(next.clone())
                } else {
                    None
                }
            },
            Rule::Next(ref next) => Some(next.clone()),
        }
    }
}

enum Property {
    X,
    M,
    A,
    S,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Property::X => write!(f, "x"),
            Property::M => write!(f, "m"),
            Property::A => write!(f, "a"),
            Property::S => write!(f, "s"),
        }
    }
}

impl Property {
    fn value(&self, part: &Part) -> usize {
        match *self {
            Property::X => part.x,
            Property::M => part.m,
            Property::A => part.a,
            Property::S => part.s,
        }
    }
}

impl TryFrom<&str> for Property {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Property::X),
            "m" => Ok(Property::M),
            "a" => Ok(Property::A),
            "s" => Ok(Property::S),
            _ => Err("Invalid property"),
        }
    }
}

#[derive(Clone)]
enum Next {
    Accept,
    Reject,
    Named(String),
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let Some((unparsed_workflows, unparsed_parts)) = input.split_once("\n\n") else {
        panic!("invalid input");
    };

    let mut workflows = HashMap::<String, Workflow>::new();
    let mut parts = vec![];

    for unparsed_workflow in unparsed_workflows.lines() {
        let workflow = parse_workflow(unparsed_workflow);

        workflows.insert(workflow.name.clone(), workflow);
    }

    for unparsed_part in unparsed_parts.lines() {
        parts.push(parse_part(unparsed_part));
    }

    (workflows, parts)
}

fn parse_workflow(unparsed_workflow: &str) -> Workflow {
    // BNF
    // <start> ::= "in"
    // <name> ::= <start> | <identifier>
    // <identifier> ::= ([a-z])+
    //
    // <property> ::= "a" | "x" | "m" | "s"
    // <comparison> ::= ">" | "<"
    // <value> ::= [1-9] ([0-9])*
    // <next_workflow> ::= "A" | "R" | <identifier>
    //
    // <conditional_rule> ::= <property> <comparison> <value> ":" <next_workflow>
    // <workflow> ::= <name> "{" <conditional_rule> ("," <conditional_rule>)* ("," <next_workflow>)? "}"

    // px{a<2006:qkq,m>2090:A,rfg}

    // but we're lazy, so we're going to do regex

    let captures = WORKFLOW_REGEX.captures(unparsed_workflow).unwrap();

    let name = captures.name("name").unwrap().as_str();

    let mut rules = vec![];
    for unparsed_rule in captures.name("pieces").unwrap().as_str().split(',') {
        rules.push(parse_rule(unparsed_rule));
    }

    Workflow {
        name: name.into(),
        rules,
    }
}

fn parse_rule_name(unparsed_rule_name: &str) -> Next {
    match unparsed_rule_name {
        "A" => Next::Accept,
        "R" => Next::Reject,
        name => Next::Named(name.into()),
    }
}

fn parse_rule(unparsed_rule: &str) -> Rule {
    if let Some(captures) = WORKFLOW_REGEX_RULE.captures(unparsed_rule) {
        let property = captures
            .name("property")
            .unwrap()
            .as_str()
            .try_into()
            .unwrap();

        let value: usize = captures.name("value").unwrap().as_str().parse().unwrap();

        let target = parse_rule_name(captures.name("target").unwrap().as_str());

        match captures.name("cmp").unwrap().as_str() {
            "<" => Rule::Lt(property, value, target),
            ">" => Rule::Gt(property, value, target),
            _ => panic!("Invalid cmp"),
        }
    } else {
        Rule::Next(parse_rule_name(unparsed_rule))
    }
}

fn parse_part(unparsed_part: &str) -> Part {
    let captures = PART_REGEX.captures(unparsed_part).unwrap();

    Part {
        x: captures.name("x").unwrap().as_str().parse().unwrap(),
        m: captures.name("m").unwrap().as_str().parse().unwrap(),
        a: captures.name("a").unwrap().as_str().parse().unwrap(),
        s: captures.name("s").unwrap().as_str().parse().unwrap(),
    }
}

fn process_part(start: &Workflow, workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut current = start;

    loop {
        let result = current.process(part);

        match result {
            Next::Accept => {
                return true;
            },
            Next::Reject => {
                return false;
            },
            Next::Named(n) => {
                current = workflows
                    .get(&n)
                    .unwrap_or_else(|| panic!("Couldn't find the '{}' workflow", n));
            },
        }
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let (workflows, parts) = parse_input(input);

        let mut sum = 0;
        let start = workflows.get("in").expect("Couldn't find the 'in' start");

        for p in parts {
            if process_part(start, &workflows, &p) {
                sum += p.x + p.m + p.a + p.s;
            }
        }

        sum.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (workflows, _) = parse_input(input);

        let start = workflows.get("in").expect("Couldn't find the 'in' start");

        workflow_recursive(
            start,
            &workflows,
            Limits {
                x: (0, 4001),
                m: (0, 4001),
                a: (0, 4001),
                s: (0, 4001),
            },
            0,
        )
        .into()
    }
}

#[derive(Clone)]
struct Limits {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl std::fmt::Display for Limits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} < x < {}, ", self.x.0, self.x.1)?;
        write!(f, "{} < m < {}, ", self.m.0, self.m.1)?;
        write!(f, "{} < a < {}, ", self.a.0, self.a.1)?;
        write!(f, "{} < s < {}", self.s.0, self.s.1)?;

        Ok(())
    }
}

impl Limits {
    fn set_less(&mut self, p: &Property, v: usize) {
        match *p {
            Property::X => self.x.1 = v,
            Property::M => self.m.1 = v,
            Property::A => self.a.1 = v,
            Property::S => self.s.1 = v,
        }
    }

    fn set_more(&mut self, p: &Property, v: usize) {
        match *p {
            Property::X => self.x.0 = v,
            Property::M => self.m.0 = v,
            Property::A => self.a.0 = v,
            Property::S => self.s.0 = v,
        }
    }
}

fn next_recursive(
    next: &Next,
    workflows: &HashMap<String, Workflow>,
    limits: Limits,
    arg: usize,
) -> usize {
    let padding = std::iter::repeat_n(' ', arg).collect::<String>();
    match *next {
        Next::Accept => {
            println!("{}GOOD: {}", padding, limits);

            (limits.x.0 + 1..limits.x.1).len()
                * (limits.m.0 + 1..limits.m.1).len()
                * (limits.a.0 + 1..limits.a.1).len()
                * (limits.s.0 + 1..limits.s.1).len()
        },
        Next::Reject => {
            println!("{}IGNORE", padding);
            0
        },
        Next::Named(ref name) => {
            let wf = workflows.get(name).expect("bad cache");

            workflow_recursive(wf, workflows, limits, arg)
        },
    }
}

fn workflow_recursive(
    current: &Workflow,
    workflows: &HashMap<String, Workflow>,
    mut limits: Limits,
    arg: usize,
) -> usize {
    let mut sums = 0;

    let padding = std::iter::repeat_n(' ', arg).collect::<String>();

    for rule in &current.rules {
        match *rule {
            Rule::Lt(ref p, v, ref n) => {
                let mut clone = limits.clone();

                println!("{}{}<{}", padding, p, v);

                clone.set_less(p, v);
                limits.set_more(p, v - 1);

                sums += next_recursive(n, workflows, clone, arg + 4);
            },
            Rule::Gt(ref p, v, ref n) => {
                let mut clone = limits.clone();

                println!("{}{}>{}", padding, p, v);

                clone.set_more(p, v);
                limits.set_less(p, v + 1);

                sums += next_recursive(n, workflows, clone, arg + 4);
            },
            Rule::Next(ref n) => {
                sums += next_recursive(n, workflows, limits.clone(), arg);
            },
        }
    }

    sums
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::{test_example_part_1, test_part_1};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_1!(420_739);
        }

        #[test]
        fn example() {
            test_example_part_1!(19114);
        }
    }

    mod part_2 {

        use advent_of_code_2023::{test_example_part_2, test_part_2};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_2!(130_251_901_420_382_usize);
        }

        #[test]
        fn example() {
            test_example_part_2!(167_409_079_868_000_usize);
        }
    }
}
