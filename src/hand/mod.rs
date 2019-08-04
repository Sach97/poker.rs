pub mod rank;
use std::fmt::{Display, Formatter, Result};

use crate::card::Card;

#[allow(dead_code)]
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