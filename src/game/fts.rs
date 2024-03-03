use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;
use crate::deck::Deck;
use crate::state::State;
use crate::config::fts::{Fts as FtsConfig, FtsWagerType};
use anyhow::{anyhow, Result};
use crate::game::{Game, GameType};
use crate::payout::Payout;

pub struct Fts<'a> {
    deck: Deck,
    config: FtsConfig,
    state: State,
    max_flop_count: u8,
    flopped_at: Option<u8>,
    payouts: Vec<Payout<'a>>
}
impl<'a> Fts<'a> {
    pub fn new(config: FtsConfig) -> Result<Self> {
        let mut fts = Fts {
            deck: Deck::default(),
            config,
            state: State::Setup,
            max_flop_count: 0,
            flopped_at: None,
            payouts: Vec::new()
        };

        fts.apply_config()?;

        Ok(fts)
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }
    fn apply_config(&mut self) -> Result<()> {

        let max_possible_flops = (self.deck.len() / 3) as u8;

        'outer:
        for value in self.config.get_base_config().get_wagers().values() {
            for wager in value.iter() {
                self.max_flop_count = cmp::max(self.max_flop_count, match wager.get_wager_type() {
                    FtsWagerType::FullDeck => max_possible_flops,
                    FtsWagerType::AtFlop(ith) => *ith,
                    FtsWagerType::FlopRange(_, endInc) => *endInc
                });

                if self.max_flop_count == max_possible_flops {
                    break 'outer
                }
            }
        }

        Ok(())
    }
    fn ready(&self) -> Result<()> {

        if self.state != State::Setup {
            return Err(anyhow!("Game already started"));
        }

        if self.max_flop_count == 0 {
            return Err(anyhow!("Game set to perform 0 flops"));
        }

        Ok(())
    }

    fn generate_payouts(&'a mut self) -> Result<()> {
        let mut house_payout = 0;

        for (player, wager_vec) in self.config.get_base_config().get_wagers().iter() {

            for wager in wager_vec {

                let amount = match wager.get_wager_type() {

                    FtsWagerType::FullDeck => match &self.flopped_at {
                        None => -(wager.amount * 17),
                        Some(flopped_at) => wager.amount * i32::from(17 - flopped_at)
                    },

                    FtsWagerType::AtFlop(flop) => match &self.flopped_at {
                        None => -wager.amount,
                        Some(flopped_at) => if flopped_at == flop { wager.amount * 17 } else { -wager.amount }
                    },

                    FtsWagerType::FlopRange(flop_start, flop_end) => match &self.flopped_at {
                        None => -( wager.amount * i32::from(flop_end - flop_start + 1) ),
                        Some(flopped_at) => if flopped_at <= flop_end && flopped_at >= flop_start {
                            wager.amount * i32::from(17 - (flopped_at - flop_start) )
                        } else {
                            -( wager.amount * i32::from(flop_end - flop_start + 1) )
                        }
                    }
                };

                if amount != 0 {
                    self.payouts.push(Payout::new(player.get_id(), Some(wager), amount)?);
                }

                house_payout -= amount;
            }
        }

        if house_payout != 0 {
            self.payouts.push(
                Payout::new::<FtsWagerType>(
                    self.config.get_base_config().get_house_id(), None, house_payout
                )?
            );
        }

        Ok(())
    }
}

impl Game for Fts<'_> {
    fn my_type(&self) -> GameType {
        GameType::Fts
    }

    fn start(&mut self) -> Result<()> {
        self.ready()?;

        self.state = State::Started;
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

        // fts does not need any internal state transitions yet, go straight to the end
        self.state = State::Ended;

        Ok(())
    }

    fn get_payout(&self) -> &[Payout] {

        // match &self.flopped_at {
        //     Some(at) => format!("Flopped at: {}\nCards: {:?}", at, self.deck.get_dealt_cards()),
        //     None =>  format!("No flop\nCards: {:?}", self.deck.get_dealt_cards())
        // }

        return &self.payouts;
    }
}

#[cfg(test)]
mod tests {
    use crate::config::fts::Wager;
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

        let config = FtsConfig::new(wager_map, "house".to_string(), None);


        let mut game = Fts::init(config)?;

        game.start()?;

        let payout = game.get_payout().unwrap();

        println!("Payout: {:?}", payout);

        println!("Result: {}", game.get_result());
        Ok(())
    }
}