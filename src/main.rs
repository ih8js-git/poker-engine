mod card_and_deck;
mod player;
mod poker_error;
mod poker_hands;

use card_and_deck::{Card, Deck};
use player::MINIMUM_BET;
use player::{GamePhases, Player, TablePositions};
use rand;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;

const SMALL_BLIND_AMOUNT: u32 = MINIMUM_BET / 2;
const BIG_BLIND_AMOUNT: u32 = MINIMUM_BET;

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

fn get_userinput_raise(player: &mut Player, current_highest_bet: &mut u32, pot: &mut u32) {
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
    match player.raise(bet_amount, current_highest_bet, pot) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            get_userinput_raise(player, current_highest_bet, pot);
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let deck = Card::new_deck();
    let mut discard_pile = Vec::<Card>::with_capacity(10);
    let mut community_cards: Vec<Card> = Vec::with_capacity(5);
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

    let mut pot: u32 = 0;
    let mut current_highest_bet: u32 = 0;

    let game_phase: GamePhases = GamePhases::PreFlop;

    let mut player_forcing_action_index: Option<usize> = None;

    for (index, player) in players.iter_mut().enumerate() {
        if game_phase == GamePhases::PreFlop {
            if player.position == TablePositions::SmallBlind {
                player.current_bet = SMALL_BLIND_AMOUNT;
                player.chips -= SMALL_BLIND_AMOUNT;
                pot += SMALL_BLIND_AMOUNT;
                current_highest_bet = SMALL_BLIND_AMOUNT;
                player_forcing_action_index = Some(index);
                println!("{} (Small Blind) posts {}", player.name, player.current_bet);
            } else if player.position == TablePositions::BigBlind {
                player.current_bet = BIG_BLIND_AMOUNT;
                player.chips -= BIG_BLIND_AMOUNT;
                pot += BIG_BLIND_AMOUNT;
                current_highest_bet = BIG_BLIND_AMOUNT;
                player_forcing_action_index = Some(index);
                println!("{} (Big Blind) posts {}", player.name, BIG_BLIND_AMOUNT);
            } else {
                println!("{} posts {}", player.name, 0);
            }
        }
    }

    let mut player_forcing_action_index =
        player_forcing_action_index.expect("Player forcing action not found!");

    for phase in [
        GamePhases::PreFlop,
        GamePhases::Flop,
        GamePhases::Turn,
        GamePhases::River,
        GamePhases::Showdown,
    ] {
        match phase {
            GamePhases::PreFlop => {
                println!("\n--- Pre-Flop ---");
            }
            GamePhases::Flop => {
                println!("\n--- Flop ---");
                for _ in 0..3 {
                    community_cards.push(deck.pop_front().unwrap());
                }
                println!("Community cards: {:?}", community_cards);
            }
            GamePhases::Turn => {
                println!("\n--- Turn ---");
                community_cards.push(deck.pop_front().unwrap());
                println!("Community cards: {:?}", community_cards);
            }
            GamePhases::River => {
                println!("\n--- River ---");
                community_cards.push(deck.pop_front().unwrap());
                println!("Community cards: {:?}", community_cards);
            }
            GamePhases::Showdown => {
                println!("\n--- Showdown ---");
            }
        }

        let mut current_index = (player_forcing_action_index + 1) % players.len();
        let mut round_over = false;

        while !round_over {
            let player_name = players[current_index].name.clone();

            println!("\n{}'s turn to act", player_name);
            println!("{:?}", players[current_index].hand);
            println!("Pot size: {}", pot);
            println!("Current bet: {}", current_highest_bet);

            let mut action_is_valid = false;
            while !action_is_valid {
                let action = get_userinput_action(&player_name, current_highest_bet);
                match action.as_str() {
                    "1" => {
                        players[current_index].fold(&mut discard_pile);
                        action_is_valid = true;
                    }
                    "2" => {
                        action_is_valid = true;
                    }
                    "3" => {
                        players[current_index]
                            .call(&mut current_highest_bet, &mut pot)
                            .expect("Failed to call");
                        action_is_valid = true;
                    }
                    "4" => {
                        get_userinput_raise(
                            &mut players[current_index],
                            &mut current_highest_bet,
                            &mut pot,
                        );
                        player_forcing_action_index = current_index;
                        action_is_valid = true;
                    }
                    _ => println!("\nInvalid action!\n"),
                }
            }

            current_index = (current_index + 1) % players.len();
            if current_index == player_forcing_action_index {
                round_over = true;
            }
        }
    }
}
