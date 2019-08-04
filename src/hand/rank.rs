use std::cmp::Ordering;

//https://www.fgbradleys.com/et_poker.asp

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Ord, Eq)]
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
            (Rank::HighCard, _) => Some(Ordering::Less),      //min
            (Rank::RoyalFlush, _) => Some(Ordering::Greater), //max

            (Rank::StraightFlush, Rank::FourOfAKind) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::StraightFlush, Rank::FullHouse) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::Flush) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::Straight) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::ThreeOfAKind) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::Pair) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::HighCard) => Some(Ordering::Greater),
            (Rank::StraightFlush, Rank::StraightFlush) => Some(Ordering::Equal),

            //FourOfAKind
            (Rank::FourOfAKind, Rank::FullHouse) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::FourOfAKind, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::FourOfAKind, Rank::Flush) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::Straight) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::ThreeOfAKind) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::Pair) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::HighCard) => Some(Ordering::Greater),
            (Rank::FourOfAKind, Rank::FourOfAKind) => Some(Ordering::Equal),

            //FullHouse
            (Rank::FullHouse, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::FullHouse, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::FullHouse, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::FullHouse, Rank::Flush) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::Straight) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::ThreeOfAKind) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::Pair) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::HighCard) => Some(Ordering::Greater),
            (Rank::FullHouse, Rank::FullHouse) => Some(Ordering::Equal),

            //Flush
            (Rank::Flush, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::Flush, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::Flush, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::Flush, Rank::FullHouse) => Some(Ordering::Less),
            (Rank::Flush, Rank::Flush) => Some(Ordering::Equal),
            (Rank::Flush, Rank::Straight) => Some(Ordering::Greater),
            (Rank::Flush, Rank::ThreeOfAKind) => Some(Ordering::Greater),
            (Rank::Flush, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::Flush, Rank::Pair) => Some(Ordering::Greater),
            (Rank::Flush, Rank::HighCard) => Some(Ordering::Greater),

            //Straight
            (Rank::Straight, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::Straight, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::Straight, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::Straight, Rank::FullHouse) => Some(Ordering::Less),
            (Rank::Straight, Rank::Flush) => Some(Ordering::Less),
            (Rank::Straight, Rank::Straight) => Some(Ordering::Equal),
            (Rank::Straight, Rank::ThreeOfAKind) => Some(Ordering::Greater),
            (Rank::Straight, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::Straight, Rank::Pair) => Some(Ordering::Greater),
            (Rank::Straight, Rank::HighCard) => Some(Ordering::Greater),

            //ThreeOfAKind
            (Rank::ThreeOfAKind, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::FullHouse) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::Flush) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::Straight) => Some(Ordering::Less),
            (Rank::ThreeOfAKind, Rank::ThreeOfAKind) => Some(Ordering::Equal),
            (Rank::ThreeOfAKind, Rank::TwoPairs) => Some(Ordering::Greater),
            (Rank::ThreeOfAKind, Rank::Pair) => Some(Ordering::Greater),
            (Rank::ThreeOfAKind, Rank::HighCard) => Some(Ordering::Greater),

            //TwoPairs
            (Rank::TwoPairs, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::FullHouse) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::Flush) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::Straight) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::ThreeOfAKind) => Some(Ordering::Less),
            (Rank::TwoPairs, Rank::TwoPairs) => Some(Ordering::Equal),
            (Rank::TwoPairs, Rank::Pair) => Some(Ordering::Greater),
            (Rank::TwoPairs, Rank::HighCard) => Some(Ordering::Greater),


            //Pair
            (Rank::Pair, Rank::RoyalFlush) => Some(Ordering::Less),
            (Rank::Pair, Rank::StraightFlush) => Some(Ordering::Less),
            (Rank::Pair, Rank::FourOfAKind) => Some(Ordering::Less),
            (Rank::Pair, Rank::FullHouse) => Some(Ordering::Less),
            (Rank::Pair, Rank::Flush) => Some(Ordering::Less),
            (Rank::Pair, Rank::Straight) => Some(Ordering::Less),
            (Rank::Pair, Rank::ThreeOfAKind) => Some(Ordering::Less),
            (Rank::Pair, Rank::TwoPairs) => Some(Ordering::Less),
            (Rank::Pair, Rank::Pair) => Some(Ordering::Equal),
            (Rank::Pair, Rank::HighCard) => Some(Ordering::Greater),

        }
    }
}