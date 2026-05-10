use crate::card_and_deck::Card;

use crate::poker_error::PokerError;

pub const MINIMUM_BET: u32 = 10;

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

impl std::fmt::Display for TablePositions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TablePositions::Button => write!(f, "(Button)"),
            TablePositions::SmallBlind => write!(f, "(Small Blind)"),
            TablePositions::BigBlind => write!(f, "(Big Blind)"),
            TablePositions::Standard => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub name: String,
    pub position: TablePositions,
    pub chips: u32,
    pub player_current_bet: u32,
}

impl Player {
    pub fn new(name: String, position: TablePositions, chips: u32) -> Player {
        return Player {
            hand: Vec::with_capacity(2),
            name,
            position,
            chips,
            player_current_bet: 0,
        };
    }
    pub fn fold(&mut self, discard_pile: &mut Vec<Card>) {
        discard_pile.append(&mut self.hand);
        self.hand.clear();
    }
    pub fn call(
        &mut self,
        current_highest_bet: &mut u32,
        pot: &mut u32,
    ) -> Result<u32, PokerError> {
        let amount = *current_highest_bet - self.player_current_bet;
        if amount > self.chips {
            return Err(PokerError::InsufficientChips);
        }
        self.player_current_bet = *current_highest_bet;
        self.chips -= amount;
        *pot += amount;
        Ok(amount)
    }
    pub fn raise(
        &mut self,
        amount: u32,
        current_highest_bet: &mut u32,
        pot: &mut u32,
    ) -> Result<u32, PokerError> {
        // A raise is a relative amount MORE than the current highest bet in the round.
        if amount < *current_highest_bet + MINIMUM_BET {
            return Err(PokerError::BetTooLow(*current_highest_bet + MINIMUM_BET));
        }
        let total_bet = amount + *current_highest_bet;
        if total_bet > self.chips {
            return Err(PokerError::InsufficientChips);
        }
        self.player_current_bet = total_bet;
        self.chips -= total_bet;
        *pot += total_bet;
        *current_highest_bet = total_bet;
        Ok(total_bet)
    }
}
