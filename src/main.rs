#[derive(Debug, Clone, Copy)]
enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING
}

fn create_deck() -> [Rank; 52] {
    let deck: [Rank; 52] = [
        Rank::ACE, Rank::ACE, Rank::ACE, Rank::ACE,
        Rank::TWO, Rank::TWO, Rank::TWO, Rank::TWO,
        Rank::THREE, Rank::THREE, Rank::THREE, Rank::THREE,
        Rank::FOUR, Rank::FOUR, Rank::FOUR, Rank::FOUR,
        Rank::FIVE, Rank::FIVE, Rank::FIVE, Rank::FIVE,
        Rank::SIX, Rank::SIX, Rank::SIX, Rank::SIX,
        Rank::SEVEN, Rank::SEVEN, Rank::SEVEN, Rank::SEVEN,
        Rank::EIGHT, Rank::EIGHT, Rank::EIGHT, Rank::EIGHT,
        Rank::NINE, Rank::NINE, Rank::NINE, Rank::NINE,
        Rank::TEN, Rank::TEN, Rank::TEN, Rank::TEN,
        Rank::JACK, Rank::JACK, Rank::JACK, Rank::JACK,
        Rank::QUEEN, Rank::QUEEN, Rank::QUEEN, Rank::QUEEN,
        Rank::KING, Rank::KING, Rank::KING, Rank::KING,
    ];

    return deck;
}

use rand::seq::SliceRandom;
use rand::thread_rng;
fn main() {
    let mut rng = thread_rng();
    let mut deck = create_deck();

    deck.shuffle(&mut rng);
    println!("{:?}", deck);
}