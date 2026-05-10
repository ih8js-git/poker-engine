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
    // pub fn is_straight_flush(cards: &[Card; 7]) -> bool {}
    pub fn is_four_of_a_kind(cards: &[Card; 7]) -> bool {
        use Ranks::*;
        for rank in [
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ] {
            let mut count = 0;
            for card in cards {
                if card.rank == rank {
                    count += 1;
                }
            }
            if count == 4 {
                return true;
            }
        }
        false
    }
    // pub fn is_full_house(cards: &[Card; 7]) -> bool {}
    pub fn is_flush(cards: &[Card; 7]) -> bool {
        let mut num_hearts = 0;
        let mut num_diamonds = 0;
        let mut num_clubs = 0;
        let mut num_spades = 0;

        for card in cards {
            match card.suit {
                Suits::Hearts => num_hearts += 1,
                Suits::Diamonds => num_diamonds += 1,
                Suits::Clubs => num_clubs += 1,
                Suits::Spades => num_spades += 1,
            }
        }

        if num_hearts >= 5 || num_diamonds >= 5 || num_clubs >= 5 || num_spades >= 5 {
            return true;
        }

        false
    }
    // pub fn is_straight(cards: &[Card; 7]) -> bool {}
    pub fn is_three_of_a_kind(cards: &[Card; 7]) -> bool {
        use Ranks::*;
        for rank in [
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ] {
            let mut count = 0;
            for card in cards {
                if card.rank == rank {
                    count += 1;
                }
            }
            if count == 3 {
                return true;
            }
        }
        false
    }
    // pub fn is_two_pair(cards: &[Card; 7]) -> bool {}
    pub fn is_pair(cards: &[Card; 7]) -> bool {
        use Ranks::*;
        for rank in [
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ] {
            let mut count = 0;
            for card in cards {
                if card.rank == rank {
                    count += 1;
                }
            }
            if count == 2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card_and_deck::Ranks::*;
    use crate::card_and_deck::Suits::*;

    #[test]
    fn test_is_royal_flush() {
        let royal_flush_cards = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Two,
                suit: Spades,
            },
            Card {
                rank: Three,
                suit: Clubs,
            },
        ];
        assert!(PokerHands::is_royal_flush(&royal_flush_cards));

        let royal_flush_cards_edge_case = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Spades,
            },
            Card {
                rank: Three,
                suit: Clubs,
            },
        ];
        assert!(PokerHands::is_royal_flush(&royal_flush_cards_edge_case));

        let mixed_hand = [
            Card {
                rank: Two,
                suit: Spades,
            },
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Three,
                suit: Clubs,
            },
        ];
        assert!(PokerHands::is_royal_flush(&mixed_hand));

        let not_royal_flush_cards = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Nine,
                suit: Hearts,
            },
            Card {
                rank: Two,
                suit: Spades,
            },
            Card {
                rank: Three,
                suit: Clubs,
            },
        ];
        assert!(!PokerHands::is_royal_flush(&not_royal_flush_cards));
    }
    #[test]
    fn test_is_four_of_a_kind() {
        let four_of_a_kind_cards = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Ace,
                suit: Diamonds,
            },
            Card {
                rank: Ace,
                suit: Clubs,
            },
            Card {
                rank: Ace,
                suit: Spades,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
        ];
        assert!(PokerHands::is_four_of_a_kind(&four_of_a_kind_cards));

        let not_four_of_a_kind_cards = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Ace,
                suit: Diamonds,
            },
            Card {
                rank: Ace,
                suit: Clubs,
            },
            Card {
                rank: King,
                suit: Spades,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Hearts,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
        ];
        assert!(!PokerHands::is_four_of_a_kind(&not_four_of_a_kind_cards));
    }
}
