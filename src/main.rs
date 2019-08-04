mod card;
//use card::Card;
mod hand;
use hand::Hand;

#[allow(dead_code)]
enum Player {
    Folded,
    Ranked,
}

//https://www.fgbradleys.com/et_poker.asp
#[allow(dead_code)]
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


fn main() {
    // let card = Card::from_string("2c");
    // let card2 = Card::from_string("2h");
    // let cards = vec![card, card2];
    let hand = Hand::from_vec(vec!["2h", "2c"]);
    println!("{}", hand);
}