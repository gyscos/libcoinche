//! Models a game of [coinche](https://en.wikipedia.org/wiki/Coinche) (a french card game).
//!
//! See [coinched](https://github.com/Gyscos/coinched) for an example of usage.

extern crate rand;
extern crate rustc_serialize;

pub use self::game::GameState;
pub use self::bid::{AuctionState,Auction};

pub mod cards;
pub mod bid;
pub mod game;
pub mod points;
pub mod trick;
pub mod pos;

/// Deals cards to 4 players randomly.
///
/// Quick method to get cards for 4 players.
pub fn deal_hands() -> [cards::Hand; 4] {
    let mut hands = [cards::Hand::new(); 4];

    let mut d = cards::Deck::new();
    d.shuffle();

    d.deal_each(&mut hands, 3);
    d.deal_each(&mut hands, 2);
    d.deal_each(&mut hands, 3);

    hands
}

#[test]
fn test_deals() {
    let hands = deal_hands();

    let mut count = [0; 32];
    for hand in hands.iter() {
        assert!(hand.size() == 8);
        for card in hand.list().iter() {
            count[card.id() as usize] += 1;
        }
    }

    for c in count.iter() {
        assert!(*c == 1);
    }
}

