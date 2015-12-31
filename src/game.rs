//! Module for the card game, after auctions are complete.
use std::fmt;

use super::pos;
use super::cards;
use super::trick;
use super::bid;
use super::points;

/// Describes the state of a coinche game, ready to play a card.
#[derive(Clone)]
pub struct GameState {
    players: [cards::Hand; 4],

    current: pos::PlayerPos,

    contract: bid::Contract,

    points: [i32; 2],
    tricks: Vec<trick::Trick>,
}

/// Result of a game.
#[derive(PartialEq,Debug)]
pub enum GameResult {
    /// The game is still playing
    Nothing,

    /// The game is over
    GameOver {
        /// Worth of won tricks
        points: [i32;2],
        /// Winning team
        winners: pos::Team,
        /// Score for this game
        scores: [i32;2]
    },
}

/// Result of a trick
#[derive(PartialEq,Debug)]
pub enum TrickResult {
    Nothing,
    TrickOver(pos::PlayerPos, GameResult),
}

/// Error that can occur during play
#[derive(PartialEq,Debug)]
pub enum PlayError {
    /// A player tried to act before his turn
    TurnError,
    /// A player tried to play a card he doesn't have
    CardMissing,
    /// A player tried to play the wrong suit, while he still have some
    IncorrectSuit,
    /// A player tried to play the wrong suit, while he still have trumps
    InvalidPiss,
    /// A player did not raise on the last played trump
    NonRaisedTrump,

    /// No last trick is available for display
    NoLastTrick,
}

impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PlayError::TurnError => write!(f, "invalid turn order"),
            &PlayError::CardMissing => write!(f, "you can only play cards you have"),
            &PlayError::IncorrectSuit => write!(f, "wrong suit played"),
            &PlayError::InvalidPiss => write!(f, "you must use trumps"),
            &PlayError::NonRaisedTrump => write!(f, "too weak trump played"),
            &PlayError::NoLastTrick => write!(f, "no trick has been played yet"),
        }
    }
}

impl GameState {

    /// Creates a new GameState, with the given cards, first player and contract.
    pub fn new(first: pos::PlayerPos, hands: [cards::Hand; 4], contract: bid::Contract) -> Self {
        GameState {
            players: hands,
            current: first,
            contract: contract,
            tricks: vec![trick::Trick::new(first)],
            points: [0; 2],
        }
    }

    /// Returns the contract used for this game
    pub fn contract(&self) -> &bid::Contract {
        &self.contract
    }

    /// Try to play a card
    pub fn play_card(&mut self, player: pos::PlayerPos, card: cards::Card)
                     -> Result<TrickResult,PlayError> {
        if self.current != player {
            return Err(PlayError::TurnError);
        }

        // Is that a valid move?
        try!(self.can_play(player, card));

        // Play the card
        let trump = self.contract.trump;
        let trick_over = self.current_trick_mut().play_card(player, card, trump);

        // Is the trick over?
        let result = if trick_over {
            let winner = self.current_trick().winner;
            let score = self.current_trick().score(trump);
            self.points[winner.team().0] += score;
            if self.tricks.len() == 8 {
                // 10 de der
                self.points[winner.team().0] += 10;
            } else {
                self.tricks.push(trick::Trick::new(winner));
            }
            self.current = winner;
            TrickResult::TrickOver(winner, self.get_game_result())
        } else {
            self.current = self.current.next();
            TrickResult::Nothing
        };


        Ok(result)
    }

    /// Returns the player expected to play next.
    pub fn next_player(&self) -> pos::PlayerPos {
        self.current
    }

    fn get_game_result(&self) -> GameResult {
        if !self.is_over() {
            return GameResult::Nothing;
        }

        let taking_team = self.contract.author.team();
        let taking_points = self.points[taking_team.0];

        let capot = self.is_capot(taking_team);

        let victory = self.contract.target.victory(taking_points, capot);

        let winners = if victory { taking_team } else { taking_team.opponent() };

        // TODO: Allow for variants in scoring. (See wikipedia article)
        let mut scores = [0; 2];
        if victory {
            scores[winners.0] = self.contract.target.score();
        } else {
            scores[winners.0] = 160;
        }

        GameResult::GameOver {
            points: self.points,
            winners: winners,
            scores: scores,
        }
    }

    fn is_capot(&self, team: pos::Team) -> bool {
        for trick in self.tricks.iter() {
            if trick.winner.team() != team {
                return false;
            }
        }

        true
    }

    /// Returns the cards of all players
    pub fn hands(&self) -> [cards::Hand; 4] {
        self.players
    }

