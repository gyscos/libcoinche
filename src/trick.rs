//! This module implements a trick in a game of coinchet vec

use super::pos;
use super::cards;
use super::points;

/// The current cards on the table
#[derive(Clone,RustcEncodable,Debug)]
pub struct Trick {
    /// Cards currently on the table (they are invalid until played).
    pub cards: Vec<cards::Card>,
    /// First player in this trick.
    pub first: pos::PlayerPos,
    /// Current winner of the trick (updated after each card).
    pub winner: pos::PlayerPos,
}

impl Trick {
    /// Creates a new, empty trick.
    pub fn new(first: pos::PlayerPos) -> Self {
        Trick {
            first: first,
            winner: first,
            cards: Vec::with_capacity(4),
        }
    }

    /// Returns the points value of this trick
    pub fn score(&self, trump: cards::Suit) -> i32 {
        let mut score = 0;
        for card in self.cards.iter() { score += points::score(*card, trump); }
        score
    }

    /// Computes the winner of the trick
    pub fn winner(&self, trump: cards::Suit, current: pos::PlayerPos) -> pos::PlayerPos {
        let mut best = self.first;
        let mut best_strength = 0;
        // Iterate on every player between the first and the current, excluded
        for pos in self.first.until(current) {
            let strength = points::strength(self.cards[pos.0], trump);
            if strength > best_strength {
                best_strength = strength;
                best = pos;
            }
        }

        best
    }

    /// Plays a card.
    ///
    /// Returns `true` if this completes the trick.
    pub fn play_card(&mut self, player: pos::PlayerPos, card: cards::Card, trump: cards::Suit) -> bool {
        self.cards.push(card);
        if player == self.first {
            return false;
        }

        if points::strength(card, trump) > points::strength(self.cards[self.first.distance_until(self.winner) % 4], trump) {
            self.winner = player
        }

        (player == self.first.prev())
    }
}

