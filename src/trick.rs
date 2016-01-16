//! This module implements a trick in a game of coinchet vec

use super::pos;
use super::cards;
use super::points;

/// The current cards on the table
#[derive(Clone,RustcEncodable,Debug)]
pub struct Trick {
    /// Cards currently on the table (they are invalid until played).
    pub cards: [Option<cards::Card>; 4],
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
            cards: [None; 4],
        }
    }

    /// Returns the points value of this trick
    pub fn score(&self, trump: cards::Suit) -> i32 {
        self.cards
            .iter()
            .map(|c| c.map_or(0, |c| points::score(c, trump)))
            .fold(0, |a, b| a + b)
    }

    /// Plays a card.
    ///
    /// Updates the winner
    /// Returns `true` if this completes the trick.
    pub fn play_card(&mut self,
                     player: pos::PlayerPos,
                     card: cards::Card,
                     trump: cards::Suit)
                     -> bool {
        self.cards[player as usize] = Some(card);
        if player == self.first {
            return false;
        }

        if points::strength(card, trump) >
           points::strength(self.cards[self.winner as usize].unwrap(), trump) {
            self.winner = player
        }

        (player == self.first.prev())
    }

    /// Returns the starting suit for this trick.
    ///
    /// Returns None if the trick hasn't started yet.
    pub fn suit(&self) -> Option<cards::Suit> {
        self.cards[self.first as usize].map(|c| c.suit())
    }
}
