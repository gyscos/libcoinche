//! Manage score and scores

use super::cards;

/// Returns the number of score `card` is worth, with the current trump suit.
pub fn score(card: cards::Card, trump: cards::Suit) -> i32 {
    let r = card.rank();
    if card.suit() == trump {
        trump_score(r)
    } else {
        usual_score(r)
    }
}

/// Returns the strength of `card`, with the current trump suit.
pub fn strength(card: cards::Card, trump: cards::Suit) -> i32 {
    let r = card.rank();
    if card.suit() == trump {
        8 + trump_strength(r)
    } else {
        usual_strength(r)
    }
}

/// Returns the score for the given rank when it is the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn trump_score(rank: cards::Rank) -> i32 {
    match rank {
        cards::RANK_J => 20,
        cards::RANK_9 => 14,
        _ => usual_score(rank),
    }
}

/// Returns the score for the given rank when it is not the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn usual_score(rank: cards::Rank) -> i32 {
    match rank {
        cards::RANK_7 => 0,
        cards::RANK_8 => 0,
        cards::RANK_9 => 0,
        cards::RANK_J => 2,
        cards::RANK_Q => 3,
        cards::RANK_K => 4,
        cards::RANK_X => 10,
        cards::RANK_A => 11,
        _ => panic!("getting score of invalid card"),
    }
}

/// Returns the strength for the given rank when it is the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn trump_strength(rank: cards::Rank) -> i32 {
    match rank {
        cards::RANK_7 => 0,
        cards::RANK_8 => 1,
        cards::RANK_Q => 2,
        cards::RANK_K => 3,
        cards::RANK_X => 4,
        cards::RANK_A => 5,
        cards::RANK_9 => 6,
        cards::RANK_J => 7,
        _ => panic!("getting strength of invalid card"),
    }
}

/// Returns the strength for the given rank when it is not the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn usual_strength(rank: cards::Rank) -> i32 {
    match rank {
        cards::RANK_7 => 0,
        cards::RANK_8 => 1,
        cards::RANK_9 => 2,
        cards::RANK_J => 3,
        cards::RANK_Q => 4,
        cards::RANK_K => 5,
        cards::RANK_X => 6,
        cards::RANK_A => 7,
        _ => panic!("getting strength of invalid card"),
    }
}
