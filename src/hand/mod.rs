#![feature(slice_partition_dedup)]
pub mod rank;
use rank::Rank;
// mod utils;
// use utils;
//use std::fmt::{Display, Formatter, Result};
//use std::array::FixedSizeArray;

use crate::card::face::Face;
use crate::card::suit::Suit;
use crate::card::Card;

#[derive(Debug, PartialEq)]
struct Hand(Vec<Card>);

impl Default for Hand {
    fn default() -> Self {
        Hand {
            0: Vec::with_capacity(5),
        }
    }

}

  fn handle_three_or_pair_kind(mut suits : Vec<Suit>) -> Rank { 
        let (_, _dup_hand) = suits.partition_dedup();
        match _dup_hand.len() {   
         0 =>Rank::TwoPairs,
         2 => Rank::ThreeOfAKind,
        _ => Rank::None,
        }
    }

#[allow(dead_code)]
impl Hand {

    pub fn new(cards: Vec<Card>) -> Self {
        Hand { 0: cards }
    }

    pub fn from_vec(cards: Vec<&str>) -> Self {
        Hand {
            0: cards.iter().map(|card| Card::from_string(card)).collect(),
        }
    }

    pub fn sort(&mut self) -> Self {
        self.0.sort();
        Hand {
            0: self.0.to_owned(),
        }
    }

    fn faces(&self) -> Vec<Face> {
        self.0.iter().map(|card| card.face).collect()
    }

    fn suits(&self) -> Vec<Suit> {
        self.0.iter().map(|card| card.suit).collect()
    }

  
    
    pub fn rank(&mut self) -> Rank {
        self.0.sort();
        let mut suits = self.suits();
        let (dedup, _dup_hand) = suits.partition_dedup();

        match _dup_hand.len() {
            
         1 => Rank::Pair,
         2 => handle_three_or_pair_kind(_dup_hand.to_vec()),
         3 => Rank::FourOfAKind,
        _ => Rank::None,
        }
    }

    // fn match_hand(hand : Vec<Suit>) -> Rank {

    // }

    //     pub fn rank(self) -> Rank {
    //         let hand = self.sort();
    // //         match hand.cards.as_slice() {
    // //             Card::new(Face::Ace, suit: Suit)
    // // }
    //         //what I would like:
    //           //if the sequence and royal => royal flush
    //         // sequence and same suit=> straight flush
    //         // sequence of same face => four of a kind
    //         // full house : three of a kind + two pairs

    //         match hand.as_slice(){
    //             [Face::Jack, Face::]
    //         }
    //     }
}

// impl Display for Hand {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         let mut str = "";
//         for card in &self.cards {
//             f.write_str(str)?;
//             f.write_str(&card.to_string())?;
//             str = ", ";
//         }
//         Ok(())

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_cards() {
        let mut hand = Hand::from_vec(vec!["Kd", "2h", "3d", "5s", "9c"]);
        hand.sort();
        assert_eq!(hand, Hand::from_vec(vec!["2h", "3d", "5s", "9c", "Kd"]));
    }

}
