extern crate rand;
extern crate rustc_serialize;

pub use self::game::GameState;
pub use self::bid::{AuctionState,new_auction};

pub mod cards;
pub mod bid;
pub mod game;
pub mod points;
pub mod trick;
pub mod pos;

pub fn deal_hands() -> [cards::Hand; 4] {
    let mut hands = [cards::new_hand(); 4];

    let mut d = cards::new_deck();
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

