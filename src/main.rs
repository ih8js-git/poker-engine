#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suits {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Ranks,
    pub suit: Suits,
}

pub type Deck = Vec<Card>;

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
        return deck;
    }
}

fn main() {
    println!("Hello, world!");
    let deck = Card::new_deck();
    println!("{:?}", deck);
}
