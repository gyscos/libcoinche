//! Player position in the table

use rustc_serialize;

/// One of two teams
#[derive(PartialEq,Clone,Copy,Debug,RustcDecodable,RustcEncodable)]
pub struct Team(pub usize);

// impl rustc_serialize::Encodable for Team {
//     fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
//         self.0.encode(s)
//     }
// }

impl Team {
    /// Returns the other team
    pub fn opponent(self) -> Team {
        Team(1 - self.0)
    }
}

/// A position in the table
#[derive(PartialEq,Clone,Copy,Debug)]
pub enum PlayerPos {
    P0,
    P1,
    P2,
    P3,
}

impl rustc_serialize::Encodable for PlayerPos {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for PlayerPos {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_usize()) {
            n @ 0 ... 3 => Ok(PlayerPos::from_n(n)),
            other => Err(d.error(&format!("invalid pos: {}", other))),
        }
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

impl PlayerPos {
    /// Returns the player's team
    pub fn team(self) -> Team {
        Team(self as usize % 2)
    }

    fn from_n(n: usize) -> Self {
        match n {
            0 => PlayerPos::P0,
            1 => PlayerPos::P1,
            2 => PlayerPos::P2,
            3 => PlayerPos::P3,
            other => panic!("invalid pos: {}", other),
        }
    }

    /// Returns `true` if `self` and `other` and in the same team
    pub fn is_partner(self, other: PlayerPos) -> bool {
        self.team() == other.team()
    }

    /// Returns the next player in line
    pub fn next(self) -> PlayerPos {
        match self {
            PlayerPos::P0 => PlayerPos::P1,
            PlayerPos::P1 => PlayerPos::P2,
            PlayerPos::P2 => PlayerPos::P3,
            PlayerPos::P3 => PlayerPos::P0,
        }
    }

    /// Returns the player `n` seats further
    pub fn next_n(self, n: usize) -> PlayerPos {
        if n == 0 {
            self
        } else {
            PlayerPos::from_n((self as usize + n) % 4)
        }
    }

    /// Returns the previous player.
    pub fn prev(self) -> PlayerPos {
        match self {
            PlayerPos::P0 => PlayerPos::P3,
            PlayerPos::P1 => PlayerPos::P0,
            PlayerPos::P2 => PlayerPos::P1,
            PlayerPos::P3 => PlayerPos::P2,
        }
    }

    /// Returns an iterator that iterates on `n` players, including this one.
    pub fn until_n(self, n: usize) -> PlayerIterator {
        PlayerIterator {
            current: self,
            remaining: n,
        }
    }

    /// Returns the number of turns after `self` to reach `other`.
    pub fn distance_until(self, other: PlayerPos) -> usize {
        (3 + other as usize - self as usize) % 4 + 1
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
            for pos in PlayerPos::from_n(i).until(PlayerPos::from_n(0)) {
                count[pos as usize] += 1;
            }
            for pos in PlayerPos::from_n(0).until(PlayerPos::from_n(i)) {
                count[pos as usize] += 1;
            }
        }

        for c in count.iter() {
            assert!(*c == 5);
        }

        for i in 0..4 {
            assert!(PlayerPos::from_n(i).next() == PlayerPos::from_n((i + 1) % 4));
            assert!(PlayerPos::from_n(i) == PlayerPos::from_n((i + 1) % 4).prev());
            assert!(PlayerPos::from_n(i).next().prev() == PlayerPos::from_n(i));
        }
    }
}
