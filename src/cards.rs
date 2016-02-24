//! This module represents a basic, rule-agnostic 32-cards system.

use rand::{thread_rng, Rng, IsaacRng, SeedableRng};
use std::str::FromStr;
use std::num::Wrapping;
use std::string::ToString;
use rustc_serialize;

/// One of the four Suits: Heart, Spade, Diamond, Club.
#[derive(PartialEq,Clone,Copy,Debug)]
#[repr(u32)]
pub enum Suit {
    /// The suit of hearts.
    Heart = 1 << 0,
    /// The suit of spades.
    Spade = 1 << 8,
    /// The suit of diamonds.
    Diamond = 1 << 16,
    /// The suit of clubs.
    Club = 1 << 24,
}

impl rustc_serialize::Encodable for Suit {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as u32).encode(s)
    }
}

impl rustc_serialize::Decodable for Suit {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_u32()) {
            0x00000001 => Ok(Suit::Heart),
            0x00000100 => Ok(Suit::Spade),
            0x00010000 => Ok(Suit::Diamond),
            0x01000000 => Ok(Suit::Club),
            other => Err(d.error(&format!("unknown suit: {}", other))),
        }
    }
}

impl Suit {
    /// Returns the suit corresponding to the number:
    ///
    /// * `0` -> Heart
    /// * `1` -> Spade
    /// * `2` -> Diamond
    /// * `3` -> Club
    ///
    /// # Panics
    ///
    /// If `n >= 4`.
    pub fn from_n(n: u32) -> Self {
        match n {
            0 => Suit::Heart,
            1 => Suit::Spade,
            2 => Suit::Diamond,
            3 => Suit::Club,
            other => panic!("bad suit number: {}", other),
        }
    }

    /// Returns a UTF-8 character representing the suit (♥, ♠, ♦ or ♣).
    pub fn to_string(self) -> String {
        match self {
            Suit::Heart => "♥",
            Suit::Spade => "♠",
            Suit::Diamond => "♦",
            Suit::Club => "♣",
        }
        .to_owned()
    }
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "H" | "h" | "heart" | "Suit::Heart" | "Heart" => Ok(Suit::Heart),
            "C" | "c" | "club" | "Suit::Club" | "Club" => Ok(Suit::Club),
            "S" | "s" | "spade" | "Suit::Spade" | "Spade" => Ok(Suit::Spade),
            "D" | "d" | "diamond" | "Suit::Diamond" | "Diamond" => Ok(Suit::Diamond),
            _ => Err(format!("invalid suit: {}", s)),
        }
    }
}


/// Rank of a card in a suit.
#[derive(PartialEq,Clone,Copy,Debug)]
#[repr(u32)]
pub enum Rank {
    /// 7
    Rank7 = 1 << 0,
    /// 8
    Rank8 = 1 << 1,
    /// 9
    Rank9 = 1 << 2,
    /// Jack
    RankJ = 1 << 3,
    /// Queen
    RankQ = 1 << 4,
    /// King
    RankK = 1 << 5,
    /// 10
    RankX = 1 << 6,
    /// Ace
    RankA = 1 << 7,
}

/// Bit RANK_MASK over all ranks.
const RANK_MASK: u32 = 255;

impl Rank {
    /// Returns the rank corresponding to the given number:
    ///
    /// * `0` -> 7
    /// * `1` -> 8
    /// * `2` -> 9
    /// * `3` -> Jack
    /// * `4` -> Queen
    /// * `5` -> King
    /// * `6` -> 10
    /// * `7` -> Ace
    ///
    /// # Panics
    ///
    /// If `n >= 8`.
    pub fn from_n(n: u32) -> Self {
        match n {
            0 => Rank::Rank7,
            1 => Rank::Rank8,
            2 => Rank::Rank9,
            3 => Rank::RankJ,
            4 => Rank::RankQ,
            5 => Rank::RankK,
            6 => Rank::RankX,
            7 => Rank::RankA,
            other => panic!("invalid rank number: {}", other),
        }
    }

    // Return the enum by its discriminant.
    fn from_discriminant(rank: u32) -> Self {
        match rank {
            1 => Rank::Rank7,
            2 => Rank::Rank8,
            4 => Rank::Rank9,
            8 => Rank::RankJ,
            16 => Rank::RankQ,
            32 => Rank::RankK,
            64 => Rank::RankX,
            128 => Rank::RankA,
            other => panic!("invalid rank discrimant: {}", other),
        }
    }

    /// Returns a character representing the given rank.
    pub fn to_string(self) -> String {
        match self {
            Rank::Rank7 => "7",
            Rank::Rank8 => "8",
            Rank::Rank9 => "9",
            Rank::RankJ => "J",
            Rank::RankQ => "Q",
            Rank::RankK => "K",
            Rank::RankX => "X",
            Rank::RankA => "A",
        }
        .to_owned()
    }
}

