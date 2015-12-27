//! Player position in the table

use rustc_serialize;

/// One of two teams
#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Team(pub usize);

impl rustc_serialize::Encodable for Team {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

impl Team {
    /// Returns the other team
    pub fn opponent(self) -> Team {
        Team(1 - self.0)
    }
}

/// A position in the table
#[derive(PartialEq,Clone,Copy,Debug)]
pub struct PlayerPos(pub usize);

impl rustc_serialize::Encodable for PlayerPos {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

/// Iterates on players
pub struct PlayerIterator {
    current: PlayerPos,
    remaining: usize,
}

impl Iterator for PlayerIterator {
    type Item = PlayerPos;

    fn next(&mut self) -> Option<PlayerPos> {
        if self.remaining == 0 {
            return None;
        }

        let r = self.current;
        self.current = self.current.next();
        self.remaining -= 1;
        Some(r)
    }
}

/// Player 0
pub const P0: PlayerPos = PlayerPos(0);
/// Player 1
pub const P1: PlayerPos = PlayerPos(1);
/// Player 2
pub const P2: PlayerPos = PlayerPos(2);
/// Player 3
pub const P3: PlayerPos = PlayerPos(3);

impl PlayerPos {
    /// Returns the player's team
    pub fn team(self) -> Team {
        Team(self.0 % 2)
    }

    /// Returns `true` if `self` and `other` and in the same team
    pub fn is_partner(self, other: PlayerPos) -> bool {
        self.team() == other.team()
    }

    /// Returns the next player in line
    pub fn next(self) -> PlayerPos {
        if self == P3 {
            P0
        } else {
            PlayerPos(self.0+1)
        }
    }

    /// Returns the player `n` seats further
    pub fn next_n(self, n: usize) -> PlayerPos {
        if n == 0 {
            self
        } else {
            PlayerPos((self.0 + n) % 4)
        }
    }

    /// Returns the previous player.
    pub fn prev(self) -> PlayerPos {
        if self == P0 {
            P3
        } else {
            PlayerPos(self.0 - 1)
        }
    }

    /// Returns an iterator that iterates on `n` players, including this one.
    pub fn until_n(self, n: usize) -> PlayerIterator {
        PlayerIterator {
            current:self,
            remaining: n,
        }
    }

    /// Returns the number of turns after `self` to reach `other`.
    pub fn distance_until(self, other: PlayerPos) -> usize {
        (3 + other.0 - self.0) % 4 + 1
    }

    /// Returns an iterator until the given player (`self` included, `other` excluded)
    pub fn until(self, other: PlayerPos) -> PlayerIterator {
        let d = self.distance_until(other);
        self.until_n(d)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pos() {
        let mut count = [0; 4];
        for i in 0..4 {
            for pos in PlayerPos(i).until(PlayerPos(0)) {
                count[pos.0] += 1;
            }
            for pos in PlayerPos(0).until(PlayerPos(i)) {
                count[pos.0] += 1;
            }
        }

        for c in count.iter() {
            assert!(*c == 5);
        }

        for i in 0..4 {
            assert!(PlayerPos(i).next() == PlayerPos((i+1)%4));
            assert!(PlayerPos(i) == PlayerPos((i+1)%4).prev());
            assert!(PlayerPos(i).next().prev() == PlayerPos(i));
        }
    }
}
