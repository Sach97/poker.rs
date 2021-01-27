use crate::card::face::Face;

//use std::cmp::Ordering;

//https://www.fgbradleys.com/et_poker.asp

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    HighCard(Face),
    Pair(Face, Face),
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush(Face),
    FullHouse,
    FourOfAKind(Face),
    StraightFlush(Face),
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
        assert_eq!(player_rank, Rank::StraightFlush(Face::Six));
        let mut opponent = hand::Hand::from_vec(vec!["Js", "Qs", "Ks", "As", "10s"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::RoyalFlush);
        assert_eq!(opponent_rank > player_rank, true);
    }
    #[test]
    fn compare_two_hands2() {
        let mut player = hand::Hand::from_vec(vec!["2h", "3h", "4h", "5h", "6h"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::StraightFlush(Face::Six));
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
    fn compare_two_hands4() {
        let mut player = hand::Hand::from_vec(vec!["As", "3s", "4s", "8s", "2s"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::Flush(Face::Ace));
        let mut opponent = hand::Hand::from_vec(vec!["2h", "3h", "5h", "6h", "7h"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::Flush(Face::Seven));
        assert_eq!(player_rank > opponent_rank, true);
    }

    #[test]
    fn compare_two_hands5() {
        let mut player = hand::Hand::from_vec(vec!["2h", "3h", "4h", "5h", "6h"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::StraightFlush(Face::Six));
        let mut opponent = hand::Hand::from_vec(vec!["4h", "5h", "6h", "7h", "8h"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::StraightFlush(Face::Eight));
        assert_eq!(opponent_rank > player_rank, true);
    }

    #[test]
    fn compare_two_hands6() {
        let mut player = hand::Hand::from_vec(vec!["6s", "Ad", "7h", "4s", "As"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::Pair(Face::Ace, Face::Seven));
        let mut opponent = hand::Hand::from_vec(vec!["Ah", "Ac", "5h", "6h", "9s"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::Pair(Face::Ace, Face::Nine));
        assert_eq!(opponent_rank > player_rank, true);
    }

    #[test]
    fn compare_two_hands7() {
        let mut player = hand::Hand::from_vec(vec!["2s", "3h", "6h", "7s", "9c"]);
        let player_rank = player.rank();
        assert_eq!(player_rank, Rank::HighCard(Face::Nine));
        let mut opponent = hand::Hand::from_vec(vec!["7h", "3c", "10h", "6h", "9s"]);
        let opponent_rank = opponent.rank();
        assert_eq!(opponent_rank, Rank::HighCard(Face::Ten));
        assert_eq!(opponent_rank > player_rank, true);
    }

    #[test]
    fn test_sort_ranks() {
        let mut ranks = vec![
            Rank::RoyalFlush,
            Rank::StraightFlush(Face::Nine),
            Rank::FourOfAKind(Face::Ace),
            Rank::FullHouse,
            Rank::Flush(Face::Nine),
            Rank::Straight,
            Rank::Pair(Face::Ace, Face::Seven),
            Rank::ThreeOfAKind,
            Rank::TwoPairs,
            Rank::Pair(Face::Ace, Face::Nine),
            Rank::HighCard(Face::Nine),
        ];
        ranks.sort();
        assert_eq!(
            ranks,
            vec![
                Rank::HighCard(Face::Nine),
                Rank::Pair(Face::Ace, Face::Seven),
                Rank::Pair(Face::Ace, Face::Nine),
                Rank::TwoPairs,
                Rank::ThreeOfAKind,
                Rank::Straight,
                Rank::Flush(Face::Nine),
                Rank::FullHouse,
                Rank::FourOfAKind(Face::Ace),
                Rank::StraightFlush(Face::Nine),
                Rank::RoyalFlush,
            ]
        )
    }
}
