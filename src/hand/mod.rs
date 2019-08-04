pub mod rank;
use std::fmt::{Display, Formatter, Result};

use crate::card::Card;

#[derive(Debug, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    #[allow(dead_code)]
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand { cards: cards }
    }

    #[allow(dead_code)]
    pub fn from_vec(cards: Vec<&str>) -> Hand {
        let cast_cards: Vec<Card> = cards.iter().map(|card| Card::from_string(card)).collect();
        Hand { cards: cast_cards }
    }

    pub fn sort(self) -> Hand {
        let mut cards = self.cards;
        cards.sort();
        Hand {
            cards: cards.to_owned(), //hmm very hacky find a way to implement Clone, maybe ?
        }
    }

}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = "";
        for card in &self.cards {
            f.write_str(str)?;
            f.write_str(&card.to_string())?;
            str = ", ";
        }
        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_cards() {
        let hand = Hand::from_vec(vec!["Kd", "2h", "3d", "5s", "9c"]);
        assert_eq!(
            hand.sort(),
            Hand::from_vec(vec!["2h", "3d", "5s", "9c", "Kd"])
        );
    }

}
