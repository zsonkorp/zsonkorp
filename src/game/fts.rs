use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;
use crate::deck::Deck;
use crate::state::State;
use crate::config::fts::{Fts as FtsConfig, FtsWagerType};
use anyhow::{anyhow, Result};
use crate::game::{Game, GameType};

pub struct Fts {
    deck: Deck,
    config: FtsConfig,
    state: State,
    max_flop_count: u8,
    flopped_at: Option<u8>
}
impl Fts {
    pub fn new(config: FtsConfig) -> Result<Self> {
        let mut fts = Fts {
            deck: Deck::default(),
            config,
            state: State::Setup,
            max_flop_count: 0,
            flopped_at: None
        };

        fts.apply_config()?;

        Ok(fts)
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn apply_config(&mut self) -> Result<()> {

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

    pub fn ready(&self) -> Result<()> {

        if self.state != State::Setup {
            return Err(anyhow!("Game already started"));
        }

        if self.max_flop_count == 0 {
            return Err(anyhow!("Game set to perform 0 flops"));
        }

        Ok(())
    }

    pub fn get_payout(&self) -> Option<HashMap<String, i32>> {
        let mut map: HashMap<String, i32> = HashMap::new();

        if self.state != State::Ended {
            return None;
        }

        let mut house_payout = 0;

        for (player, wager_vec) in self.config.get_base_config().get_wagers().iter() {

            let mut player_payout = 0;

            for wager in wager_vec {
                let payout = match wager.get_wager_type() {

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

                player_payout += payout;
                house_payout -= payout;
            }

            if player_payout != 0 {
                map.insert(player.id.clone(), player_payout);
            }
        }

        if house_payout != 0 {
            map.insert(self.config.get_base_config().get_house_id().to_string(), house_payout);
        }

        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}

impl Game for Fts {
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

    fn get_result(&self) -> String {

        match &self.flopped_at {
            Some(at) => format!("Flopped at: {}\nCards: {:?}", at, self.deck.get_dealt_cards()),
            None =>  format!("No flop\nCards: {:?}", self.deck.get_dealt_cards())
        }
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

        let player = Player{ id: "player1".to_string() };
        let wager_map: HashMap<Player, Vec<Wager<FtsWagerType>>> = HashMap::from(
            [
                (player, vec![Wager::new( 0, FullDeck, 100)?])
            ]
        );

        let config = FtsConfig::new(wager_map, "house".to_string());


        let mut game = Fts::init(config)?;

        game.start()?;

        let payout = game.get_payout().unwrap();

        println!("Payout: {:?}", payout);

        println!("Result: {}", game.get_result());
        Ok(())
    }
}