pub mod rank;
use std::cmp::{Ord, Ordering};

use rank::Rank;

use crate::card::face::Face;
use crate::card::suit::Suit;
use crate::card::Card;
use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Hand([Card; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank = self.rank();
        let other_rank = &other.rank();
        Some(rank.cmp(&other_rank))
    }
}

impl Ord for &mut Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank = self.rank();
        let other_rank = other.rank();
        rank.cmp(&other_rank)
    }
}

// impl Default for Hand {
//     fn default() -> Self {
//         let cards: [Card; 5] = [Card::from_string("10s"); 5];
//         Hand { 0: cards }
//     }
// }

#[allow(dead_code)]
impl Hand {
    pub fn new(mut cards: [Card; 5]) -> Self {
        cards.sort();
        Hand { 0: cards }
    }

    pub fn from_vec(cards: Vec<&str>) -> Self {
        // cards.sort();
        let mut cards: Vec<Card> = cards.iter().map(|card| Card::from_string(card)).collect();
        cards.sort();
        let array: ArrayVec<_> = cards.into_iter().collect();
        Hand {
            0: array.into_inner().unwrap(),
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

    pub fn rank(self) -> Rank {
        let mut faces = self.faces();
        // self.sort();
        // faces.sort();
        let (dedup_hand, dup_hand) = faces.partition_dedup();
        dedup_hand.sort();
        dup_hand.sort();
        // println!("{:#?}", &faces);
        match dup_hand.len() {
            0 => self.handle_straight_or_flush_or_high(),
            1 => {
                let index = dedup_hand
                    .iter()
                    .position(|x| *x == *dup_hand.first().unwrap())
                    .unwrap();
                let mut remain = dedup_hand.to_owned();
                remain.remove(index);
                Rank::Pair(
                    dup_hand.last().unwrap().to_owned(),
                    remain.last().unwrap().to_owned(),
                )
            }
            2 => self.handle_three_or_pairs(dup_hand.to_vec()),
            3 => self.handle_four_or_full(dup_hand.to_vec()),
            _ => Rank::HighCard(faces.last().unwrap().to_owned()),
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

    fn is_royal_flush(&self) -> bool {
        self.faces().sort();
        self.is_flush() && self.faces()[0] == Face::Ten
    }

    fn handle_straight_or_flush_or_high(&self) -> Rank {
        let highest = self.faces().last().unwrap().to_owned();

        if self.is_royal_flush() {
            Rank::RoyalFlush
        } else {
            if self.is_straight() {
                if self.is_flush() {
                    Rank::StraightFlush(highest)
                } else {
                    Rank::Straight
                }
            } else {
                if self.is_flush() {
                    Rank::Flush(highest)
                } else {
                    Rank::HighCard(highest)
                }
            }
        }
    }

    fn handle_three_or_pairs(&self, mut faces: Vec<Face>) -> Rank {
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {
            0 => Rank::TwoPairs,
            _ => Rank::ThreeOfAKind,
        }
    }

    fn handle_four_or_full(&self, mut faces: Vec<Face>) -> Rank {
        let (_, _dup_hand) = faces.partition_dedup();
        match _dup_hand.len() {
            2 => Rank::FourOfAKind(_dup_hand.last().unwrap().to_owned()),
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

        let mut hand = Hand::from_vec(vec!["Kd", "2h", "3d", "5s", "10h"]);
        hand.sort();
        assert_eq!(hand, Hand::from_vec(vec!["2h", "3d", "5s", "Kd", "10h"]));
    }

    #[test]
    fn test_rank_pair() {
        let hand = Hand::from_vec(vec!["2d", "2h", "3d", "5s", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::Pair(Face::Two, Face::Nine));
    }

    #[test]
    fn test_rank_royal_flush() {
        let hand = Hand::from_vec(vec!["10d", "Jd", "Qd", "Kd", "Ad"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::RoyalFlush);
    }

    #[test]
    fn test_rank_straight_flush() {
        let hand = Hand::from_vec(vec!["5d", "6d", "7d", "8d", "9d"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::StraightFlush(Face::Nine));
    }

    #[test]
    fn test_rank_flush() {
        let hand = Hand::from_vec(vec!["5d", "2d", "7d", "8d", "9d"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::Flush(Face::Nine));
    }

    #[test]
    fn test_rank_straight() {
        let hand = Hand::from_vec(vec!["5h", "6s", "7d", "8d", "9d"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::Straight);
    }

    #[test]
    fn test_rank_two_pairs() {
        let hand = Hand::from_vec(vec!["2d", "2h", "3s", "3s", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::TwoPairs);
    }

    #[test]
    fn test_rank_three_kind() {
        let hand = Hand::from_vec(vec!["2d", "2c", "2s", "3s", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::ThreeOfAKind);
    }

    #[test]
    fn test_rank_four_kind() {
        let hand = Hand::from_vec(vec!["2d", "2c", "2s", "2h", "9c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::FourOfAKind(Face::Two));
    }

    #[test]
    fn test_rank_full_house() {
        let hand = Hand::from_vec(vec!["2d", "2c", "2s", "3h", "3c"]);
        let rank = hand.rank();
        println!("{:?}", rank);
        assert_eq!(rank, Rank::FullHouse);
    }
}
