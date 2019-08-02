use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Debug)]
#[allow(dead_code)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl FromStr for Suit {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "c" => Ok(Suit::Clubs),
            "d" => Ok(Suit::Diamonds),
            "h" => Ok(Suit::Hearts),
            "s" => Ok(Suit::Spades),

            _ => Err(()),
        }
    }
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
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}


impl FromStr for Face {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "2" => Ok(Face::Two),
            "3" => Ok(Face::Three),
            "4" => Ok(Face::Four),
            "5" => Ok(Face::Five),
            "6" => Ok(Face::Six),
            "7" => Ok(Face::Seven),
            "8" => Ok(Face::Eight),
            "9" => Ok(Face::Nine),
            "10" => Ok(Face::Ten),
            "J" => Ok(Face::Jack),
            "Q" => Ok(Face::Queen),
            "K" => Ok(Face::King),
            "A" => Ok(Face::Ace),
            _ => Err(()),
        }
    }
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
            Face::Eight => write!(f, "{}", "8"),
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

#[allow(dead_code)]
impl Card {
    fn new(face: Face, suit: Suit) -> Card {
        Card {
            face: face,
            suit: suit,

        }
    }

    fn from_string(card: &str) -> Card {
        let (face, suit) = card.split_at(1);
        Card {
            face: Face::from_str(face).unwrap(),
            suit: Suit::from_str(suit).unwrap(),
        }

    }

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
#[allow(dead_code)]
impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        Hand { cards: cards }
    }
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

fn main() {
    let card = Card::from_string("2c");
    let cards = vec![card];
    let hand = Hand::new(cards);
    //assert_eq!(card,Card::new(Face::Two, Suit::Clubs) );
    println!("{}", hand);
}