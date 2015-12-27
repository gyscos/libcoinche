//! This module implements a trick in a game of coinchet vec

use super::pos;
use super::cards;
use super::points;

#[derive(Clone,RustcEncodable)]
pub struct Trick {
    pub cards: [cards::Card; 4],
    pub first: pos::PlayerPos,
    pub winner: pos::PlayerPos,
}

pub fn empty_trick(first: pos::PlayerPos) -> Trick {
    Trick {
        first: first,
        winner: first,
        cards: [cards::Card::null(); 4],
    }
}

impl Trick {
    pub fn score(&self, trump: cards::Suit) -> i32 {
        let mut score = 0;
        for card in self.cards.iter() { score += points::score(*card, trump); }
        score
    }

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

    pub fn play_card(&mut self, player: pos::PlayerPos, card: cards::Card, trump: cards::Suit) -> bool {
        self.cards[player.0] = card;
        if player == self.first {
            return false;
        }

        if points::strength(card, trump) > points::strength(self.cards[self.winner.0], trump) {
            self.winner = player
        }

        (player == self.first.prev())
    }
}

