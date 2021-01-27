#![feature(slice_partition_dedup)]
pub mod rank;
use rank::Rank;
use std::slice::Windows;
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
    pub fn new(mut cards: Vec<Card>) -> Self {
        cards.sort();
        Hand { 0: cards }
    }

    pub fn from_vec(mut cards: Vec<&str>) -> Self {
        cards.sort();
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
            0 => self.handle_straight_or_flush(),
            1 => Rank::Pair,
            2 => self.handle_three_or_pairs(_dup_hand.to_vec()),
            3 => self.handle_four_or_full(_dup_hand.to_vec()),
            _ => Rank::HighCard,
        }
    }

    fn is_straight(&self) -> bool {
        self.faces()
            .into_iter()
            .map(|face| face as i8 - self.0[0].face as i8)
            .eq(0..5)
    }

    fn is_flush(&self) -> bool {
        self.suits().windows(2).all(|w| w[0] == w[1])
    }

    fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    fn is_royal_flush(&mut self) -> bool {
        self.faces().sort();
        self.is_flush() && self.faces()[0] == Face::Ten
    }

    fn handle_straight_or_flush(&mut self) -> Rank {
        if self.is_royal_flush() {
            Rank::RoyalFlush
        } else {
            if self.is_straight() {
                if self.is_flush() {
                    Rank::StraightFlush
                } else {
                    Rank::Straight
                }
            } else {
                Rank::Flush
            }
        }
    }

    fn handle_three_or_pairs(&self, mut faces: Vec<Face>) -> Rank {
        //hacky way, put this in the trait and fight the compiler
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {
            0 => Rank::TwoPairs,
            _ => Rank::ThreeOfAKind,
        }
    }

    fn handle_four_or_full(&self, mut faces: Vec<Face>) -> Rank {
        //hacky way, put this in the trait and fight the compiler
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {
            2 => Rank::FourOfAKind(faces.last().unwrap().to_owned()),
            _ => Rank::FullHouse,
        }
    }
}

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
        println!("{:?}", rank);
        assert_eq!(rank, Rank::Pair);
    }

    #[test]
    fn test_rank_royal_flush() {
        let mut hand = Hand::from_vec(vec!["10d", "Jd", "Qd", "Kd", "Ad"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::RoyalFlush);
    }

    #[test]
    fn test_rank_straight_flush() {
        let mut hand = Hand::from_vec(vec!["5d", "6d", "7d", "8d", "9d"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::StraightFlush);
    }

    #[test]
    fn test_rank_flush() {
        let mut hand = Hand::from_vec(vec!["5d", "2d", "7d", "8d", "9d"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::Flush);
    }

    #[test]
    fn test_rank_straight() {
        let mut hand = Hand::from_vec(vec!["5h", "6s", "7d", "8d", "9d"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::Straight);
    }

    #[test]
    fn test_rank_two_pairs() {
        let mut hand = Hand::from_vec(vec!["2d", "2h", "3s", "3s", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::TwoPairs);
    }

    #[test]
    fn test_rank_three_kind() {
        let mut hand = Hand::from_vec(vec!["2d", "2c", "2s", "3s", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::ThreeOfAKind);
    }

    #[test]
    fn test_rank_four_kind() {
        let mut hand = Hand::from_vec(vec!["2d", "2c", "2s", "2h", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::FourOfAKind(Face::Two));
    }

    #[test]
    fn test_rank_full_house() {
        let mut hand = Hand::from_vec(vec!["2d", "2c", "2s", "3h", "3c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::FullHouse);
    }
}