    /// Returns `true` if the move appear legal.
    pub fn can_play(&self, p: pos::PlayerPos, card: cards::Card) -> Result<(),PlayError> {
        let hand = self.players[p.0];

        // First, we need the card to be able to play
        if !hand.has(card) {
            return Err(PlayError::CardMissing);;
        }

        let trick = self.current_trick();

        if p == trick.first {
            return Ok(());
        }

        let card_suit = card.suit();
        let starting_suit = trick.suit().unwrap();
        if card_suit != starting_suit {
            if hand.has_any(starting_suit) {
                return Err(PlayError::IncorrectSuit);
            }

            if card_suit != self.contract.trump {
                let partner_winning = p.is_partner(self.current_trick().winner);
                if !partner_winning && hand.has_any(self.contract.trump) {
                    return Err(PlayError::InvalidPiss);
                }
            }
        }

        // One must raise when playing trump
        if card_suit == self.contract.trump {
            let highest = highest_trump(&self.current_trick(), self.contract.trump, p);
            if points::trump_strength(card.rank()) < highest {
                if has_higher(hand, card_suit, highest) {
                    return Err(PlayError::NonRaisedTrump);;
                }
            }
        }

        Ok(())
    }

    fn is_over(&self) -> bool {
        self.tricks.len() == 8
    }

    /// Return the last trick, if possible
    pub fn last_trick(&self) -> Result<&trick::Trick,PlayError> {
        if self.tricks.len() == 1 {
            Err(PlayError::NoLastTrick)
        } else {
            let i = self.tricks.len()-2;
            Ok(&self.tricks[i])
        }
    }

    /// Returns the current trick.
    pub fn current_trick(&self) -> &trick::Trick {
        let i = self.tricks.len()-1;
        &self.tricks[i]
    }

    fn current_trick_mut(&mut self) -> &mut trick::Trick {
        let i = self.tricks.len()-1;
        &mut self.tricks[i]
    }
}

fn has_higher(hand: cards::Hand, trump: cards::Suit, strength: i32) -> bool {
    for ri in 0..8 {
        let rank = cards::Rank::from_n(ri);
        if points::trump_strength(rank) > strength && hand.has(cards::Card::new(trump, rank)) {
            return true
        }
    }

    false
}

