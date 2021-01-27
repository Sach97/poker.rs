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
        Some(self.face.cmp(&other.face))
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
        let card1 = Card::from_string("10s");
        let card2 = Card::from_string("7h");
        let card3 = Card::from_string("3c");
        let card4 = Card::from_string("6h");
        let card5 = Card::from_string("9s");
        assert!(card1 > card5);
        assert_eq!(card1, Card::new(Face::Ten, Suit::Spades));
        assert_eq!(card2, Card::new(Face::Seven, Suit::Hearts));
        assert_eq!(card3, Card::new(Face::Three, Suit::Clubs));
        assert_eq!(card4, Card::new(Face::Six, Suit::Hearts));
        assert_eq!(card5, Card::new(Face::Nine, Suit::Spades));
    }

    #[test]
    fn test_card_ord() {
        let mut cards = vec![
            Card::from_string("9h"),
            Card::from_string("Ks"),
            Card::from_string("10s"),
            Card::from_string("Qs"),
            Card::from_string("Js"),
        ];
        cards.sort();
        assert_eq!(
            cards,
            vec![
                Card::from_string("9h"),
                Card::from_string("10s"),
                Card::from_string("Js"),
                Card::from_string("Qs"),
                Card::from_string("Ks"),
            ]
        )
    }
}
