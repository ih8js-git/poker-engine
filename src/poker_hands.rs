use crate::card_and_deck::{Card, Ranks, Suits};
use std::collections::HashMap;

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
    pub fn check_dupes_of_a_kind(cards: &[Card; 7]) -> (Option<PokerHands>, Option<Vec<Card>>) {
        let mut rank_counts = HashMap::new();
        for card in cards {
            rank_counts
                .entry(card.rank)
                .or_insert(Vec::new())
                .push(card.clone());
        }

        for count in [4, 3, 2] {
            let mut best_rank: Option<Ranks> = None;
            let mut best_cards: Option<Vec<Card>> = None;

            for (rank, matching_cards) in &rank_counts {
                if matching_cards.len() == count {
                    if best_rank.is_none() || rank.to_int() > best_rank.as_ref().unwrap().to_int() {
                        best_rank = Some(*rank);
                        best_cards = Some(matching_cards.clone());
                    }
                }
            }

            if let Some(cards) = best_cards {
                let hand = match count {
                    4 => PokerHands::FourOfAKind,
                    3 => PokerHands::ThreeOfAKind,
                    2 => PokerHands::Pair,
                    _ => unreachable!(),
                };
                return (Some(hand), Some(cards));
            }
        }
        (None, None)
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
    pub fn is_flush(cards: &[Card; 7]) -> (Option<PokerHands>, Option<Vec<Card>>) {
        let mut hearts = Vec::new();
        let mut diamonds = Vec::new();
        let mut clubs = Vec::new();
        let mut spades = Vec::new();

        for card in cards {
            match card.suit {
                Suits::Hearts => hearts.push(card.clone()),
                Suits::Diamonds => diamonds.push(card.clone()),
                Suits::Clubs => clubs.push(card.clone()),
                Suits::Spades => spades.push(card.clone()),
            }
        }

        if hearts.len() >= 5 {
            return (Some(PokerHands::Flush), Some(hearts));
        }
        if diamonds.len() >= 5 {
            return (Some(PokerHands::Flush), Some(diamonds));
        }
        if clubs.len() >= 5 {
            return (Some(PokerHands::Flush), Some(clubs));
        }
        if spades.len() >= 5 {
            return (Some(PokerHands::Flush), Some(spades));
        }

        (None, None)
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

        let (dupe_hand, dupe_cards) = Self::check_dupes_of_a_kind(cards);
        if let Some(FourOfAKind) = dupe_hand {
            return (FourOfAKind, dupe_cards);
        }
        if Self::is_full_house(cards) {
            return (FullHouse, None);
        }
        let (flush_hand, flush_cards) = Self::is_flush(cards);
        if let Some(hand) = flush_hand {
            return (hand, flush_cards);
        }

        if let Some(hand) = straight_hand {
            return (hand, straight_cards);
        }

        if let Some(ThreeOfAKind) = dupe_hand {
            return (ThreeOfAKind, dupe_cards);
        }
        if Self::is_two_pair(cards) {
            return (TwoPair, None);
        }
        if let Some(Pair) = dupe_hand {
            return (Pair, dupe_cards);
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
        assert_eq!(
            PokerHands::check_straights(&not_straight_flush),
            (None, None)
        );
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
                suit: Clubs,
            },
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
                rank: Three,
                suit: Spades,
            },
            Card {
                rank: Four,
                suit: Hearts,
            },
            Card {
                rank: Four,
                suit: Clubs,
            },
            Card {
                rank: Five,
                suit: Hearts,
            },
        ];
        let (hand, sc) = PokerHands::check_straights(&cards);
        assert_eq!(hand, Some(PokerHands::StraightFlush));
        let sc = sc.unwrap();
        assert_eq!(sc[0].rank, Ace);
        assert_eq!(sc[4].rank, Five);
        assert!(sc.iter().all(|c| c.suit == Hearts));
    }
}
