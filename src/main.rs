use rand;
use rand::prelude::SliceRandom;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TablePosition {
    Button, // Also known as the Dealer
    SmallBlind,
    BigBlind,
    Standard,
}

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub name: String,
    pub position: TablePosition,
    pub chips: u32,
}

impl Player {
    pub fn new(name: String, position: TablePosition, chips: u32) -> Player {
        return Player {
            hand: Vec::with_capacity(2),
            name,
            position,
            chips,
        };
    }
}

fn deal(deck: &mut Deck, players: &mut Vec<Player>) {
    for player in players {}
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
        return deck;
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut deck = Card::new_deck();
    let number_of_players = rand::random_range(3..=6);
    let mut players: Vec<Player> = vec![];

    for player_number in 1..=number_of_players {
        let position = match player_number {
            1 => TablePosition::Button,
            2 => TablePosition::SmallBlind,
            3 => TablePosition::BigBlind,
            _ => TablePosition::Standard,
        };
        players.push(Player::new(
            format!("Player {}", player_number),
            position,
            1000,
        ));
    }

    println!("Dealing cards to {:?}", &players);

    deck.shuffle(&mut rng);

    deal(&mut deck, &mut players);
}
