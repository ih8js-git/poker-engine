use poker_engine::card_and_deck::Card;
use poker_engine::poker_hands::PokerHands;
use rand::prelude::SliceRandom;

#[test]
fn test_get_best_hand_with_random_cards() {
    let mut rng = rand::rng();
    let deck = Card::new_deck();
    let mut deck_vec: Vec<Card> = deck.into();
    deck_vec.shuffle(&mut rng);
    
    // Take 7 random cards
    let cards_vec: Vec<Card> = deck_vec.into_iter().take(7).collect();
    let cards: [Card; 7] = cards_vec.try_into().expect("Failed to get 7 cards from deck");
    
    println!("\nGenerated 7 random cards:");
    for (i, card) in cards.iter().enumerate() {
        print!("{}", card);
        if i < 6 {
            print!(", ");
        }
    }
    println!();
    
    let (best_hand, high_card) = PokerHands::get_best_hand(&cards);
    println!("Best hand found: {}", best_hand);
    if let Some(card) = high_card {
        println!("High card: {}", card);
    }
}
