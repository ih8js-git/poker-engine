use crate::card_and_deck::{Card, Ranks, Suits};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
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

impl std::fmt::Display for PokerHands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PokerHands::RoyalFlush => "Royal Flush",
            PokerHands::StraightFlush => "Straight Flush",
            PokerHands::FourOfAKind => "Four of a Kind",
            PokerHands::FullHouse => "Full House",
            PokerHands::Flush => "Flush",
            PokerHands::Straight => "Straight",
            PokerHands::ThreeOfAKind => "Three of a Kind",
            PokerHands::TwoPair => "Two Pair",
            PokerHands::Pair => "Pair",
            PokerHands::HighCard => "High Card",
        };
        write!(f, "{}", s)
    }
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
    pub fn is_straight_flush(cards: &[Card; 7]) -> bool {
        let mut ranks: Vec<u32> = cards.iter().map(|card| card.rank.to_int()).collect();
        ranks.sort();
        ranks.dedup();

        // Handle Ace-low straight (A, 2, 3, 4, 5)
        // If we have an Ace (14), we also count it as a 1
        if ranks.contains(&14) {
            ranks.push(1);
            ranks.sort();
        }

        if ranks.len() < 5 {
            return false;
        }

        // Check if we have 5 consecutive numbers
        for i in 0..=ranks.len() - 5 {
            if ranks[i + 4] == ranks[i] + 4 {
                // Check if those 5 consecutive numbers are all the same suit
                let mut suits = HashSet::new();
                for card in cards {
                    if card.rank.to_int() >= ranks[i] && card.rank.to_int() <= ranks[i + 4] {
                        suits.insert(card.suit);
                    }
                }
                if suits.len() == 1 {
                    return true;
                }
            }
        }

        false
    }
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
    pub fn is_full_house(cards: &[Card; 7]) -> bool {
        use Ranks::*;
        let mut has_three_of_a_kind = false;
        let mut has_pair = false;

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
                has_three_of_a_kind = true;
            }
            if count == 2 {
                has_pair = true;
            }
        }
        has_three_of_a_kind && has_pair
    }
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
    pub fn is_straight(cards: &[Card; 7]) -> bool {
        // Convert our 7 cards to their int form using Ranks::to_int
        let mut ranks: Vec<u32> = cards.iter().map(|card| card.rank.to_int()).collect();

        // Sort and remove duplicates
        ranks.sort();
        ranks.dedup();

        // Handle Ace-low straight (A, 2, 3, 4, 5)
        // If we have an Ace (14), we also count it as a 1
        if ranks.contains(&14) {
            ranks.push(1);
            ranks.sort();
        }

        if ranks.len() < 5 {
            return false;
        }

        // Check if we have 5 consecutive numbers
        for i in 0..=ranks.len() - 5 {
            if ranks[i + 4] == ranks[i] + 4 {
                return true;
            }
        }

        false
    }
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
    pub fn is_two_pair(cards: &[Card; 7]) -> bool {
        use Ranks::*;
        let mut pairs_found = 0;
        for rank in [
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ] {
            let mut count = 0;
            for card in cards {
                if card.rank == rank {
                    count += 1;
                }
            }
            if count >= 2 {
                pairs_found += 1;
            }
        }
        pairs_found >= 2
    }
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
    pub fn get_best_hand(cards: &[Card; 7]) -> PokerHands {
        use PokerHands::*;
        match () {
            _ if Self::is_royal_flush(cards) => RoyalFlush,
            _ if Self::is_straight_flush(cards) => StraightFlush,
            _ if Self::is_four_of_a_kind(cards) => FourOfAKind,
            _ if Self::is_full_house(cards) => FullHouse,
            _ if Self::is_flush(cards) => Flush,
            _ if Self::is_straight(cards) => Straight,
            _ if Self::is_three_of_a_kind(cards) => ThreeOfAKind,
            _ if Self::is_two_pair(cards) => TwoPair,
            _ if Self::is_pair(cards) => Pair,
            _ => HighCard,
        }
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
    #[test]
    fn test_is_full_house() {
        let full_house_cards = [
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
        assert!(PokerHands::is_full_house(&full_house_cards));

        let not_full_house_cards = [
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
        ];
        assert!(!PokerHands::is_full_house(&not_full_house_cards));
    }
    #[test]
    fn test_is_flush() {
        let flush_cards = [
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
        assert!(PokerHands::is_flush(&flush_cards));
        let not_flush_cards = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Diamonds,
            },
            Card {
                rank: Queen,
                suit: Clubs,
            },
            Card {
                rank: Jack,
                suit: Spades,
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
        assert!(!PokerHands::is_flush(&not_flush_cards));
    }
    #[test]
    fn test_is_two_pair() {
        let is_two_pair = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Ace,
                suit: Diamonds,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Clubs,
            },
            Card {
                rank: Queen,
                suit: Spades,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Clubs,
            },
        ];
        assert!(PokerHands::is_two_pair(&is_two_pair));

        let not_two_pair = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Ace,
                suit: Diamonds,
            },
            Card {
                rank: King,
                suit: Hearts,
            },
            Card {
                rank: Seven,
                suit: Clubs,
            },
            Card {
                rank: Queen,
                suit: Spades,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Clubs,
            },
        ];
        assert!(!PokerHands::is_two_pair(&not_two_pair));

        let three_of_a_kind = [
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
                suit: Hearts,
            },
            Card {
                rank: Queen,
                suit: Spades,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Clubs,
            },
        ];
        assert!(!PokerHands::is_two_pair(&three_of_a_kind));

        let full_house = [
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
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Clubs,
            },
            Card {
                rank: Jack,
                suit: Hearts,
            },
            Card {
                rank: Ten,
                suit: Clubs,
            },
        ];
        assert!(PokerHands::is_two_pair(&full_house));
    }

    #[test]
    fn test_is_straight() {
        let straight_cards = [
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Nine,
                suit: Diamonds,
            },
            Card {
                rank: Eight,
                suit: Clubs,
            },
            Card {
                rank: Seven,
                suit: Spades,
            },
            Card {
                rank: Six,
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
        assert!(PokerHands::is_straight(&straight_cards));

        let ace_low_straight = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Two,
                suit: Diamonds,
            },
            Card {
                rank: Three,
                suit: Clubs,
            },
            Card {
                rank: Four,
                suit: Spades,
            },
            Card {
                rank: Five,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Spades,
            },
            Card {
                rank: Queen,
                suit: Clubs,
            },
        ];
        assert!(PokerHands::is_straight(&ace_low_straight));

        let not_straight = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: King,
                suit: Diamonds,
            },
            Card {
                rank: Queen,
                suit: Clubs,
            },
            Card {
                rank: Jack,
                suit: Spades,
            },
            Card {
                rank: Nine,
                suit: Hearts,
            },
            Card {
                rank: Eight,
                suit: Spades,
            },
            Card {
                rank: Seven,
                suit: Clubs,
            },
        ];
        assert!(!PokerHands::is_straight(&not_straight));
    }
}
