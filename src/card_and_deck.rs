use owo_colors::{OwoColorize, Rgb};
use std::collections::VecDeque;
use std::fmt;

pub type Deck = VecDeque<Card>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Suits {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl fmt::Display for Suits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suits::Hearts => write!(f, "{}", "♥".red()),
            Suits::Diamonds => write!(f, "{}", "♦".color(Rgb(255, 165, 0))),
            Suits::Clubs => write!(f, "{}", "♣".blue()),
            Suits::Spades => write!(f, "{}", "♠".bright_black()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ranks {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Ranks {
    pub fn to_int(&self) -> u32 {
        match self {
            Ranks::Ace => 14,
            Ranks::King => 13,
            Ranks::Queen => 12,
            Ranks::Jack => 11,
            Ranks::Ten => 10,
            Ranks::Nine => 9,
            Ranks::Eight => 8,
            Ranks::Seven => 7,
            Ranks::Six => 6,
            Ranks::Five => 5,
            Ranks::Four => 4,
            Ranks::Three => 3,
            Ranks::Two => 2,
        }
    }
}

impl fmt::Display for Ranks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ranks::Ace => "A",
            Ranks::King => "K",
            Ranks::Queen => "Q",
            Ranks::Jack => "J",
            Ranks::Ten => "10",
            Ranks::Nine => "9",
            Ranks::Eight => "8",
            Ranks::Seven => "7",
            Ranks::Six => "6",
            Ranks::Five => "5",
            Ranks::Four => "4",
            Ranks::Three => "3",
            Ranks::Two => "2",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Ranks,
    pub suit: Suits,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as "RankSymbol" e.g. "A♥"
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    pub fn new_deck() -> Deck {
        let mut deck = Vec::with_capacity(52);
        use Ranks::*;
        use Suits::*;

        for suit in [Hearts, Diamonds, Clubs, Spades] {
            for rank in [
                Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
            ] {
                deck.push(Card { rank, suit });
            }
        }
        return VecDeque::from(deck);
    }
}
