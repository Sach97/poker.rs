use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[allow(dead_code)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}


impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Suit::Clubs => write!(f, "{}", "c"),
            Suit::Diamonds => write!(f, "{}", "d"),
            Suit::Hearts => write!(f, "{}", "h"),
            Suit::Spades => write!(f, "{}", "s"),
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


impl Display for Face {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Face::Two => write!(f, "{}", "2"),
            Face::Three => write!(f, "{}", "3"),
            Face::Four => write!(f, "{}", "4"),
            Face::Five => write!(f, "{}", "5"),
            Face::Six => write!(f, "{}", "6"),
            Face::Seven => write!(f, "{}", "7"),
            Face::Height => write!(f, "{}", "8"),
            Face::Nine => write!(f, "{}", "9"),
            Face::Ten => write!(f, "{}", "10"),
            Face::Jack => write!(f, "{}", "J"),
            Face::Queen => write!(f, "{}", "Q"),
            Face::King => write!(f, "{}", "K"),
            Face::Ace => write!(f, "{}", "A"),
        }
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    face: Face,
}


impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", &self.face.to_string(), &self.suit.to_string())
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
struct Hand {
    cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = "";
        for card in &self.cards {
            f.write_str(str)?;
            f.write_str(&card.to_string())?;
            str = ", ";
        }
        Ok(())

    }
}

#[allow(dead_code)]
fn main() {
    let card = Card {
        suit: Suit::Clubs,
        face: Face::Ten,
    };
    println!("My card : {:?}", card.to_string());
    println!("My card face : {:?}", card.face.to_string());
    println!("My card suit : {:?}", card.suit.to_string());
    //let hand = vec![card];
    let hand = Hand { cards: vec![card] };

    println!("My hand : {:?}", hand.to_string());
}