//! Manage points and scores

use super::cards;

/// Returns the number of points `card` is worth, with the current trump suit.
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
        cards::Rank::RankJ => 20,
        cards::Rank::Rank9 => 14,
        _ => usual_score(rank),
    }
}

/// Returns the score for the given rank when it is not the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn usual_score(rank: cards::Rank) -> i32 {
    match rank {
        cards::Rank::Rank7 | cards::Rank::Rank8 | cards::Rank::Rank9 => 0,
        cards::Rank::RankJ => 2,
        cards::Rank::RankQ => 3,
        cards::Rank::RankK => 4,
        cards::Rank::RankX => 10,
        cards::Rank::RankA => 11,
    }
}

/// Returns the strength for the given rank when it is the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn trump_strength(rank: cards::Rank) -> i32 {
    match rank {
        cards::Rank::Rank7 => 0,
        cards::Rank::Rank8 => 1,
        cards::Rank::RankQ => 2,
        cards::Rank::RankK => 3,
        cards::Rank::RankX => 4,
        cards::Rank::RankA => 5,
        cards::Rank::Rank9 => 6,
        cards::Rank::RankJ => 7,
    }
}

/// Returns the strength for the given rank when it is not the trump.
///
/// # Panics
/// If `rank` is invalid.
pub fn usual_strength(rank: cards::Rank) -> i32 {
    match rank {
        cards::Rank::Rank7 => 0,
        cards::Rank::Rank8 => 1,
        cards::Rank::Rank9 => 2,
        cards::Rank::RankJ => 3,
        cards::Rank::RankQ => 4,
        cards::Rank::RankK => 5,
        cards::Rank::RankX => 6,
        cards::Rank::RankA => 7,
    }
}