fn highest_trump(trick: &trick::Trick, trump: cards::Suit, player: pos::PlayerPos) -> i32 {
    let mut highest = -1;

    for p in trick.first.until(player) {
        if trick.cards[p.0].unwrap().suit() == trump {
            let str = points::trump_strength(trick.cards[p.0].unwrap().rank());
            if str > highest {
                highest = str;
            }
        }
    }

    highest
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::has_higher;
    use ::{points,cards,bid,pos};

    #[test]
    fn test_play_card() {
        let mut hands = [cards::Hand::new(); 4];
        hands[0].add(cards::Card::new(cards::HEART, cards::RANK_8));
        hands[0].add(cards::Card::new(cards::HEART, cards::RANK_X));
        hands[0].add(cards::Card::new(cards::HEART, cards::RANK_A));
        hands[0].add(cards::Card::new(cards::HEART, cards::RANK_9));
        hands[0].add(cards::Card::new(cards::CLUB, cards::RANK_7));
        hands[0].add(cards::Card::new(cards::CLUB, cards::RANK_8));
        hands[0].add(cards::Card::new(cards::CLUB, cards::RANK_9));
        hands[0].add(cards::Card::new(cards::CLUB, cards::RANK_J));

        hands[1].add(cards::Card::new(cards::CLUB, cards::RANK_Q));
        hands[1].add(cards::Card::new(cards::CLUB, cards::RANK_K));
        hands[1].add(cards::Card::new(cards::CLUB, cards::RANK_X));
        hands[1].add(cards::Card::new(cards::CLUB, cards::RANK_A));
        hands[1].add(cards::Card::new(cards::SPADE, cards::RANK_7));
        hands[1].add(cards::Card::new(cards::SPADE, cards::RANK_8));
        hands[1].add(cards::Card::new(cards::SPADE, cards::RANK_9));
        hands[1].add(cards::Card::new(cards::SPADE, cards::RANK_J));

        hands[2].add(cards::Card::new(cards::DIAMOND, cards::RANK_7));
        hands[2].add(cards::Card::new(cards::DIAMOND, cards::RANK_8));
        hands[2].add(cards::Card::new(cards::DIAMOND, cards::RANK_9));
        hands[2].add(cards::Card::new(cards::DIAMOND, cards::RANK_J));
        hands[2].add(cards::Card::new(cards::SPADE, cards::RANK_Q));
        hands[2].add(cards::Card::new(cards::SPADE, cards::RANK_K));
        hands[2].add(cards::Card::new(cards::HEART, cards::RANK_Q));
        hands[2].add(cards::Card::new(cards::HEART, cards::RANK_K));

        hands[3].add(cards::Card::new(cards::DIAMOND, cards::RANK_Q));
        hands[3].add(cards::Card::new(cards::DIAMOND, cards::RANK_K));
        hands[3].add(cards::Card::new(cards::DIAMOND, cards::RANK_X));
        hands[3].add(cards::Card::new(cards::DIAMOND, cards::RANK_A));
        hands[3].add(cards::Card::new(cards::SPADE, cards::RANK_X));
        hands[3].add(cards::Card::new(cards::SPADE, cards::RANK_A));
        hands[3].add(cards::Card::new(cards::HEART, cards::RANK_7));
        hands[3].add(cards::Card::new(cards::HEART, cards::RANK_J));

        let contract = bid::Contract {
            trump: cards::HEART,
            author: pos::P0,
            target: bid::Target::Contract80,
            coinche_level: 0,
        };

        let mut game = GameState::new(pos::P0, hands, contract);

        // Wrong turn
        assert_eq!(
            game.play_card(pos::P1, cards::Card::new(cards::CLUB, cards::RANK_X)).err(),
            Some(PlayError::TurnError));
        assert_eq!(
            game.play_card(pos::P0, cards::Card::new(cards::CLUB, cards::RANK_7)).ok(),
            Some(TrickResult::Nothing));
        // Card missing
        assert_eq!(
            game.play_card(pos::P1, cards::Card::new(cards::HEART, cards::RANK_7)).err(),
            Some(PlayError::CardMissing));
        // Wrong color
        assert_eq!(
            game.play_card(pos::P1, cards::Card::new(cards::SPADE, cards::RANK_7)).err(),
            Some(PlayError::IncorrectSuit));
        assert_eq!(
            game.play_card(pos::P1, cards::Card::new(cards::CLUB, cards::RANK_Q)).ok(),
            Some(TrickResult::Nothing));
        // Invalid piss
        assert_eq!(
            game.play_card(pos::P2, cards::Card::new(cards::DIAMOND, cards::RANK_7)).err(),
            Some(PlayError::InvalidPiss));
        assert_eq!(
            game.play_card(pos::P2, cards::Card::new(cards::HEART, cards::RANK_Q)).ok(),
            Some(TrickResult::Nothing));
        // UnderTrump
        assert_eq!(
            game.play_card(pos::P3, cards::Card::new(cards::HEART, cards::RANK_7)).err(),
            Some(PlayError::NonRaisedTrump));
        assert_eq!(
            game.play_card(pos::P3, cards::Card::new(cards::HEART, cards::RANK_J)).ok(),
            Some(TrickResult::TrickOver(pos::P3, game.get_game_result())));
    }

    #[test]
    fn test_has_higher_1() {
        // Simple case: X is always higher than Q.
        let mut hand = cards::Hand::new();

        hand.add(cards::Card::new(cards::HEART, cards::RANK_8));
        hand.add(cards::Card::new(cards::SPADE, cards::RANK_X));
        assert!(has_higher(hand, cards::SPADE, points::trump_strength(cards::RANK_Q)));
    }

    #[test]
    fn test_has_higher_2() {
        // Test that we don't mix colors
        let mut hand = cards::Hand::new();

        hand.add(cards::Card::new(cards::HEART, cards::RANK_8));
        hand.add(cards::Card::new(cards::SPADE, cards::RANK_X));
        assert!(!has_higher(hand, cards::HEART, points::trump_strength(cards::RANK_Q)));
    }

    #[test]
    fn test_has_higher_3() {
        // In the trump order, X is lower than 9
        let mut hand = cards::Hand::new();

        hand.add(cards::Card::new(cards::HEART, cards::RANK_J));
        hand.add(cards::Card::new(cards::SPADE, cards::RANK_X));
        assert!(!has_higher(hand, cards::SPADE, points::trump_strength(cards::RANK_9)));
    }

    #[test]
    fn test_has_higher_4() {
        // In the trump order, J is higher than A
        let mut hand = cards::Hand::new();

        hand.add(cards::Card::new(cards::HEART, cards::RANK_8));
        hand.add(cards::Card::new(cards::SPADE, cards::RANK_J));
        assert!(has_higher(hand, cards::SPADE, points::trump_strength(cards::RANK_A)));
    }

    #[test]
    fn test_has_higher_5() {
        // Test when we have no trump at all
        let mut hand = cards::Hand::new();

        hand.add(cards::Card::new(cards::HEART, cards::RANK_J));
        hand.add(cards::Card::new(cards::DIAMOND, cards::RANK_J));
        hand.add(cards::Card::new(cards::SPADE, cards::RANK_J));
        assert!(!has_higher(hand, cards::CLUB, points::trump_strength(cards::RANK_7)));
    }
}


#[cfg(feature="use_bench")]
mod benchs {
    use ::test::Bencher;
    use ::deal_seeded_hands;

    use ::{pos,cards,bid};
    use super::*;

    #[bench]
    fn bench_can_play(b: &mut Bencher) {

        fn try_deeper(game: &GameState, depth: usize) {
            let player = game.next_player();
            for c in game.hands()[player.0].list() {
                let mut new_game = game.clone();
                match new_game.play_card(player, c) {
                    Ok(_) => if depth > 0 {
                        try_deeper(&new_game, depth - 1);
                    },
                    _ => (),
                };
            }
        }

        let seed = &[3, 32, 654, 1, 844];
        let hands = deal_seeded_hands(seed);
        let game = GameState::new(pos::P0, hands, bid::Contract {
            author: pos::P0,
            trump: cards::HEART,
            target: bid::Target::Contract80,
            coinche_level: 0,
        });
        b.iter(|| {
            try_deeper(&game, 4)
        });
    }
}
