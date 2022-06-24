//! Auctions and bidding during the first phase of the game.

use std::fmt;
use std::str::FromStr;

use super::cards;
use super::game;
use super::pos;

/// Goal set by a contract.
///
/// Determines the winning conditions and the score on success.
#[derive(Eq, PartialEq, Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Target {
    /// Team must get 80 points
    Contract80,
    /// Team must get 90 points
    Contract90,
    /// Team must get 100 points
    Contract100,
    /// Team must get 110 points
    Contract110,
    /// Team must get 120 points
    Contract120,
    /// Team must get 130 points
    Contract130,
    /// Team must get 140 points
    Contract140,
    /// Team must get 150 points
    Contract150,
    /// Team must get 160 points
    Contract160,
    /// Team must win all tricks
    ContractCapot,
}

impl Target {
    /// Returns the score this target would give on success.
    pub fn score(self) -> i32 {
        match self {
            Target::Contract80 => 80,
            Target::Contract90 => 90,
            Target::Contract100 => 100,
            Target::Contract110 => 110,
            Target::Contract120 => 120,
            Target::Contract130 => 130,
            Target::Contract140 => 140,
            Target::Contract150 => 150,
            Target::Contract160 => 160,
            Target::ContractCapot => 250,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Target::Contract80 => "80",
            Target::Contract90 => "90",
            Target::Contract100 => "100",
            Target::Contract110 => "110",
            Target::Contract120 => "120",
            Target::Contract130 => "130",
            Target::Contract140 => "140",
            Target::Contract150 => "150",
            Target::Contract160 => "160",
            Target::ContractCapot => "Capot",
        }
    }

    /// Determines whether this target was reached.
    pub fn victory(self, points: i32, capot: bool) -> bool {
        match self {
            Target::ContractCapot => capot,
            other => points >= other.score(),
        }
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "80" => Ok(Target::Contract80),
            "90" => Ok(Target::Contract90),
            "100" => Ok(Target::Contract100),
            "110" => Ok(Target::Contract110),
            "120" => Ok(Target::Contract120),
            "130" => Ok(Target::Contract130),
            "140" => Ok(Target::Contract140),
            "150" => Ok(Target::Contract150),
            "160" => Ok(Target::Contract160),
            "Capot" => Ok(Target::ContractCapot),
            _ => Err(format!("invalid target: {}", s)),
        }
    }
}

impl ToString for Target {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

/// Contract taken by a team.
///
/// Composed of a trump suit and a target to reach.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// Initial author of the contract.
    pub author: pos::PlayerPos,
    /// Trump suit for this game.
    pub trump: cards::Suit,
    /// Target for the contract.
    pub target: Target,
    /// Level of coinche:
    ///
    /// * `0`: not coinched
    /// * `1`: coinched
    /// * `2`: surcoinched
    pub coinche_level: i32,
}

impl Contract {
    fn new(author: pos::PlayerPos, trump: cards::Suit, target: Target) -> Self {
        Contract {
            author,
            trump,
            target,
            coinche_level: 0,
        }
    }
}

/// Current state of an auction
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AuctionState {
    /// Players are still bidding for the highest contract
    Bidding,
    /// One player coinched, maybe another one will surcoinche?
    Coinching,
    /// Auction is over, game will begin
    Over,
    /// No contract was taken, a new game will start
    Cancelled,
}

/// Represents the entire auction process.
pub struct Auction {
    history: Vec<Contract>,
    pass_count: usize,
    first: pos::PlayerPos,
    state: AuctionState,
    players: [cards::Hand; 4],
}

/// Possible error occuring during an Auction.
#[derive(Eq, PartialEq, Debug)]
pub enum BidError {
    /// The auction was closed and does not accept more contracts.
    AuctionClosed,
    /// A player tried bidding before his turn.
    TurnError,
    /// The given bid was not higher than the previous one.
    NonRaisedTarget,
    /// Cannot complete the auction when it is still running.
    AuctionRunning,
    /// No contract was offered during the auction, it cannot complete.
    NoContract,
    /// The contract was coinched too many times.
    OverCoinche,
}

impl fmt::Display for BidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BidError::AuctionClosed => write!(f, "auctions are closed"),
            BidError::TurnError => write!(f, "invalid turn order"),
            BidError::NonRaisedTarget => write!(f, "bid must be higher than current contract"),
            BidError::AuctionRunning => write!(f, "the auction are still running"),
            BidError::NoContract => write!(f, "no contract was offered"),
            BidError::OverCoinche => write!(f, "contract is already sur-coinched"),
        }
    }
}

impl Auction {
    /// Starts a new auction, starting with the player `first`.
    pub fn new(first: pos::PlayerPos) -> Self {
        Auction {
            history: Vec::new(),
            pass_count: 0,
            state: AuctionState::Bidding,
            first,
            players: super::deal_hands(),
        }
    }

    /// Returns the current state of the auctions.
    pub fn get_state(&self) -> AuctionState {
        self.state
    }

    fn can_bid(&self, target: Target) -> Result<(), BidError> {
        if self.state != AuctionState::Bidding {
            return Err(BidError::AuctionClosed);
        }

        if !self.history.is_empty()
            && target.score() <= self.history[self.history.len() - 1].target.score()
        {
            return Err(BidError::NonRaisedTarget);
        }

        Ok(())
    }

