type CardFlag = u32;

// The first two bits say how many, and the rest identify the suit
const ACE: CardFlag   = 0b00000000000000000000000001;
const TWO: CardFlag   = 0b00000000000000000000000100;
const THREE: CardFlag = 0b00000000000000000000010000;
const FOUR: CardFlag  = 0b00000000000000000001000000;
const FIVE: CardFlag  = 0b00000000000000000100000000;
const SIX: CardFlag   = 0b00000000000000010000000000;
const SEVEN: CardFlag = 0b00000000000001000000000000;
const EIGHT: CardFlag = 0b00000000000100000000000000;
const NINE: CardFlag  = 0b00000000010000000000000000;
const TEN: CardFlag   = 0b00000001000000000000000000;
const JACK: CardFlag  = 0b00000100000000000000000000;
const QUEEN: CardFlag = 0b00010000000000000000000000;
const KING: CardFlag  = 0b01000000000000000000000000;

const fn create_deck() -> [CardFlag; 52] {
    let deck: [CardFlag; 52] = [
        ACE, ACE, ACE, ACE,
        TWO, TWO, TWO, TWO,
        THREE, THREE, THREE, THREE,
        FOUR, FOUR, FOUR, FOUR,
        FIVE, FIVE, FIVE, FIVE,
        SIX, SIX, SIX, SIX,
        SEVEN, SEVEN, SEVEN, SEVEN,
        EIGHT, EIGHT, EIGHT, EIGHT,
        NINE, NINE, NINE, NINE,
        TEN, TEN, TEN, TEN,
        JACK, JACK, JACK, JACK,
        QUEEN, QUEEN, QUEEN, QUEEN,
        KING, KING, KING, KING,
    ];

    return deck;
}

// This counts how many cards of a specific rank are in the hand
fn count_rank(hand: CardFlag, rank: CardFlag) -> u8 {
    return ((hand & (3*rank)) >> rank.trailing_zeros()) as u8;
}

struct Player {
    hand: CardFlag
}

impl Player {
    fn take_turn(&mut self, draw: CardFlag) -> (CardFlag, bool) {
        match count_rank(self.hand, draw) {
            0 => (draw, false),
            1 => {

            },
            2 => {

            },
            3 => {

            }
            _ => (draw, false)
        }
    }
}

fn spoons_sim(num_players: u8, rng: &mut ThreadRng) -> u16 {
    let mut turns: u16 = 0;
    let mut deck = create_deck();
    deck.shuffle(rng);

    todo!();

    return turns;
}

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
fn main() {

}