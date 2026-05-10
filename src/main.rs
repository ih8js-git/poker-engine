use rand;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;

const MINIMUM_BET: u32 = 10;
const SMALL_BLIND_AMOUNT: u32 = MINIMUM_BET / 2;
const BIG_BLIND_AMOUNT: u32 = MINIMUM_BET;

pub type Deck = VecDeque<Card>;

#[derive(Debug)]
pub enum PokerError {
    BetTooLow(u32),
    InsufficientChips,
    InvalidPhase,
}

impl std::fmt::Display for PokerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PokerError::BetTooLow(min) => write!(f, "Your bet must be at least {} chips.", min),
            PokerError::InsufficientChips => write!(f, "You don't have enough chips for that bet."),
            PokerError::InvalidPhase => write!(
                f,
                "You cannot perform that action in the current game phase."
            ),
        }
    }
}

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
    pub current_bet: u32,
}

impl Player {
    pub fn new(name: String, position: TablePositions, chips: u32) -> Player {
        return Player {
            hand: Vec::with_capacity(2),
            name,
            position,
            chips,
            current_bet: 0,
        };
    }
    pub fn fold(&mut self, discard_pile: &mut Vec<Card>) {
        discard_pile.append(&mut self.hand);
        self.hand.clear();
    }
    pub fn raise(
        &mut self,
        amount: u32,
        current_highest_bet: &mut u32,
        bet_override: bool,
    ) -> Result<u32, PokerError> {
        // A raise is a relative amount MORE than the current highest bet in the round.
        if amount < (*current_highest_bet + MINIMUM_BET) && !bet_override {
            return Err(PokerError::BetTooLow(*current_highest_bet + MINIMUM_BET));
        }
        let total_bet = amount + *current_highest_bet;
        self.current_bet = total_bet;
        self.chips -= total_bet;
        *current_highest_bet = total_bet;
        Ok(total_bet)
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

fn get_userinput_action(player_name: &str, current_bet: u32) -> String {
    println!("Enter your action, {}:", player_name);
    println!("1. Fold");
    println!("2. Check");
    println!("3. Call {}", current_bet);
    println!("4. Raise (minimum 10 more than current bet)");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_userinput_raise(player: &mut Player, current_highest_bet: &mut u32) {
    println!("Enter your raise amount, {}:", player.name);
    println!(
        "Minimum raise amount: {}",
        *current_highest_bet + MINIMUM_BET
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let bet_amount = input.trim().parse().expect("Failed to parse");
    match player.raise(bet_amount, current_highest_bet, false) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            get_userinput_raise(player, current_highest_bet);
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let deck = Card::new_deck();
    let mut discard_pile = Vec::<Card>::with_capacity(10);
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
    let mut current_highest_bet: u32 = 0;

    let game_phase: GamePhases = GamePhases::PreFlop;
    let mut big_blind_index: Option<usize> = None;

    for (index, player) in players.iter_mut().enumerate() {
        if game_phase == GamePhases::PreFlop {
            if player.position == TablePositions::SmallBlind {
                player
                    .raise(SMALL_BLIND_AMOUNT, &mut current_highest_bet, true)
                    .expect("Failed to raise");
                pot += SMALL_BLIND_AMOUNT;
                println!("{} (Small Blind) posts {}", player.name, player.current_bet);
            } else if player.position == TablePositions::BigBlind {
                player
                    .raise(BIG_BLIND_AMOUNT, &mut current_highest_bet, true)
                    .expect("Failed to raise");
                pot += BIG_BLIND_AMOUNT;
                big_blind_index = Some(index);
                println!("{} (Big Blind) posts {}", player.name, BIG_BLIND_AMOUNT);
            } else {
                println!("{} posts {}", player.name, 0);
            }
        }
    }

    let big_blind_index = big_blind_index.expect("Big Blind not found!");

    match game_phase {
        GamePhases::PreFlop => {
            println!("\n--- Pre-Flop ---");
        }
        GamePhases::Flop => {
            println!("\n--- Flop ---");
        }
        GamePhases::Turn => {
            println!("\n--- Turn ---");
        }
        GamePhases::River => {
            println!("\n--- River ---");
        }
        GamePhases::Showdown => {
            println!("\n--- Showdown ---");
        }
    }

    for i in 0..players.len() {
        let index = (big_blind_index + 1 + i) % players.len();
        let player_name = players[index].name.clone();

        println!("\n{}'s turn to act", player_name);
        println!("Pot size: {}", pot);
        println!("Current bet: {}", current_highest_bet);

        let mut action_is_valid = false;
        while !action_is_valid {
            let action = get_userinput_action(&player_name, current_highest_bet);
            match action.as_str() {
                "1" => {
                    players[index].fold(&mut discard_pile);
                    action_is_valid = true;
                }
                "2" => {
                    action_is_valid = true;
                }
                "3" => {
                    players[index]
                        .raise(current_highest_bet, &mut current_highest_bet, true)
                        .expect("Failed to raise");
                    action_is_valid = true;
                }
                "4" => {
                    get_userinput_raise(&mut players[index], &mut current_highest_bet);
                    action_is_valid = true;
                }
                _ => println!("\nInvalid action!\n"),
            }
        }
    }
}