    /// Returns the player that is expected to play next.
    pub fn next_player(&self) -> pos::PlayerPos {
        let base = if let Some(contract) = self.history.last() {
            contract.author.next()
        } else {
            self.first
        };
        base.next_n(self.pass_count)
    }

    /// Bid a new, higher contract.
    pub fn bid(
        &mut self,
        pos: pos::PlayerPos,
        trump: cards::Suit,
        target: Target,
    ) -> Result<AuctionState, BidError> {
        if pos != self.next_player() {
            return Err(BidError::TurnError);
        }

        self.can_bid(target)?;

        // If we're all the way to the top, there's nowhere else to go
        if target == Target::ContractCapot {
            self.state = AuctionState::Coinching;
        }

        let contract = Contract::new(pos, trump, target);
        self.history.push(contract);
        self.pass_count = 0;

        // Only stops the bids if the guy asked for a capot
        Ok(self.state)
    }

    /// Look at the last offered contract.
    ///
    /// Returns `None` if no contract was offered yet.
    pub fn current_contract(&self) -> Option<&Contract> {
        if self.history.is_empty() {
            None
        } else {
            Some(&self.history[self.history.len() - 1])
        }
    }

    /// Returns the players cards.
    pub fn hands(&self) -> [cards::Hand; 4] {
        self.players
    }

    /// The current player passes his turn.
    ///
    /// Returns the new auction state :
    ///
    /// * `AuctionState::Cancelled` if all players passed
    /// * `AuctionState::Over` if 3 players passed in a row
    /// * The previous state otherwise
    pub fn pass(&mut self, pos: pos::PlayerPos) -> Result<AuctionState, BidError> {
        if pos != self.next_player() {
            return Err(BidError::TurnError);
        }

        self.pass_count += 1;

        // After 3 passes, we're back to the contract author, and we can start.
        if !self.history.is_empty() {
            if self.pass_count >= 3 {
                self.state = AuctionState::Over;
            }
        } else if self.pass_count >= 4 {
            self.state = AuctionState::Cancelled;
        };

        Ok(self.state)
    }

    /// Attempt to coinche the current contract.
    pub fn coinche(&mut self, pos: pos::PlayerPos) -> Result<AuctionState, BidError> {
        if pos != self.next_player() {
            return Err(BidError::TurnError);
        }

        if self.history.is_empty() {
            return Err(BidError::NoContract);
        }

        let i = self.history.len() - 1;
        if self.history[i].coinche_level > 1 {
            return Err(BidError::OverCoinche);
        }

        self.history[i].coinche_level += 1;
        // Stop if we are already sur-coinching
        self.state = if self.history[i].coinche_level == 2 {
            AuctionState::Over
        } else {
            AuctionState::Coinching
        };

        Ok(self.state)
    }

    /// Consumes a complete auction to enter the second game phase.
    ///
    /// If the auction was ready, returns `Ok<GameState>`
    pub fn complete(&mut self) -> Result<game::GameState, BidError> {
        if self.state != AuctionState::Over {
            Err(BidError::AuctionRunning)
        } else if self.history.is_empty() {
            Err(BidError::NoContract)
        } else {
            Ok(game::GameState::new(
                self.first,
                self.players,
                self.history.pop().expect("contract history empty"),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cards, pos};

    #[test]
    fn test_auction() {
        let mut auction = Auction::new(pos::PlayerPos::P0);

        assert!(auction.state == AuctionState::Bidding);

        // First three people pass.
        assert_eq!(auction.pass(pos::PlayerPos::P0), Ok(AuctionState::Bidding));
        assert_eq!(auction.pass(pos::PlayerPos::P1), Ok(AuctionState::Bidding));
        assert_eq!(auction.pass(pos::PlayerPos::P2), Ok(AuctionState::Bidding));

        assert_eq!(auction.pass(pos::PlayerPos::P1), Err(BidError::TurnError));
        assert_eq!(
            auction.coinche(pos::PlayerPos::P2),
            Err(BidError::TurnError)
        );

        // Someone bids.
        assert_eq!(
            auction.bid(pos::PlayerPos::P3, cards::Suit::Heart, Target::Contract80),
            Ok(AuctionState::Bidding)
        );
        assert_eq!(
            auction
                .bid(pos::PlayerPos::P0, cards::Suit::Club, Target::Contract80)
                .err(),
            Some(BidError::NonRaisedTarget)
        );
        assert_eq!(
            auction
                .bid(pos::PlayerPos::P1, cards::Suit::Club, Target::Contract100)
                .err(),
            Some(BidError::TurnError)
        );
        assert_eq!(auction.pass(pos::PlayerPos::P0), Ok(AuctionState::Bidding));
        // Partner surbids
        assert_eq!(
            auction.bid(pos::PlayerPos::P1, cards::Suit::Heart, Target::Contract100),
            Ok(AuctionState::Bidding)
        );
        assert_eq!(auction.pass(pos::PlayerPos::P2), Ok(AuctionState::Bidding));
        assert_eq!(auction.pass(pos::PlayerPos::P3), Ok(AuctionState::Bidding));
        assert_eq!(auction.pass(pos::PlayerPos::P0), Ok(AuctionState::Over));

        assert!(auction.state == AuctionState::Over);

        match auction.complete() {
            Err(_) => assert!(false),
            _ => {}
        }
    }
}
