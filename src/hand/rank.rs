//use std::cmp::Ordering;

//https://www.fgbradleys.com/et_poker.asp

//!  - Inner line doc
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
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_ranks() {
        let mut ranks = vec![
            Rank::RoyalFlush,
            Rank::StraightFlush,
            Rank::FourOfAKind,
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
                Rank::FourOfAKind,
                Rank::StraightFlush,
                Rank::RoyalFlush,
            ]
        )
    }

}