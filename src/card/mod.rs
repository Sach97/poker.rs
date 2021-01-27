use itertools::Itertools;
use std::str::FromStr;
use std::{cmp::Ordering, iter};

pub mod suit;
use suit::Suit;
pub mod face;
use face::Face;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub face: Face,
}

#[allow(dead_code)]
impl Card {
    pub fn new(face: Face, suit: Suit) -> Card {
        Card {
            face: face,
            suit: suit,
        }
    }

    pub fn from_string(card: &str) -> Card {
        let card_chars = card.chars();
        let count = card_chars.clone().count();

        if count == 3 {
            let split: Vec<String> = card_chars
                .group_by(|&x| match x {
                    '\u{0030}'..='\u{0039}' => true,
                    _ => false,
                })
                .into_iter()
                .map(|(_, r)| r.collect())
                .collect();
            let num_pair: (&String, &String) = (&split[0], &split[1]);

            let (face, suit) = num_pair;
            Card {
                face: Face::from_str(&face).unwrap(),
                suit: Suit::from_str(&suit).unwrap(),
            }
        } else {
            let (face, suit) = card.split_at(1);
            Card {
                face: Face::from_str(&face).unwrap(),
                suit: Suit::from_str(&suit).unwrap(),
            }
        }
    }
}

// impl Display for Card {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{}{}", &self.face.to_string(), &self.suit.to_string())
//     }
// }

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.face.cmp(&other.face)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_card() {
        let card = Card::from_string("10s");
        assert_eq!(card, Card::new(Face::Ten, Suit::Spades));
    }

    #[test]
    fn test_card_ord() {
        let mut cards = vec![
            Card::from_string("Ks"),
            Card::from_string("As"),
            Card::from_string("10s"),
            Card::from_string("Qs"),
            Card::from_string("Js"),
        ];
        cards.sort();
        assert_eq!(
            cards,
            vec![
                Card::from_string("10s"),
                Card::from_string("Js"),
                Card::from_string("Qs"),
                Card::from_string("Ks"),
                Card::from_string("As")
            ]
        )
    }
}
