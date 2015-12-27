#![cfg_attr(feature = "use_bench", feature(test))]
//! Models a game of [coinche](https://en.wikipedia.org/wiki/Coinche) (a french card game).
//!
//! See [coinched](https://github.com/Gyscos/coinched) for an example of usage.
//!
//! Here is a simple example:
//!
//! ```rust
//! extern crate libcoinche;
//! use libcoinche::{bid,cards,pos};
//!
//! fn main() {
//!     // The first player
//!     let first = pos::P0;
//!
//!     // Start the first phase with an auction
//!     let mut auction = bid::Auction::new(first);
//!
//!     // Check their cards
//!     let hands = auction.hands();
//!
//!     // Players bid or pass
//!     auction.bid(pos::P0, cards::HEART, bid::Target::Contract80).unwrap();
//!     auction.pass(pos::P1).unwrap();
//!     auction.pass(pos::P2).unwrap();
//!     // The result is `Over` when the auction is ready to complete
//!     match auction.pass(pos::P3) {
//!         Ok(bid::AuctionState::Over) => (),
//!         _ => panic!("Should not happen"),
//!     };
//!
//!     // Complete the auction to enter the second phase
//!     let mut game = auction.complete().unwrap();
//!
//!     // Play some cards
//!     game.play_card(pos::P0, hands[0].get_card());
//!     // ...
//! }
//! ```
extern crate rand;
extern crate rustc_serialize;
#[cfg(feature = "use_bench")]
extern crate test;

pub mod bid;
pub mod cards;
pub mod game;
pub mod points;
pub mod pos;
pub mod trick;

// Expose the module or their content directly? Still unsure.

// pub use bid::*;
// pub use cards::*;
// pub use game::*;
// pub use points::*;
// pub use pos::*;
// pub use trick::*;

/// Quick method to get cards for 4 players.
///
/// Deals cards to 4 players randomly.
pub fn deal_hands() -> [cards::Hand; 4] {
    let mut hands = [cards::Hand::new(); 4];

    let mut d = cards::Deck::new();
    d.shuffle();

    d.deal_each(&mut hands, 3);
    d.deal_each(&mut hands, 2);
    d.deal_each(&mut hands, 3);

    hands
}

/// Deal cards for 4 players deterministically.
fn deal_seeded_hands(seed: &[u32]) -> [cards::Hand; 4] {
    let mut hands = [cards::Hand::new(); 4];

    let mut d = cards::Deck::new();
    d.shuffle_seeded(seed);

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