/// Represents a single card.
#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Card(u32);

// TODO: Add card constants? (8 of heart, Queen of spades, ...?)
// (As associated constants when it's stable?)

impl rustc_serialize::Encodable for Card {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

impl rustc_serialize::Decodable for Card {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Self, D::Error> {
        Ok(Card(try!(d.read_u32())))
    }
}

impl Card {
    /// Returns the card id (from 0 to 31).
    pub fn id(self) -> u32 {
        let mut i = 0;
        let Card(mut v) = self;
        while v != 0 {
            i += 1;
            v = v >> 1;
        }

        i - 1
    }

    /// Returns the card corresponding to the given id.
    ///
    /// # Panics
    ///
    /// If `id >= 32`
    pub fn from_id(id: u32) -> Self {
        if id > 31 {
            panic!("invalid card id");
        }
        Card(1 << id)
    }

    /// Returns the card's rank.
    pub fn rank(self) -> Rank {
        let suit = self.suit();
        let Card(v) = self;
        Rank::from_discriminant(v / suit as u32)
    }

    /// Returns the card's suit.
    pub fn suit(self) -> Suit {
        let Card(n) = self;
        if n < Suit::Spade as u32 {
            Suit::Heart
        } else if n < Suit::Diamond as u32 {
            Suit::Spade
        } else if n < Suit::Club as u32 {
            Suit::Diamond
        } else {
            Suit::Club
        }
    }

    /// Returns a string representation of the card (ex: "7♦").
    pub fn to_string(self) -> String {
        let r = self.rank();
        let s = self.suit();
        r.to_string() + &s.to_string()
    }

    /// Creates a card from the given suit and rank.
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card(suit as u32 * rank as u32)
    }
}


/// Represents an unordered set of cards.
#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Hand(u32);

impl rustc_serialize::Encodable for Hand {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}


impl rustc_serialize::Decodable for Hand {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Self, D::Error> {
        Ok(Hand(try!(d.read_u32())))
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
        (self.0 & (RANK_MASK * suit as u32)) != 0
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
        let mut s = "[".to_owned();

        for c in &(*self).list() {
            s = s + &c.to_string();
            s = s + ",";
        }

        s + "]"
    }
}

/// A deck of cards.
pub struct Deck {
    cards: Vec<Card>,
}


impl Deck {
    /// Returns a full, sorted deck of 32 cards.
    pub fn new() -> Self {
        let mut d = Deck { cards: Vec::with_capacity(32) };

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
        if self.len() < 4 * n {
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
        let mut s = "[".to_owned();

        for c in &self.cards {
            s = s + &c.to_string();
            s = s + ",";
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
            Card::new(Suit::Heart, Rank::Rank7),
            Card::new(Suit::Heart, Rank::Rank8),
            Card::new(Suit::Spade, Rank::Rank9),
            Card::new(Suit::Spade, Rank::RankJ),
            Card::new(Suit::Club, Rank::RankQ),
            Card::new(Suit::Club, Rank::RankK),
            Card::new(Suit::Diamond, Rank::RankX),
            Card::new(Suit::Diamond, Rank::RankA),
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
    use test::Bencher;
    use deal_seeded_hands;

    #[bench]
    fn bench_deal(b: &mut Bencher) {
        let seed = &[1, 2, 3, 4, 5];
        b.iter(|| {
            deal_seeded_hands(seed);
        });
    }

    #[bench]
    fn bench_list_hand(b: &mut Bencher) {
        let seed = &[1, 2, 3, 4, 5];
        let hands = deal_seeded_hands(seed);
        b.iter(|| {
            for hand in hands.iter() {
                hand.list().len();
            }
        });
    }

    #[bench]
    fn bench_del_add_check(b: &mut Bencher) {
        let seed = &[1, 2, 3, 4, 5];
        let hands = deal_seeded_hands(seed);
        let cards: Vec<_> = hands.iter().map(|h| h.list()).collect();
        b.iter(|| {
            let mut hands = hands.clone();
            for (hand, cards) in hands.iter_mut().zip(cards.iter()) {
                for c in cards.iter() {
                    hand.remove(*c);
                }
            }
            for (hand, cards) in hands.iter_mut().zip(cards.iter()) {
                for c in cards.iter() {
                    hand.add(*c);
                }
            }

            for (hand, cards) in hands.iter_mut().zip(cards.iter()) {
                for c in cards.iter() {
                    if !hand.has(*c) {
                        panic!("Error!");
                    }
                }
            }
        });
    }
}
