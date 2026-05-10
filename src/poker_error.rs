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
