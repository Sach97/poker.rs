

use std::cmp::Ordering;

//https://www.fgbradleys.com/et_poker.asp

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum Rank {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    Pair,
    HighCard,
}


impl PartialOrd for Rank {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match ((*self).clone(), (*other).clone()) {
            (Rank::RoyalFlush, _) => Some(Ordering::Greater), //max
            (Rank::HighCard, _) => Some(Ordering::Less),      //min
            (Rank::StraightFlush, Rank::FourOfAKind) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::FullHouse) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::FullHouse, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::FullHouse, _) => Some(Ordering::Greater),
            (Rank::Flush, Rank::FullHouse) => Some(Ordering::Less),
            (Rank::Flush, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::Flush, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::Flush, _) => Some(Ordering::Greater),
            (Rank::Straight, Rank::ThreeOfAKind) => Some(Ordering::Greater),
            (Rank::Straight, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::Straight, Rank::Pair) => Some(Ordering::Greater),
            (Rank::Straight, _) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::ThreeOfAKind, Rank::Pair) => Some(Ordering::Greater),
            (Rank::ThreeOfAKind, _) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::Pair) => Some(Ordering::Greater),
            (Rank::TwoPairs, Rank::HighCard) => Some(Ordering::Greater),
            (Rank::TwoPairs, _) => Some(Ordering::Less),
            (Rank::Pair, Rank::HighCard) => Some(Ordering::Greater),
            (Rank::Pair, _) => Some(Ordering::Less),
            (Rank::StraightFlush, _) => Some(Ordering::Greater),
            (Rank::FourOfAKind, _) => Some(Ordering::Greater),

        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert!(Rank::RoyalFlush > Rank::StraightFlush)
    }

}