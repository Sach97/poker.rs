use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

mod suit;
use suit::Suit;
mod face;
use face::Face;

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    face: Face,
}

// impl PartialEq for Card {
//     fn eq(&self, other: &Self) -> bool {
//         (self.face == other.face) && (self.suit == self.suit)
//     }
// }

#[allow(dead_code)]
impl Card {
    fn new(face: Face, suit: Suit) -> Card {
        Card {
            face: face,
            suit: suit,

        }
    }

    pub fn from_string(card: &str) -> Card {
        let (face, suit) = card.split_at(1);
        Card {
            face: Face::from_str(face).unwrap(),
            suit: Suit::from_str(suit).unwrap(),
        }

    }

}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", &self.face.to_string(), &self.suit.to_string())
    }
}