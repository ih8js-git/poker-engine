use rand;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;

const MINIMUM_BET: u32 = 10;
const SMALL_BLIND_AMOUNT: u32 = MINIMUM_BET / 2;
const BIG_BLIND_AMOUNT: u32 = MINIMUM_BET;

pub type Deck = VecDeque<Card>;

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

#[derive(PartialEq, Eq)]
pub enum GamePhases {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

pub enum Actions {
    Fold,
    Check,
    Call,
    Raise(u32),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TablePositions {
    Button, // Also known as the Dealer
    SmallBlind,
    BigBlind,
    Standard,
}

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub name: String,
    pub position: TablePositions,
    pub chips: u32,
}

impl Player {
    pub fn new(name: String, position: TablePositions, chips: u32) -> Player {
        return Player {
            hand: Vec::with_capacity(2),
            name,
            position,
            chips,
        };
    }
}

fn deal(deck: &mut Deck, players: &mut Vec<Player>) {
    for player in players {
        for _ in 0..2 {
            let card = deck.pop_front().expect("Ran out of cards");
            player.hand.push(card);
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let deck = Card::new_deck();
    let number_of_players = rand::random_range(3..=6);
    let mut players: Vec<Player> = vec![];

    for player_number in 1..=number_of_players {
        let position = match player_number {
            1 => TablePositions::Button,
            2 => TablePositions::SmallBlind,
            3 => TablePositions::BigBlind,
            _ => TablePositions::Standard,
        };
        players.push(Player::new(
            format!("Player {}", player_number),
            position,
            1000,
        ));
    }

    let mut deck_vec: Vec<Card> = deck.into();
    deck_vec.shuffle(&mut rng);
    let mut deck = VecDeque::from(deck_vec);

    deal(&mut deck, &mut players);

    println!("Dealing cards to {:?}", &players);

    let mut pot: u32 = 0;

    let game_phase: GamePhases = GamePhases::PreFlop;
    let mut big_blind_index: Option<usize> = None;

    for (index, player) in players.iter_mut().enumerate() {
        if game_phase == GamePhases::PreFlop {
            if player.position == TablePositions::SmallBlind {
                player.chips -= SMALL_BLIND_AMOUNT;
                pot += SMALL_BLIND_AMOUNT;
                println!("{} (Small Blind) posts {}", player.name, SMALL_BLIND_AMOUNT);
            } else if player.position == TablePositions::BigBlind {
                player.chips -= BIG_BLIND_AMOUNT;
                pot += BIG_BLIND_AMOUNT;
                big_blind_index = Some(index);
                println!("{} (Big Blind) posts {}", player.name, BIG_BLIND_AMOUNT);
            } else {
                pot += 0;
                println!("{} posts {}", player.name, 0);
            }
        }
    }

    let big_blind_index = big_blind_index.expect("Big Blind not found!");

    println!("Total pot: {}", pot);
    println!("\n--- Betting Round ---");
    for i in 0..players.len() {
        let index = (big_blind_index + 1 + i) % players.len();
        let player = &players[index];
        println!("{}'s turn to act", player.name);
    }
}
