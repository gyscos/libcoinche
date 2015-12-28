//! This module represents a basic, rule-agnostic 32-cards system.

use ::rand::{
    thread_rng,
    Rng,
    IsaacRng,
    SeedableRng
};
use std::string::ToString;
use rustc_serialize;

/// One of the four Suits: Heart, Spade, Diamond, Club.
#[derive(PartialEq,Clone,Copy,Debug,Hash)]
pub enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

impl rustc_serialize::Encodable for Suit {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(match self {
            &Suit::Heart => "Heart",
            &Suit::Spade => "Spade",
            &Suit::Club => "Club",
            &Suit::Diamond => "Diamond",
        })
    }
}

impl Suit {
    /// Returns the suit corresponding to the number:
    ///
    /// * `0`: Heart
    /// * `1`: Spade
    /// * `2`: Diamond
    /// * `3`: Club
    pub fn from_id(n: u32) -> Option<Self> {
        match n {
            0 => Some(Suit::Heart),
            1 => Some(Suit::Spade),
            2 => Some(Suit::Diamond),
            3 => Some(Suit::Club),
            _ => None
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            &Suit::Heart => 0,
            &Suit::Spade => 1,
            &Suit::Diamond => 2,
            &Suit::Club => 3,
        }
    }
}

impl ToString for Suit {
    /// Returns a UTF-8 character representing the suit.
    fn to_string(&self) -> String {
        match self {
            &Suit::Heart => "♥",
            &Suit::Spade => "♠",
            &Suit::Diamond => "♦",
            &Suit::Club => "♣",
        }.to_string()
    }
}


/// Rank of a card in a suit.
#[derive(PartialEq,Clone,Copy,Debug,Hash,RustcEncodable)]
pub enum Rank {
    Rank7,
    Rank8,
    Rank9,
    RankJ,
    RankQ,
    RankK,
    RankX,
    RankA,
}

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
    pub fn from_id(n: u32) -> Option<Self> {
        match n {
            0 => Some(Rank::Rank7),
            1 => Some(Rank::Rank8),
            2 => Some(Rank::Rank9),
            3 => Some(Rank::RankJ),
            4 => Some(Rank::RankQ),
            5 => Some(Rank::RankK),
            6 => Some(Rank::RankX),
            7 => Some(Rank::RankA),
            _ => None,
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            &Rank::Rank7 => 0,
            &Rank::Rank8 => 1,
            &Rank::Rank9 => 2,
            &Rank::RankJ => 3,
            &Rank::RankQ => 4,
            &Rank::RankK => 5,
            &Rank::RankX => 6,
            &Rank::RankA => 7,
        }
    }
}

impl ToString for Rank {
    /// Returns a character representing the given rank.
    fn to_string(&self) -> String {
        match self {
            &Rank::Rank7 => "7",
            &Rank::Rank8 => "8",
            &Rank::Rank9 => "9",
            &Rank::RankJ => "J",
            &Rank::RankQ => "Q",
            &Rank::RankK => "K",
            &Rank::RankX => "X",
            &Rank::RankA => "A",
        }.to_string()
    }
}

/// Represents a single card.
#[derive(PartialEq,Clone,Copy,Debug,Hash,RustcEncodable)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// TODO: Add card constants? (8 of heart, Queen of spades, ...?)

impl Card {
    /// Returns the card number (from 0 to 31)
    pub fn id(&self) -> u32 {
        8 * self.suit.id() + self.rank.id()
    }

    /// Returns the card corresponding to the given number.
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            n if n < 32 => {
                let suit = Suit::from_id(n / 8).unwrap();
                let rank = Rank::from_id(n % 8).unwrap();
                Some(Card::new(suit, rank))
            },
            _ => None,
        }
    }

    /// Creates a card from the given suit and rank
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit: suit,
            rank: rank,
        }
    }
}

impl ToString for Card {
    /// Returns a string representation of the card.
    fn to_string(&self) -> String {
        self.rank.to_string() + &self.suit.to_string()
    }

}


/// Represents an unordered set of cards
#[derive(PartialEq,Clone,Debug,RustcEncodable)]
pub struct Hand(Vec<Card>);

impl Hand {
    /// Returns an empty hand.
    pub fn new() -> Self {
        Hand(Vec::new())
    }

    pub fn make_4() -> [Hand; 4] {
        [Hand::new(),
         Hand::new(),
         Hand::new(),
         Hand::new()]
    }

    pub fn clone(hands: &[Hand; 4]) -> [Hand; 4] {
        [hands[0].clone(),
         hands[1].clone(),
         hands[2].clone(),
         hands[3].clone()]
    }

    /// Add `card` to `self`.
    ///
    /// No effect if `self` already contains `card`.
    pub fn add(&mut self, card: Card) -> &mut Hand {
        self.0.push(card);
        self
    }

    /// Removes `card` from `self`.
    ///
    /// No effect if `self` does not contains `card`.
    pub fn remove(&mut self, card: Card) -> &mut Hand {
        self.0.retain(|c| *c != card);
        self
    }

    /// Remove all cards from `self`.
    pub fn clean(&mut self) {
        self.0.clear();
    }

    /// Returns `true` if `self` contains `card`.
    pub fn has(&self, card: Card) -> bool {
        self.0.contains(&card)
    }

    /// Returns `true` if the hand contains any card of the given suit.
    pub fn has_any(&self, suit: Suit) -> bool {
        for c in self.0.iter() {
            if c.suit == suit { return true; }
        }
        false
    }

    /// Returns `true` if `self` contains no card.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a card from `self`.
    ///
    /// Returns an invalid card if `self` is empty.
    pub fn get_card(&self) -> Option<Card> {
        self.0.first().map(|c| *c)
    }

    /// Returns the cards contained in `self` as a `Vec`.
    pub fn list(&self) -> &[Card] {
        &self.0
    }

    /// Returns the number of cards in `self`.
    pub fn size(&self) -> usize {
        self.0.len()
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
            d.cards.push(Card::from_id(i).unwrap());
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
            let card = Card::from_id(i).unwrap();
            assert!(i == card.id());
        }

        for s in 0..4 {
            let suit = Suit::from_id(s).unwrap();
            for r in 0..8 {
                let rank = Rank::from_id(r).unwrap();
                let card = Card::new(suit, rank);
                assert!(card.rank == rank);
                assert!(card.suit == suit);
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
    use super::*;
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
            let mut hands = Hand::clone(&hands);
            for hand in hands.iter_mut() {
                let list = hand.list().to_vec();
                for c in list {
                    hand.remove(c);
                }
            }
        });
    }
}
