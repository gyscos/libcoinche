use std::fmt;
use std::str::FromStr;

use super::game;
use super::cards;
use super::pos;

use rustc_serialize;

#[derive(PartialEq,Clone,Copy)]
pub enum Target {
    Contract80,
    Contract90,
    Contract100,
    Contract110,
    Contract120,
    Contract130,
    Contract140,
    Contract150,
    Contract160,
    ContractCapot,
}

impl rustc_serialize::Encodable for Target {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(match self {
            &Target::Contract80 => "80",
            &Target::Contract90 => "90",
            &Target::Contract100 => "100",
            &Target::Contract110 => "110",
            &Target::Contract120 => "120",
            &Target::Contract130 => "130",
            &Target::Contract140 => "140",
            &Target::Contract150 => "150",
            &Target::Contract160 => "160",
            &Target::ContractCapot => "Capot",
        })
    }
}


impl Target {
    pub fn score(&self) -> i32 {
        match *self {
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

    pub fn victory(&self, score: i32, capot: bool) -> bool {
        match *self {
            Target::Contract80 => score >= 80,
            Target::Contract90 => score >= 90,
            Target::Contract100 => score >= 100,
            Target::Contract110 => score >= 110,
            Target::Contract120 => score >= 120,
            Target::Contract130 => score >= 130,
            Target::Contract140 => score >= 140,
            Target::Contract150 => score >= 150,
            Target::Contract160 => score >= 160,
            Target::ContractCapot => capot,
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

#[derive(Clone,RustcEncodable)]
pub struct Contract {
    pub trump: cards::Suit,
    pub author: pos::PlayerPos,
    pub target: Target,
    pub coinche_level: i32,
}

#[derive(PartialEq,Clone,Copy,Debug)]
pub enum AuctionState {
    Bidding,
    Coinching,
    Over,
    Cancelled,
}

pub struct Auction {
    history: Vec<Contract>,
    pass_count: usize,
    first: pos::PlayerPos,
    state: AuctionState,
    players: [cards::Hand; 4],
}

pub fn new_auction(first: pos::PlayerPos) -> Auction {
    Auction {
        history: Vec::new(),
        pass_count: 0,
        state: AuctionState::Bidding,
        first: first,
        players: super::deal_hands(),
    }
}

#[derive(PartialEq,Debug)]
pub enum BidError {
    AuctionClosed,
    PreCoinchedContract,
    TurnError,
    NonRaisedTarget,
    AuctionRunning,
    NoContract,
    OverCoinche,
}

impl fmt::Display for BidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BidError::AuctionClosed => write!(f, "auctions are closed"),
            &BidError::PreCoinchedContract => write!(f, "cannot bid a coinched contract"),
            &BidError::TurnError => write!(f, "invalid turn order"),
            &BidError::NonRaisedTarget => write!(f, "bid must be higher than current contract"),
            &BidError::AuctionRunning => write!(f, "the auction are still running"),
            &BidError::NoContract => write!(f, "no contract was offered"),
            &BidError::OverCoinche => write!(f, "contract is already sur-coinched"),
        }
    }
}

impl Auction {

    pub fn get_state(&self) -> AuctionState {
        self.state
    }

    fn can_bid(&self, contract: &Contract) -> Result<(),BidError> {
        if self.state != AuctionState::Bidding {
            return Err(BidError::AuctionClosed);
        }

        if contract.coinche_level != 0 {
            return Err(BidError::PreCoinchedContract);
        }

        if !self.history.is_empty() {
            if contract.author != self.current_contract().expect("no contract found").author.next_n(self.pass_count + 1) {
                return Err(BidError::TurnError);
            }
            if contract.target.score() <= self.history[self.history.len()-1].target.score() {
                return Err(BidError::NonRaisedTarget);
            }
        } else {
            if contract.author != self.first.next_n(self.pass_count) {
                return Err(BidError::TurnError);
            }
        }

        Ok(())
    }

    // Bid a new, higher contract.
    pub fn bid(&mut self, contract: Contract) -> Result<AuctionState,BidError> {
        match self.can_bid(&contract) {
            Err(err) => return Err(err),
            Ok(_) => (),
        }

        if contract.target == Target::ContractCapot {
            self.state = AuctionState::Coinching;
        }

        self.history.push(contract);
        self.pass_count = 0;

        // Only stops the bids if the guy asked for a capot
        Ok(self.state)
    }

    pub fn current_contract(&self) -> Option<&Contract> {
        if self.history.is_empty() {
            None
        } else {
            Some(&self.history[self.history.len() - 1])
        }
    }

    pub fn hands(&self) -> [cards::Hand; 4] {
        self.players
    }

    pub fn pass(&mut self) -> AuctionState {
        self.pass_count += 1;

        // After 3 passes, we're back to the contract author, and we can start.
        if !self.history.is_empty() {
            if self.pass_count >= 3 {
                self.state = AuctionState::Over;
            }
        } else {
            if self.pass_count >= 4 {
                self.state = AuctionState::Cancelled;
            }
        };

        self.state
    }

    pub fn coinche(&mut self) -> Result<AuctionState,BidError> {
        if self.history.is_empty() {
            Err(BidError::NoContract)
        } else {
            let i = self.history.len() - 1;
            if self.history[i].coinche_level > 1 {
                Err(BidError::OverCoinche)
            } else {
                self.history[i].coinche_level += 1;
                // Stop if we are already sur-coinching
                self.state = if self.history[i].coinche_level == 2 {
                    AuctionState::Over
                } else {
                    AuctionState::Coinching
                };

                Ok(self.state)
            }
        }
    }

    // Moves the auction to kill it
    pub fn complete(&mut self) -> Result<game::GameState,BidError> {
        if self.state != AuctionState::Over {
            Err(BidError::AuctionRunning)
        } else if self.history.is_empty() {
            Err(BidError::NoContract)
        } else {
            Ok(game::new_game(self.first, self.players, self.history.pop().expect("contract history empty")))
        }
    }
}

#[test]
fn test_auction() {
    let mut auction = new_auction(pos::PlayerPos(0));

    assert!(auction.state == AuctionState::Bidding);

    // First three people pass.
    assert_eq!(auction.pass(), AuctionState::Bidding);
    assert_eq!(auction.pass(), AuctionState::Bidding);
    assert_eq!(auction.pass(), AuctionState::Bidding);

    // Someone bids.
    assert_eq!(auction.bid(Contract{
        author: pos::PlayerPos(3),
        trump: cards::HEART,
        target: Target::Contract80,
        coinche_level: 0,
    }), Ok(AuctionState::Bidding));
    assert_eq!(auction.bid(Contract{
        author: pos::PlayerPos(0),
        trump: cards::CLUB,
        target: Target::Contract80,
        coinche_level: 0,
    }).err(), Some(BidError::NonRaisedTarget));
    assert_eq!(auction.bid(Contract{
        author: pos::PlayerPos(1),
        trump: cards::CLUB,
        target: Target::Contract100,
        coinche_level: 0,
    }).err(), Some(BidError::TurnError));
    assert_eq!(auction.pass(), AuctionState::Bidding);
    // Partner surbids
    assert_eq!(auction.bid(Contract{
        author: pos::PlayerPos(1),
        trump: cards::HEART,
        target: Target::Contract100,
        coinche_level: 0,
    }), Ok(AuctionState::Bidding));
    assert_eq!(auction.pass(), AuctionState::Bidding);
    assert_eq!(auction.pass(), AuctionState::Bidding);
    assert_eq!(auction.pass(), AuctionState::Over);

    assert!(auction.state == AuctionState::Over);

    match auction.complete() {
        Err(_) => assert!(false),
        _=> {},
    }
}
