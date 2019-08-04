use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, //royal
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_faces() {
        let mut faces = vec![
            Face::Two,
            Face::Three,
            Face::Four,
            Face::Five,
            Face::Six,
            Face::Seven,
            Face::Eight,
            Face::Nine,
            Face::Ten,
            Face::Jack,
            Face::Queen,
            Face::King,
            Face::Ace,
        ];

        faces.sort();

        assert_eq!(
            faces,
            vec![
                Face::Two,
                Face::Three,
                Face::Four,
                Face::Five,
                Face::Six,
                Face::Seven,
                Face::Eight,
                Face::Nine,
                Face::Ten,
                Face::Jack,
                Face::Queen,
                Face::King,
                Face::Ace,
            ]
        )
    }

}