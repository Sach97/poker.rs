use crate::card::face::Face;

//use std::cmp::Ordering;

//https://www.fgbradleys.com/et_poker.asp

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind(Face),
    StraightFlush,
    RoyalFlush,
}

#[cfg(test)]
mod tests {
    use crate::{card::face::Face, hand};

    use super::*;

    #[test]
    fn compare_two_hands() {
        let mut player = hand::Hand::from_vec(vec!["2h", "3h", "4h", "5h", "6h"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::StraightFlush);
        let mut opponent = hand::Hand::from_vec(vec!["Js", "Qs", "Ks", "As", "10s"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::RoyalFlush);
        assert_eq!(opponent_rank > player_rank, true);
    }
    #[test]
    fn compare_two_hands2() {
        let mut player = hand::Hand::from_vec(vec!["2h", "3h", "4h", "5h", "6h"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::StraightFlush);
        let mut opponent = hand::Hand::from_vec(vec!["As", "Ad", "Ac", "Ah", "Jd"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::FourOfAKind(Face::Ace));
        assert_eq!(player_rank > opponent_rank, true);
    }

    #[test]
    fn compare_two_hands3() {
        let mut player = hand::Hand::from_vec(vec!["As", "Ah", "2h", "Ad", "Ac"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::FourOfAKind(Face::Ace));
        let mut opponent = hand::Hand::from_vec(vec!["Js", "Jd", "Jc", "Jh", "3d"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::FourOfAKind(Face::Jack));
        assert_eq!(player_rank > opponent_rank, true);
    }

    #[test]
    fn test_sort_ranks() {
        let mut ranks = vec![
            Rank::RoyalFlush,
            Rank::StraightFlush,
            Rank::FourOfAKind(Face::Ace),
            Rank::FullHouse,
            Rank::Flush,
            Rank::Straight,
            Rank::ThreeOfAKind,
            Rank::TwoPairs,
            Rank::Pair,
            Rank::HighCard,
        ];
        ranks.sort();
        assert_eq!(
            ranks,
            vec![
                Rank::HighCard,
                Rank::Pair,
                Rank::TwoPairs,
                Rank::ThreeOfAKind,
                Rank::Straight,
                Rank::Flush,
                Rank::FullHouse,
                Rank::FourOfAKind(Face::Ace),
                Rank::StraightFlush,
                Rank::RoyalFlush,
            ]
        )
    }
}
