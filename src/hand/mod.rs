#![feature(slice_partition_dedup)]
pub mod rank;
use rank::Rank;
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
        let mut faces = self.faces();
        faces.sort();
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {
         1 => Rank::Pair,
         2 => Hand::handle_three_or_pairs(_dup_hand.to_vec()),
         3 => Hand::handle_four_or_full(_dup_hand.to_vec()),
         _ => Rank::HighCard,
        }
    }

    fn handle_three_or_pairs(mut faces : Vec<Face>) -> Rank { //hacky way, put this in the trait and fight the compiler
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {   
         0 => Rank::TwoPairs,
         _ => Rank::ThreeOfAKind,
        }
    }
    
    fn handle_four_or_full(mut faces : Vec<Face>) -> Rank { //hacky way, put this in the trait and fight the compiler
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {   
         2 => Rank::FourOfAKind,
         _ => Rank::FullHouse,
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

    #[test]
    fn test_rank_pair() {
         let mut hand = Hand::from_vec(vec!["2d", "2h", "3d", "5s", "9c"]);
         let rank = hand.rank();
         println!("{:?}",rank);
         assert_eq!(rank, Rank::Pair);
    }

    #[test]
    fn test_rank_two_pairs() {
         let mut hand = Hand::from_vec(vec!["2d", "2h", "3s", "3s", "9c"]);
         let rank = hand.rank();
         println!("{:?}",rank);
         assert_eq!(rank, Rank::TwoPairs);
    }

     #[test]
    fn test_rank_three_kind() {
         let mut hand = Hand::from_vec(vec!["2d", "2c", "2s", "3s", "9c"]);
         let rank = hand.rank();
         println!("{:?}",rank);
         assert_eq!(rank, Rank::ThreeOfAKind);
    }
    
     #[test]
    fn test_rank_four_kind() {
         let mut hand = Hand::from_vec(vec!["2d", "2c", "2s", "2h", "9c"]);
         let rank = hand.rank();
         println!("{:?}",rank);
         assert_eq!(rank, Rank::FourOfAKind);
    }

     #[test]
    fn test_rank_full_house() {
         let mut hand = Hand::from_vec(vec!["2d", "2c", "2s", "3h", "3c"]);
         let rank = hand.rank();
         println!("{:?}",rank);
         assert_eq!(rank, Rank::FullHouse);
    }
}
