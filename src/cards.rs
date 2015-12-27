//! This module represents a basic, rule-agnostic 32-cards system.

use ::rand::{
    thread_rng,
    Rng,
    IsaacRng,
    SeedableRng
};
use std::num::Wrapping;
use std::string::ToString;
use rustc_serialize;

/// One of the four Suits: Heart, Spade, Diamond, Club
#[derive(PartialEq,Clone,Copy)]
pub struct Suit(u32);

// TODO: Make these associated const when it's stable

/// The Heart suit
pub const HEART: Suit = Suit(1 << 0);
/// The Spade suit
pub const SPADE: Suit = Suit(1 << 8);
/// The Diamond suit
pub const DIAMOND: Suit = Suit(1 << 16);
/// The Club suit
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
    /// Returns the suit corresponding to the number:
    ///
    /// * `0`: Heart
    /// * `1`: Spade
    /// * `2`: Diamond
    /// * `3`: Club
    ///
    /// # Panics
    ///
    /// If `n >= 4`
    pub fn from_n(n: u32) -> Self {
        if n >= 4 { panic!("Bad suit number"); }
        Suit(1 << 8*n)
    }

    /// Returns a UTF-8 character representing the suit.
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


/// Rank of a card in a suit
#[derive(PartialEq,Clone,Copy)]
pub struct Rank(u32);
/// 7
pub const RANK_7: Rank = Rank(1 << 0);
/// 8
pub const RANK_8: Rank = Rank(1 << 1);
/// 9
pub const RANK_9: Rank = Rank(1 << 2);
/// Jack
pub const RANK_J: Rank = Rank(1 << 3);
/// Queen
pub const RANK_Q: Rank = Rank(1 << 4);
/// King
pub const RANK_K: Rank = Rank(1 << 5);
/// 10
pub const RANK_X: Rank = Rank(1 << 6);
/// Ace
pub const RANK_A: Rank = Rank(1 << 7);
/// Bit mask over all ranks
pub const RANK_MASK: Rank = Rank(255);

impl Rank {

    /// Returns the rank corresponding to the given number:
    ///
    /// * `0`: 7
    /// * `1`: 8
    /// * `2`: 9
    /// * `3`: Jack
    /// * `4`: Queen
    /// * `5`: King
    /// * `6`: 10
    /// * `7`: Ace
    ///
    /// # Panics
    ///
    /// If `n >= 8`
    pub fn from_n(n: u32) -> Self {
        if n >= 8 { panic!("Invalid rank number: {}", n); }
        Rank(1 << n)
    }

    /// Returns a character representing the given rank
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

/// Represents a single card
#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Card(u32);

// TODO: Add card constants? (8 of heart, Queen of spades, ...?)

impl rustc_serialize::Encodable for Card {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}


impl Card {
    /// Returns the card number (from 0 to 31)
    pub fn id(self) -> u32 {
        let mut i = 0;
        let Card(mut v) = self;
        while v != 0 {
            i+=1;
            v = v>>1;
        }

        i-1
    }

    /// Returns an invalid card
    pub fn null() -> Self {
        Card(0)
    }

    /// Returns the card corresponding to the given number
    pub fn from_id(id: u32) -> Self {
        if id > 31 { panic!("invalid card id"); }
        Card(1 << id)
    }

    /// Returns the card's rank
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

    /// Returns the card's suit
    pub fn suit(self) -> Suit {
        let Card(v) = self;
        let Rank(r) = self.rank();
        Suit(v / r)
    }

    /// Returns a string representation of the card
    pub fn to_string(self) -> String {
        let r = self.rank();
        let s = self.suit();
        r.to_string() + &s.to_string()
    }

    /// Creates a card from the given suit and rank
    pub fn new(suit: Suit, rank: Rank) -> Self {
        let Suit(s) = suit;
        let Rank(r) = rank;

        Card(s * r)
    }
}


/// Represents an unordered set of cards
#[derive(PartialEq,Clone,Copy)]
pub struct Hand(u32);

impl rustc_serialize::Encodable for Hand {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

impl Hand {
    /// Returns an empty hand.
    pub fn new() -> Self {
        Hand(0)
    }

    /// Add `card` to `self`.
    ///
    /// No effect if `self` already contains `card`.
    pub fn add(&mut self, card: Card) -> &mut Hand {
        self.0 |= card.0;
        self
    }

