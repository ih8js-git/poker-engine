use crate::card_and_deck::{Card, Ranks, Suits};

pub enum PokerHands {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl PokerHands {
    // This function looks super wasteful, but because of edge cases
    // this is the best way I've found to do it
    pub fn is_royal_flush(cards: &[Card; 7]) -> bool {
        use Ranks::*;
        use Suits::*;

        for suit in [Hearts, Diamonds, Clubs, Spades] {
            let mut has_ten = false;
            let mut has_jack = false;
            let mut has_queen = false;
            let mut has_king = false;
            let mut has_ace = false;

            for card in cards {
                if card.suit == suit {
                    match card.rank {
                        Ten => has_ten = true,
                        Jack => has_jack = true,
                        Queen => has_queen = true,
                        King => has_king = true,
                        Ace => has_ace = true,
                        _ => {}
                    }
                }
            }

            if has_ten && has_jack && has_queen && has_king && has_ace {
                return true;
            }
        }

        false
    }
}
