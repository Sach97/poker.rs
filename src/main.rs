mod card;
use card::Card;
mod hand;
use hand::Hand;

#[allow(dead_code)]
enum Player {
    Folded,
    Ranked,
}

#[allow(dead_code)]
enum Rank {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}


fn main() {
    let card = Card::from_string("2c");
    let cards = vec![card];
    let hand = Hand::new(cards);
    //assert_eq!(card,Card::new(Face::Two, Suit::Clubs) );
    println!("{}", hand);
}