    /// Removes `card` from `self`.
    ///
    /// No effect if `self` does not contains `card`.
    pub fn remove(&mut self, card: Card) {
        self.0 &= !card.0;
    }

    /// Remove all cards from `self`.
    pub fn clean(&mut self) {
        *self = Hand::new();
    }

    /// Returns `true` if `self` contains `card`.
    pub fn has(self, card: Card) -> bool {
        (self.0 & card.0) != 0
    }

    /// Returns `true` if the hand contains any card of the given suit.
    pub fn has_any(self, suit: Suit) -> bool {
        (self.0 & (RANK_MASK.0 * suit.0)) != 0
    }

    /// Returns `true` if `self` contains no card.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns a card from `self`.
    ///
    /// Returns an invalid card if `self` is empty.
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

    /// Returns the cards contained in `self` as a `Vec`.
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

    /// Returns the number of cards in `self`.
    pub fn size(self) -> usize {
        self.list().len()
    }
}

impl ToString for Hand {
    /// Returns a string representation of `self`.
    fn to_string(&self) -> String {
        let mut s = "[".to_string();

        for c in (*self).list().iter() {
            s = s + &c.to_string();
            s = s +",";
        }

        s + "]"
    }
}

/// A deck of cards.
pub struct Deck{
    cards: Vec<Card>,
}


impl Deck {
    /// Returns a full, sorted deck of 32 cards.
    pub fn new() -> Self {
        let mut d = Deck{cards:Vec::with_capacity(32)};

        for i in 0..32 {
            d.cards.push(Card::from_id(i));
        }

        d
    }

    /// Shuffle this deck.
    pub fn shuffle(&mut self) {
        self.shuffle_from(thread_rng());
    }

    /// Shuffle this deck with the given random seed.
    ///
    /// Result is determined by the seed.
    pub fn shuffle_seeded(&mut self, seed: &[u32]) {
        let mut rng = IsaacRng::new_unseeded();
        rng.reseed(seed);
        self.shuffle_from(rng);
    }

    fn shuffle_from<RNG: Rng>(&mut self, mut rng: RNG) {
        rng.shuffle(&mut self.cards[..]);
    }

    /// Draw the top card from the deck.
    ///
    /// # Panics
    /// If `self` is empty.
    pub fn draw(&mut self) -> Card {
        self.cards.pop().expect("deck is empty")
    }

    /// Returns `true` if this deck is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns the number of cards left in this deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Deal `n` cards to each hand.
    ///
    /// # Panics
    /// If `self.len() < 4 * n`
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
}

impl ToString for Deck {
    fn to_string(&self) -> String {
        let mut s = "[".to_string();

        for c in self.cards.iter() {
            s = s + &c.to_string();
            s = s +",";
        }

        s + "]"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards() {
        for i in 0..32 {
            let card = Card::from_id(i);
            assert!(i == card.id());
        }

        for s in 0..4 {
            let suit = Suit::from_n(s);
            for r in 0..8 {
                let rank = Rank::from_n(r);
                let card = Card::new(suit, rank);
                assert!(card.rank() == rank);
                assert!(card.suit() == suit);
            }
        }
    }

    #[test]
    fn test_hand() {
        let mut hand = Hand::new();

        let cards: Vec<Card> = vec![
            Card::new(HEART, RANK_7),
            Card::new(HEART, RANK_8),
            Card::new(SPADE, RANK_9),
            Card::new(SPADE, RANK_J),
            Card::new(CLUB, RANK_Q),
            Card::new(CLUB, RANK_K),
            Card::new(DIAMOND, RANK_X),
            Card::new(DIAMOND, RANK_A),
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

    #[test]
    fn test_deck() {
        let mut deck = Deck::new();
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
}

#[cfg(feature="use_bench")]
mod benchs {
    use ::test::Bencher;
    use ::deal_seeded_hands;

    #[bench]
    fn bench_deal(b: &mut Bencher) {
        let seed = &[1,2,3,4,5];
        b.iter(|| {
            deal_seeded_hands(seed);
        });
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        let seed = &[1,2,3,4,5];
        let hands = deal_seeded_hands(seed);
        b.iter(|| {
            let mut hands = hands.clone();
            for hand in hands.iter_mut() {
                for c in hand.list() {
                    hand.remove(c);
                }
            }
        });
    }
}
