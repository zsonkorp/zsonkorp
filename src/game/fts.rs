use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;
use crate::deck::Deck;
use crate::state::State;
use crate::config::fts::{Fts as FtsConfig, FtsWagerType};
use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};
use crate::game::{Game, GameType};
use crate::game::Error::InvalidTransition;
use crate::payout::Payout;
use crate::state::GameState::*;
use crate::transition::Transition;
use crate::transition::GameTransition::Start;

pub struct Fts {
    deck: Deck,
    config: FtsConfig,
    state: State,
    max_flop_count: u8,
    flopped_at: Option<u8>     // This is the ith flop where the first flop is 0
}
impl Fts {
    pub fn new(config: FtsConfig) -> Result<Self> {
        let mut fts = Fts {
            deck: Deck::default(),
            config,
            state: State::Game(Setup),
            max_flop_count: 0,
            flopped_at: None
        };

        fts.apply_config()?;

        Ok(fts)
    }

    fn apply_config(&mut self) -> Result<()> {

        'outer:
        for value in self.config.get_base_config().get_wagers().values() {
            for wager in value.iter() {
                self.max_flop_count = cmp::max(self.max_flop_count, match wager.get_wager_type() {
                    FtsWagerType::FullDeck => self.get_max_possible_flop_count(),
                    FtsWagerType::AtFlop(ith) => *ith,
                    FtsWagerType::FlopRange(_, endInc) => *endInc
                });

                if self.max_flop_count == self.get_max_possible_flop_count() {
                    break 'outer
                }
            }
        }

        Ok(())
    }
    fn can_start(&self) -> Result<()> {
        if self.max_flop_count == 0 {
            return Err(anyhow!("Game set to perform 0 flops"));
        }

        Ok(())
    }

    fn get_max_possible_flop_count(&self) -> u8 {
        (self.deck.total_len() / 3) as u8
    }

    fn transition_state(&mut self, transition: Transition) -> Result<State>{
        let new_state = match &self.state {
            State::Game(Setup) => {
                match &transition {
                    Transition::Game(Start) => {
                        self.start_game()?;
                        // fts does not need any internal state transitions yet, go straight to the end
                        State::Game(Ended)
                    },
                    _ => return Err(InvalidTransition.into())
                }
            },
            _ => return Err(InvalidTransition.into())
        };

        Ok(new_state)
    }

    fn start_game(&mut self) -> Result<()>{
        self.can_start()?;

        self.deck.shuffle();

        match self.deck.deal_multi((self.max_flop_count * 3).into()) {
            None => return Err(anyhow!("Could not deal the required amount of cards")),
            Some(cards) => {
                let result = cards
                    .chunks(3)
                    .enumerate()
                    .filter_map(|(i, chunk)| {
                        if chunk[0].get_suit() == chunk[1].get_suit() && chunk[1].get_suit() == chunk[2].get_suit() {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .next();

                if let Some(flopped_at) = result {
                    self.flopped_at = Some(flopped_at as u8);
                }
            }
        }

        Ok(())
    }
}

impl Serialize for Fts {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}

#[typetag::serialize]
impl Game for Fts {
    fn get_type(&self) -> GameType {
        GameType::Fts
    }

    fn transition(&mut self, transition: Transition) -> Result<()> {

        let new_state = self.transition_state(transition)?;

        self.state = new_state;
        Ok(())
    }

    fn get_valid_transitions(&self) -> Vec<Transition> {

        let mut transitions: Vec<Transition> = Vec::new();

        match self.state {
            State::Game(Setup) => transitions.push(Transition::Game(Start)),
            _ => {}
        }

        transitions
    }

    fn get_payout(&self) -> Result<Vec<Payout>> {

        // match &self.flopped_at {
        //     Some(at) => format!("Flopped at: {}\nCards: {:?}", at, self.deck.get_dealt_cards()),
        //     None =>  format!("No flop\nCards: {:?}", self.deck.get_dealt_cards())
        // }

        let mut payouts: Vec<Payout> = Vec::new();

        if self.state != State::Game(Ended) {
            return Ok(payouts);
        }

        let mut house_payout = 0;

        for (player, wager_vec) in self.config.get_base_config().get_wagers().iter() {

            for wager in wager_vec {

                let amount = match wager.get_wager_type() {

                    FtsWagerType::FullDeck => match &self.flopped_at {

                        // No flop, player loses their wager times number of flops for the deck
                        None => -(wager.amount * i32::from(self.get_max_possible_flop_count())),

                        // Player wins full deck odds * wager - failed flops * wager or wager * (full deck odds - failed flops)
                        Some(flopped_at) =>
                            wager.amount * (self.config.get_odds().get_full_deck() - i32::from(*flopped_at))
                    },

                    FtsWagerType::AtFlop(flop) => match &self.flopped_at {
                        // No flop, player loses the wager
                        None => -wager.amount,

                        // Player wins only if it's on the same flop, player only loses if it flopped after
                        Some(flopped_at) =>
                            if flopped_at == flop {
                                wager.amount * self.config.get_odds().get_at_flop()
                            } else if flopped_at > flop {
                                -wager.amount
                            } else {
                                0
                            }
                    },

                    FtsWagerType::FlopRange(flop_start, flop_end) => match &self.flopped_at {
                        // No flop, player loses the wager * the length of the range
                        None => -( wager.amount * i32::from(flop_end - flop_start + 1) ),

                        // Player wins if it's within range, loses if flopped happened after the range
                        Some(flopped_at) => if flopped_at <= flop_end && flopped_at >= flop_start {
                            wager.amount * (self.config.get_odds().get_flop_range() - i32::from(flopped_at - flop_start) )
                        } else if flopped_at > flop_end {
                            -( wager.amount * i32::from(flop_end - flop_start + 1) )
                        } else {
                            0
                        }
                    }
                };

                if amount != 0 {
                    payouts.push(Payout::new(player.get_id(), Some(wager), amount)?);
                }

                house_payout -= amount;
            }
        }

        if house_payout != 0 {
            payouts.push(
                Payout::new::<FtsWagerType>(
                    self.config.get_base_config().get_house_id(), None, house_payout
                )?
            );
        }

        Ok(payouts)
    }
}

#[cfg(test)]
mod tests {
    use crate::wager::Wager;
    use crate::config::fts::FtsWagerType::FullDeck;
    use crate::player::Player;
    use super::*;

    #[test]
    fn flow() -> Result<()>{

        let player = Player::new("player1".to_string());
        let wager_map: HashMap<Player, Vec<Wager<FtsWagerType>>> = HashMap::from(
            [
                (player, vec![Wager::new( 0, FullDeck, 100)?])
            ]
        );

        let config = FtsConfig::new(wager_map, "house".to_string(), None)?;


        let mut game = Fts::new(config)?;

        game.transition(Transition::Game(Start))?;

        let payout = game.get_payout();

        println!("Payout: {:?}", payout);

        // println!("Result: {}", game.get_result());
        Ok(())
    }
}