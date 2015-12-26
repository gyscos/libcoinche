//! This module represents a basic, rule-agnostic 32-cards system.

extern crate rand;

use self::rand::{thread_rng,Rng};
use std::num::Wrapping;
use rustc_serialize;

#[derive(PartialEq,Clone,Copy)]
pub struct Suit(pub u32);
pub const HEART: Suit = Suit(1 << 0);
pub const SPADE: Suit = Suit(1 << 8);
pub const DIAMOND: Suit = Suit(1 << 16);
pub const CLUB: Suit = Suit(1 << 24);


impl rustc_serialize::Encodable for Suit {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match *self {
            HEART | SPADE | DIAMOND | CLUB => self.0.encode(s),
            _ => "??".encode(s),
        }
    }
}

impl Suit {
    pub fn from_n(n: u32) -> Self {
        if n >= 4 { panic!("Bad suit number"); }
        Suit(1 << 8*n)
    }

    pub fn to_string(self) -> String {
        match self {
            HEART => "♥",
            SPADE => "♠",
            DIAMOND => "♦",
            CLUB => "♣",
            _ => "?",
        }.to_string()
    }
}


#[derive(PartialEq,Clone,Copy)]
pub struct Rank(pub u32);
pub const RANK_7: Rank = Rank(1 << 0);
pub const RANK_8: Rank = Rank(1 << 1);
pub const RANK_9: Rank = Rank(1 << 2);
pub const RANK_J: Rank = Rank(1 << 3);
pub const RANK_Q: Rank = Rank(1 << 4);
pub const RANK_K: Rank = Rank(1 << 5);
pub const RANK_X: Rank = Rank(1 << 6);
pub const RANK_A: Rank = Rank(1 << 7);
pub const RANK_MASK: Rank = Rank(255);

pub fn get_rank(n: u32) -> Rank {
    Rank(1 << n)
}

impl Rank {

    pub fn to_string(self) -> String {
        match self {
            RANK_7 => "7",
            RANK_8 => "8",
            RANK_9 => "9",
            RANK_J => "J",
            RANK_Q => "Q",
            RANK_K => "K",
            RANK_X => "X",
            RANK_A => "A",
            _ => "?",
        }.to_string()
    }
}

#[derive(PartialEq,Clone,Copy)]
pub struct Card(pub u32);

impl rustc_serialize::Encodable for Card {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}


impl Card {
    pub fn id(self) -> u32 {
        let mut i = 0;
        let Card(mut v) = self;
        while v != 0 {
            i+=1;
            v = v>>1;
        }

        i-1
    }

    pub fn from_id(id: u32) -> Self {
        if id > 31 { panic!("invalid card id"); }
        Card(1 << id)
    }

    pub fn rank(self) -> Rank {
        let Card(mut v) = self;
        let mut r: u32 = 0;
        let Rank(mask) = RANK_MASK;

        r |= mask & v;
        v = v >> 8;
        r |= mask & v;
        v = v >> 8;
        r |= mask & v;
        v = v >> 8;
        r |= v;

        Rank(r)
    }

    pub fn suit(self) -> Suit {
        let Card(v) = self;
        let Rank(r) = self.rank();
        Suit(v / r)
    }

    pub fn to_string(self) -> String {
        let r = self.rank();
        let s = self.suit();
        r.to_string() + &s.to_string()
    }

}

pub fn make_card(suit: Suit, rank: Rank) -> Card {
    let Suit(s) = suit;
    let Rank(r) = rank;

    Card(s * r)
}

#[test]
fn card_test() {
    for i in 0..32 {
        let card = Card::from_id(i);
        assert!(i == card.id());
    }

    for s in 0..4 {
        let suit = Suit::from_n(s);
        for r in 0..8 {
            let rank = get_rank(r);
            let card = make_card(suit, rank);
            assert!(card.rank() == rank);
            assert!(card.suit() == suit);
        }
    }
}

#[derive(PartialEq,Clone,Copy)]
pub struct Hand(pub u32);

impl rustc_serialize::Encodable for Hand {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

pub fn new_hand() -> Hand {
    Hand(0)
}

impl Hand {
    pub fn add(&mut self, card: Card) -> &mut Hand {
        self.0 |= card.0;
        self
    }

    pub fn remove(&mut self, card: Card) {
        self.0 &= !card.0;
    }

    pub fn clean(&mut self) {
        *self = new_hand();
    }

    pub fn has(self, card: Card) -> bool {
        (self.0 & card.0) != 0
    }

    pub fn has_any(self, suit: Suit) -> bool {
        (self.0 & (RANK_MASK.0 * suit.0)) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn get_card(self) -> Card {
        if self.is_empty() {
            return Card(0);
        }

        let Hand(h) = self;
        // Finds the rightmost bit, shifted to the left by 1.
        // let n = 1 << (h.trailing_zeroes());
        let n = Wrapping(h ^ (h - 1)) + Wrapping(1);
        if n.0 == 0 {
            // We got an overflow. This means the desired bit it the leftmost one.
            Card::from_id(31)
        } else {
            // We just need to shift it back.
            Card(n.0 >> 1)
        }
    }

    pub fn list(self) -> Vec<Card> {
        let mut cards = Vec::new();
        let mut h = self;

        while !h.is_empty() {
            let c = h.get_card();
            h.remove(c);
            cards.push(c);
        }

        cards
    }

    pub fn size(self) -> usize {
        self.list().len()
    }

    pub fn to_string(self) -> String {
        let mut s = "[".to_string();

        for c in self.list().iter() {
            s = s + &c.to_string();
            s = s +",";
        }

        s + "]"
    }
}

#[test]
fn hand_test() {
    let mut hand = new_hand();

    let cards: Vec<Card> = vec![
        make_card(HEART, RANK_7),
        make_card(HEART, RANK_8),
        make_card(SPADE, RANK_9),
        make_card(SPADE, RANK_J),
        make_card(CLUB, RANK_Q),
        make_card(CLUB, RANK_K),
        make_card(DIAMOND, RANK_X),
        make_card(DIAMOND, RANK_A),
    ];

    assert!(hand.is_empty());

    for card in cards.iter() {
        assert!(!hand.has(*card));
        hand.add(*card);
        assert!(hand.has(*card));
    }

    assert!(hand.size() == cards.len());

    for card in cards.iter() {
        assert!(hand.has(*card));
        hand.remove(*card);
        assert!(!hand.has(*card));
    }
}

pub struct Deck{
    cards: Vec<Card>,
}

// Returns a full, sorted deck of 32 cards.
pub fn new_deck() -> Deck {
    let mut d = Deck{cards:Vec::with_capacity(32)};

    for i in 0..32 {
        d.cards.push(Card::from_id(i));
    }

    d
}

impl Deck {
    pub fn shuffle(&mut self) {
        thread_rng().shuffle(&mut self.cards[..]);
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().expect("deck is empty")
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn deal_each(&mut self, hands: &mut [Hand; 4], n: usize) {
        if self.len() < 4*n {
            panic!("Deck has too few cards!");
        }

        for hand in hands.iter_mut() {
            for _ in 0..n {
                hand.add(self.draw());
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = "[".to_string();

        for c in self.cards.iter() {
            s = s + &c.to_string();
            s = s +",";
        }

        s + "]"
    }
}

#[test]
fn test_deck() {
    let mut deck = new_deck();
    deck.shuffle();

    assert!(deck.len() == 32);

    let mut count = [0; 32];
    while !deck.is_empty() {
        let card = deck.draw();
        count[card.id() as usize] += 1;
    }

    for c in count.iter() {
        assert!(*c == 1);
    }
}


