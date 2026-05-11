use crate::card_and_deck::{Card, Ranks, Suits};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    pub fn check_straights(cards: &[Card; 7]) -> (Option<PokerHands>, Option<Vec<Card>>) {
        // 1. Check for Straight Flush / Royal Flush
        let mut suit_groups: HashMap<crate::card_and_deck::Suits, Vec<&Card>> = HashMap::new();
        for card in cards {
            suit_groups.entry(card.suit).or_default().push(card);
        }

        let mut best_straight_flush: Option<(PokerHands, Vec<Card>)> = None;

        for (_suit, suit_cards) in suit_groups {
            if suit_cards.len() >= 5 {
                let mut ranks_map = HashMap::new();
                for card in suit_cards {
                    ranks_map.insert(card.rank.to_int(), card);
                }
                // Handle Ace-low
                if let Some(&ace_card) = ranks_map.get(&14) {
                    ranks_map.insert(1, ace_card);
                }

                let mut ranks: Vec<u32> = ranks_map.keys().copied().collect();
                ranks.sort();

                for i in (0..=ranks.len() - 5).rev() {
                    if ranks[i + 4] == ranks[i] + 4 {
                        let sc = vec![
                            ranks_map[&ranks[i]].clone(),
                            ranks_map[&ranks[i + 1]].clone(),
                            ranks_map[&ranks[i + 2]].clone(),
                            ranks_map[&ranks[i + 3]].clone(),
                            ranks_map[&ranks[i + 4]].clone(),
                        ];

                        // Highest rank in the straight
                        let high_rank = ranks[i + 4];
                        let is_royal = high_rank == 14 && ranks[i] == 10;

                        let hand_type = if is_royal {
                            PokerHands::RoyalFlush
                        } else {
                            PokerHands::StraightFlush
                        };

                        if best_straight_flush.is_none()
                            || high_rank > best_straight_flush.as_ref().unwrap().1[4].rank.to_int()
                        {
                            best_straight_flush = Some((hand_type, sc));
                            break; // Found highest for this suit
                        }
                    }
                }
            }
        }

        if let Some((hand, sc)) = best_straight_flush {
            return (Some(hand), Some(sc));
        }

        // 2. Check for regular Straight
        let mut ranks_map = HashMap::new();
        for card in cards {
            // Prefer keeping the first card of each rank encountered, 
            // but for a regular straight the suit doesn't matter.
            ranks_map.entry(card.rank.to_int()).or_insert(card);
        }
        if let Some(&ace_card) = ranks_map.get(&14) {
            ranks_map.entry(1).or_insert(ace_card);
        }

        let mut ranks: Vec<u32> = ranks_map.keys().copied().collect();
        ranks.sort();

        if ranks.len() >= 5 {
            for i in (0..=ranks.len() - 5).rev() {
                if ranks[i + 4] == ranks[i] + 4 {
                    let sc = vec![
                        ranks_map[&ranks[i]].clone(),
                        ranks_map[&ranks[i + 1]].clone(),
                        ranks_map[&ranks[i + 2]].clone(),
                        ranks_map[&ranks[i + 3]].clone(),
                        ranks_map[&ranks[i + 4]].clone(),
                    ];
                    return (Some(PokerHands::Straight), Some(sc));
                }
            }
        }

        (None, None)
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
    pub fn is_high_card(cards: &[Card; 7]) -> Option<Vec<Card>> {
        let mut high_card = None;
        for card in cards {
            match &high_card {
                None => high_card = Some(card.clone()),
                Some(c) => {
                    if card.rank.to_int() > c.rank.to_int() {
                        high_card = Some(card.clone());
                    }
                }
            }
        }
        high_card.map(|card| vec![card])
    }
    pub fn get_best_hand(cards: &[Card; 7]) -> (PokerHands, Option<Vec<Card>>) {
        use PokerHands::*;

        let (straight_hand, straight_cards) = Self::check_straights(cards);
        if let Some(hand) = straight_hand {
            if hand == RoyalFlush || hand == StraightFlush {
                return (hand, straight_cards);
            }
        }

        if Self::is_four_of_a_kind(cards) {
            return (FourOfAKind, None);
        }
        if Self::is_full_house(cards) {
            return (FullHouse, None);
        }
        if Self::is_flush(cards) {
            return (Flush, None);
        }

        if let Some(hand) = straight_hand {
            return (hand, straight_cards);
        }

        if Self::is_three_of_a_kind(cards) {
            return (ThreeOfAKind, None);
        }
        if Self::is_two_pair(cards) {
            return (TwoPair, None);
        }
        if Self::is_pair(cards) {
            return (Pair, None);
        }

        (HighCard, Self::is_high_card(cards))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card_and_deck::Ranks::*;
    use crate::card_and_deck::Suits::*;

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
        assert!(
            PokerHands::check_straights(&straight_cards)
                == (
                    Some(PokerHands::Straight),
                    Some(vec![
                        Card {
                            rank: Six,
                            suit: Hearts,
                        },
                        Card {
                            rank: Seven,
                            suit: Spades,
                        },
                        Card {
                            rank: Eight,
                            suit: Clubs,
                        },
                        Card {
                            rank: Nine,
                            suit: Diamonds,
                        },
                        Card {
                            rank: Ten,
                            suit: Hearts,
                        },
                    ])
                )
        );

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
        assert!(
            PokerHands::check_straights(&ace_low_straight)
                == (
                    Some(PokerHands::Straight),
                    Some(vec![
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
                    ])
                )
        );

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
        assert!(PokerHands::check_straights(&not_straight) == (None, None));
        let get_higher_straight = [
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
                rank: Nine,
                suit: Spades,
            },
            Card {
                rank: Eight,
                suit: Clubs,
            },
        ];
        assert!(
            PokerHands::check_straights(&get_higher_straight)
                == (
                    Some(PokerHands::Straight),
                    Some(vec![
                        Card {
                            rank: Ten,
                            suit: Hearts,
                        },
                        Card {
                            rank: Jack,
                            suit: Spades,
                        },
                        Card {
                            rank: Queen,
                            suit: Clubs,
                        },
                        Card {
                            rank: King,
                            suit: Diamonds,
                        },
                        Card {
                            rank: Ace,
                            suit: Hearts,
                        },
                    ])
                )
        );
    }
    #[test]
    fn test_straight_flush() {
        let straight_flush_cards = [
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Nine,
                suit: Hearts,
            },
            Card {
                rank: Eight,
                suit: Hearts,
            },
            Card {
                rank: Seven,
                suit: Hearts,
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
        assert_eq!(
            PokerHands::check_straights(&straight_flush_cards),
            (
                Some(PokerHands::StraightFlush),
                Some(vec![
                    Card {
                        rank: Six,
                        suit: Hearts
                    },
                    Card {
                        rank: Seven,
                        suit: Hearts
                    },
                    Card {
                        rank: Eight,
                        suit: Hearts
                    },
                    Card {
                        rank: Nine,
                        suit: Hearts
                    },
                    Card {
                        rank: Ten,
                        suit: Hearts
                    }
                ])
            )
        );
        let ace_low_straight_flush_cards = [
            Card {
                rank: Ace,
                suit: Hearts,
            },
            Card {
                rank: Two,
                suit: Hearts,
            },
            Card {
                rank: Three,
                suit: Hearts,
            },
            Card {
                rank: Four,
                suit: Hearts,
            },
            Card {
                rank: Five,
                suit: Hearts,
            },
            Card {
                rank: Six,
                suit: Spades,
            },
            Card {
                rank: Seven,
                suit: Clubs,
            },
        ];
        assert_eq!(
            PokerHands::check_straights(&ace_low_straight_flush_cards),
            (
                Some(PokerHands::StraightFlush),
                Some(vec![
                    Card {
                        rank: Ace,
                        suit: Hearts
                    },
                    Card {
                        rank: Two,
                        suit: Hearts
                    },
                    Card {
                        rank: Three,
                        suit: Hearts
                    },
                    Card {
                        rank: Four,
                        suit: Hearts
                    },
                    Card {
                        rank: Five,
                        suit: Hearts
                    }
                ])
            )
        );
        let not_straight_flush = [
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
                rank: Seven,
                suit: Hearts,
            },
            Card {
                rank: Nine,
                suit: Spades,
            },
            Card {
                rank: Eight,
                suit: Clubs,
            },
        ];
        assert_eq!(PokerHands::check_straights(&not_straight_flush), (None, None));
        let get_higher_straight_flush = [
            Card {
                rank: Ten,
                suit: Hearts,
            },
            Card {
                rank: Nine,
                suit: Hearts,
            },
            Card {
                rank: Eight,
                suit: Hearts,
            },
            Card {
                rank: Seven,
                suit: Hearts,
            },
            Card {
                rank: Six,
                suit: Hearts,
            },
            Card {
                rank: Five,
                suit: Hearts,
            },
            Card {
                rank: Four,
                suit: Hearts,
            },
        ];
        assert_eq!(
            PokerHands::check_straights(&get_higher_straight_flush),
            (
                Some(PokerHands::StraightFlush),
                Some(vec![
                    Card {
                        rank: Six,
                        suit: Hearts
                    },
                    Card {
                        rank: Seven,
                        suit: Hearts
                    },
                    Card {
                        rank: Eight,
                        suit: Hearts
                    },
                    Card {
                        rank: Nine,
                        suit: Hearts
                    },
                    Card {
                        rank: Ten,
                        suit: Hearts
                    }
                ])
            )
        );
    }

    #[test]
    fn test_straight_vs_straight_flush() {
        // Higher Straight (J-high) vs Lower Straight Flush (T-high)
        let cards = [
            Card { rank: Jack, suit: Spades },
            Card { rank: Ten, suit: Hearts },
            Card { rank: Nine, suit: Hearts },
            Card { rank: Eight, suit: Hearts },
            Card { rank: Seven, suit: Hearts },
            Card { rank: Six, suit: Hearts },
            Card { rank: Five, suit: Clubs },
        ];
        // The T-high straight flush should be found
        let (hand, sc) = PokerHands::check_straights(&cards);
        assert_eq!(hand, Some(PokerHands::StraightFlush));
        let sc = sc.unwrap();
        assert_eq!(sc[4].rank, Ten);
        assert_eq!(sc[0].rank, Six);
        assert!(sc.iter().all(|c| c.suit == Hearts));
    }

    #[test]
    fn test_straight_flush_with_extra_suits() {
        // Straight flush A-2-3-4-5 Hearts, but with extra 3 Spades and 4 Clubs
        let cards = [
            Card { rank: Ace, suit: Hearts },
            Card { rank: Two, suit: Hearts },
            Card { rank: Three, suit: Hearts },
            Card { rank: Three, suit: Spades },
            Card { rank: Four, suit: Hearts },
            Card { rank: Four, suit: Clubs },
            Card { rank: Five, suit: Hearts },
        ];
        let (hand, sc) = PokerHands::check_straights(&cards);
        assert_eq!(hand, Some(PokerHands::StraightFlush));
        let sc = sc.unwrap();
        assert_eq!(sc[0].rank, Ace);
        assert_eq!(sc[4].rank, Five);
        assert!(sc.iter().all(|c| c.suit == Hearts));
    }
}
