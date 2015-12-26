use rustc_serialize;

#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Team(pub usize);

impl rustc_serialize::Encodable for Team {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

impl Team {
    pub fn opponent(self) -> Team {
        Team(1 - self.0)
    }
}

#[derive(PartialEq,Clone,Copy,Debug)]
pub struct PlayerPos(pub usize);

impl rustc_serialize::Encodable for PlayerPos {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.0.encode(s)
    }
}

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

pub const P0: PlayerPos = PlayerPos(0);
pub const P1: PlayerPos = PlayerPos(1);
pub const P2: PlayerPos = PlayerPos(2);
pub const P3: PlayerPos = PlayerPos(3);

impl PlayerPos {
    pub fn team(self) -> Team {
        Team(self.0 % 2)
    }

    pub fn is_partner(self, other: PlayerPos) -> bool {
        self.team() == other.team()
    }

    pub fn next(self) -> PlayerPos {
        if self == P3 {
            P0
        } else {
            PlayerPos(self.0+1)
        }
    }

    pub fn next_n(self, n: usize) -> PlayerPos {
        if n == 0 {
            self
        } else {
            self.next().next_n(n-1)
        }
    }

    pub fn prev(self) -> PlayerPos {
        if self == P0 {
            P3
        } else {
            PlayerPos(self.0 - 1)
        }
    }

    pub fn until_n(self, n: usize) -> PlayerIterator {
        PlayerIterator {
            current:self,
            remaining: n,
        }
    }

    pub fn distance_until(self, other: PlayerPos) -> usize {
        (3 + other.0 - self.0) % 4 + 1
    }

    // Iterate on every player between self included and other excluded.
    pub fn until(self, other: PlayerPos) -> PlayerIterator {
        let d = self.distance_until(other);
        self.until_n(d)
    }
}

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
