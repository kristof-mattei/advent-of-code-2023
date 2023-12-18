use std::cmp::Ordering;

use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2023::solution!(248_559_379, 249_631_254);

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    // for part 2
    Wildcard = 1,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(format!(
                "Invalid character: {} is not part of a deck",
                value
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut counts = HashMap::<&Card, usize>::new();
        self.cards.iter().for_each(|c| {
            if let Some(cc) = counts.get_mut(c) {
                *cc += 1;
            } else {
                counts.insert(c, 1);
            }
        });

        // part 2 wildcard detection
        if let Some(wildcard_count) = counts.remove(&Card::Wildcard) {
            if wildcard_count == 5 {
                return Type::FiveOfAKind;
            }

            let mut max_k = None;
            let mut max_v = 0;

            for (k, v) in &counts {
                if v > &max_v {
                    max_k = Some(*k);
                    max_v = *v;
                }
            }

            let max_k = max_k.expect("Invalid count");

            *(counts.get_mut(max_k)).unwrap() += wildcard_count;
        }

        match counts.len() {
            1 => Type::FiveOfAKind,
            2 => {
                if counts.values().any(|c| c == &4) {
                    Type::FourOfAKind
                } else {
                    Type::FullHouse
                }
            },
            3 => {
                if counts.values().any(|c| c == &3) {
                    Type::ThreeOfAKind
                } else {
                    Type::TwoPair
                }
            },
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => panic!("Too many cards"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.get_type().cmp(&other.get_type());

        if cmp == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            cmp
        }
    }
}

fn parse_game(game: &str) -> Vec<Card> {
    game.chars()
        .map(|c| c.try_into().expect("Invalid character"))
        .collect::<Vec<Card>>()
}

fn parse_game_jack_is_wildcard(game: &str) -> Vec<Card> {
    let mut parsed = parse_game(game);

    for card in &mut parsed {
        if card == &Card::Jack {
            *card = Card::Wildcard;
        }
    }

    parsed
}

fn _parse_hands(input: &str, parse_game_fn: fn(&str) -> Vec<Card>) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let (game, bid) = line.split_once(' ').expect("Invalid game");

        let parsed_game = parse_game_fn(game);

        let parsed_bid = bid.parse().expect("Invalid bid");
        hands.push(Hand {
            cards: parsed_game,
            bid: parsed_bid,
        });
    }

    hands
}

fn parse_hands(input: &str) -> Vec<Hand> {
    _parse_hands(input, parse_game)
}

fn parse_hands_jack_is_wildcard(input: &str) -> Vec<Hand> {
    _parse_hands(input, parse_game_jack_is_wildcard)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut hands = parse_hands(input);

        hands.sort();

        hands
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, hand)| acc + ((i + 1) * hand.bid))
            .into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut hands = parse_hands_jack_is_wildcard(input);

        hands.sort();

        hands
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, hand)| acc + ((i + 1) * hand.bid))
            .into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{parse_game, parse_hands, Card, Hand, Solution, Type, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                248_559_379,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(6440, (Solution {}).part_1(&read_file("examples", &DAY)));
        }

        #[test]
        fn parse_example() {
            let input = &read_file("examples", &DAY);

            let hands = parse_hands(input.as_str());

            assert_eq!(
                vec![
                    Hand {
                        cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                        bid: 765
                    },
                    Hand {
                        cards: vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                        bid: 684
                    },
                    Hand {
                        cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                        bid: 28
                    },
                    Hand {
                        cards: vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                        bid: 220
                    },
                    Hand {
                        cards: vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                        bid: 483
                    }
                ],
                hands
            );
        }

        #[test]
        fn parse_hand_type_five_of_a_kind() {
            assert_eq!(
                Hand {
                    cards: parse_game("AAAAA"),
                    bid: 0
                }
                .get_type(),
                Type::FiveOfAKind
            );
        }

        #[test]
        fn parse_hand_type_four_of_a_kind() {
            assert_eq!(
                Hand {
                    cards: parse_game("AA8AA"),
                    bid: 0
                }
                .get_type(),
                Type::FourOfAKind
            );
        }

        #[test]
        fn parse_hand_type_full_house() {
            assert_eq!(
                Hand {
                    cards: parse_game("23332"),
                    bid: 0
                }
                .get_type(),
                Type::FullHouse
            );
        }

        #[test]
        fn parse_hand_type_three_of_a_kind() {
            assert_eq!(
                Hand {
                    cards: parse_game("TTT98"),
                    bid: 0
                }
                .get_type(),
                Type::ThreeOfAKind
            );
        }

        #[test]
        fn parse_hand_type_two_pair() {
            assert_eq!(
                Hand {
                    cards: parse_game("23432"),
                    bid: 0
                }
                .get_type(),
                Type::TwoPair
            );
        }

        #[test]
        fn parse_hand_type_one_pair() {
            assert_eq!(
                Hand {
                    cards: parse_game("A23A4"),
                    bid: 0
                }
                .get_type(),
                Type::OnePair
            );
        }

        #[test]
        fn parse_hand_type_high_card() {
            assert_eq!(
                Hand {
                    cards: parse_game("23456"),
                    bid: 0
                }
                .get_type(),
                Type::HighCard
            );
        }

        #[test]
        fn sort_order_1() {
            let left = Hand {
                cards: parse_game("33332"),
                bid: 0,
            };

            let right = Hand {
                cards: parse_game("2AAAA"),
                bid: 0,
            };

            assert_eq!(left.get_type(), right.get_type());

            assert!(left > right);
        }

        #[test]
        fn sort_order_2() {
            let left = Hand {
                cards: parse_game("77888"),
                bid: 0,
            };

            let right = Hand {
                cards: parse_game("77788"),
                bid: 0,
            };

            assert_eq!(left.get_type(), right.get_type());

            assert!(left > right);
        }

        #[test]
        fn example_sort_order() {
            let input = &read_file("examples", &DAY);

            let mut hands = parse_hands(input);

            hands.sort();

            assert_eq!(
                vec![
                    Hand {
                        cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                        bid: 765
                    },
                    Hand {
                        cards: vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                        bid: 220
                    },
                    Hand {
                        cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                        bid: 28
                    },
                    Hand {
                        cards: vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                        bid: 684
                    },
                    Hand {
                        cards: vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                        bid: 483
                    },
                ],
                hands
            );
        }
    }

    mod part_2 {
        use advent_of_code_2023::shared::solution::read_file;
        use advent_of_code_2023::shared::Parts;

        use crate::{parse_game, parse_game_jack_is_wildcard, Hand, Solution, Type, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                249_631_254,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(5905, (Solution {}).part_2(&read_file("examples", &DAY)));
        }

        #[test]
        fn get_type_baed_on_wildcards() {
            let t = (Hand {
                cards: parse_game_jack_is_wildcard("QJJQ2"),
                bid: 0,
            })
            .get_type();

            assert_eq!(t, Type::FourOfAKind);
        }

        #[test]
        fn compare_wildcards() {
            let left = Hand {
                cards: parse_game_jack_is_wildcard("JKKK2"),
                bid: 0,
            };

            let right = Hand {
                cards: parse_game("QQQQ2"),
                bid: 0,
            };

            assert_eq!(left.get_type(), right.get_type());
            assert!(left < right);
        }
    }
}
