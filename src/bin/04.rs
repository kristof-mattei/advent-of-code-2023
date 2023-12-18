use hashbrown::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

advent_of_code_2023::solution!(18_619, 8_063_216);

use advent_of_code_2023::shared::{PartSolution, Parts};

struct Game {
    game_number: usize,
    intersection_count: usize,
}

impl Hash for Game {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.game_number.hash(state);
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.game_number == other.game_number
    }
}

impl Eq for Game {}

fn parse_lines(lines: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in lines.lines() {
        let (game, all_numbers) = line.split_once(':').expect("Invalid game");

        let (winning_numbers, my_numbers) = all_numbers.split_once('|').expect("Invalid numbers");

        let intersection_count = split_string_into_numbers(winning_numbers)
            .intersection(&split_string_into_numbers(my_numbers))
            .count();

        games.push(Game {
            game_number: parse_game_number(game),
            intersection_count,
        });
    }

    games
}

fn get_scores(games: &[Game]) -> usize {
    let mut scores = 0;

    for game in games {
        let score = game.get_score();

        scores += score;
    }

    scores
}

impl Game {
    fn get_score(&self) -> usize {
        let intersection: u32 = self
            .get_intersection_count()
            .try_into()
            .expect("Intersection too large");

        intersection.checked_sub(1).map_or(0, |i| 2usize.pow(i))
    }

    fn get_intersection_count(&self) -> usize {
        self.intersection_count
    }
}

fn parse_game_number(game: &str) -> usize {
    let (_, game_number) = game.split_once(' ').expect("Invalid game");

    game_number
        .trim()
        .parse::<usize>()
        .expect("Invalid game number")
}

fn count_played_cards(games: &mut [Game]) -> u32 {
    let mut cache: HashMap<&Game, u32> = HashMap::new();

    count_played_cards_r(Some(games), true, &mut cache)
}

fn count_played_cards_r<'games>(
    games: Option<&'games [Game]>,
    base_run: bool,
    cache: &mut HashMap<&'games Game, u32>,
) -> u32 {
    let Some(games) = games else {
        return 0;
    };

    let Some(game) = games.get(0) else {
        return 0;
    };

    if let Some(c) = cache.get(&game) {
        return *c;
    }

    let mut total_cards = 1;

    for offset in 0..game.get_intersection_count() {
        total_cards += count_played_cards_r(games.get(1 + offset..), false, cache);
    }

    cache.insert(game, total_cards);

    // there is one base run where we always advance
    if base_run {
        for i in 1..games.len() {
            total_cards += count_played_cards_r(games.get(i..), false, cache);
        }
    }

    total_cards
}

fn split_string_into_numbers(numbers: &str) -> HashSet<u32> {
    numbers
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<u32>().expect("Invalid number"))
        .collect()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let games = parse_lines(input);

        get_scores(&games).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut parsed = parse_lines(input);

        let total_cards = count_played_cards(&mut parsed);

        total_cards.into()
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
            assert_eq!(18_619, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(13, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(8_063_216, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(30, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
