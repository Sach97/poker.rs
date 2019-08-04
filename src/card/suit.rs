use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Debug,PartialEq,Copy,Clone,Eq)]
pub enum Suit {
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
