#[derive(Debug)]
#[allow(dead_code)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn to_str(self) -> &'static str {
        match self {
            Suit::Clubs => "c",
            Suit::Diamonds => "d",
            Suit::Hearts => "h",
            Suit::Spades => "s",
        }
    }

}

#[allow(dead_code)]
#[derive(Debug)]
enum Face {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Height,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Face {
    fn to_str(self) -> &'static str {
        match self {
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Height => "8",
            Face::Nine => "9",
            Face::Ten => "10",
            Face::Jack => "J",
            Face::Queen => "Q",
            Face::King => "K",
            Face::Ace => "A",
        }
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    face: Face,
}

impl Card {
    fn to_string(self) -> String {
        let mut result = String::new();
        result.push_str(self.face.to_str());
        result.push_str(self.suit.to_str());
        return result;
    }
}

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

#[allow(dead_code)]
enum Hand {
    Winner,
}


#[allow(dead_code)]
fn main() {
    let card = Card {
        suit: Suit::Clubs,
        face: Face::Ten,
    };
    //let hand = vec![card];
    println!("My card : {:?}", card.to_string())
}