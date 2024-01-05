use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;
use crate::deck::Deck;
use crate::state::State;
use crate::config::fts::{Fts as FtsConfig, WagerType};
use anyhow::{anyhow, Result};

struct Fts {
    deck: Deck,
    config: FtsConfig,
    state: State,
    max_flop_count: u8,
    flopped_at: Option<u8>
}
impl Fts {
    pub fn init(config: FtsConfig) -> Result<Self> {
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

        let max_possible_flops = self.deck.len() / 3;

        'outer:
        for value in self.config.wagers.values() {
            for wager in value.iter() {
                self.max_flop_count = cmp::max(self.max_flop_count, match wager.wager_type {
                    WagerType::FullDeck => max_possible_flops.into(),
                    WagerType::AtFlop(ith) => *ith,
                    WagerType::FlopRange(startInc, endInc) => *endInc
                });

                if self.max_flop_count as usize == max_possible_flops {
                    break 'outer
                }
            }
        }

        Ok(())
    }

    pub fn ready(&self) -> Result<()> {

        if self.state != State::Setup {
            Err(anyhow!("Game already started"))
        }

        self.config.validate()?;

        if self.max_flop_count == 0 {
            Err(anyhow!("Game set to perform 0 flops"))
        }

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        self.ready()?;

        self.state = State::Started;


        match self.deck.deal_multi((self.max_flop_count * 3) as usize) {
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
                    self.flopped_at = Some(flopped_at.into());
                }
            }
        }

        // fts does not need any internal state transitions yet, go straight to the end
        self.state = State::Ended;

        if let Some(payout_map) = self.get_payout() {
            println!("Payout: {:?}",payout_map);
        }

        Ok(())
    }

    pub fn get_payout(&self) -> Option<HashMap<String, i32>> {
        let mut map: HashMap<String, i32> = HashMap::new();

        if self.state != State::Ended {
            None
        }

        let mut house_payout = 0;

        for (player, wager_vec) in self.config.wagers {

            let mut player_payout = 0;

            for wager in wager_vec {
                let payout = match &wager.wager_type {
                    WagerType::FullDeck => match &self.flopped_at {
                        None => -(wager.amount * 17),
                        Some(flopped_at) => flopped_at * 17 - wager.amount * (17 - flopped_at)
                    },
                    WagerType::AtFlop(flop) => match &self.flopped_at {
                        None => -wager.amount,
                        Some(flopped_at) => if flopped_at == flop { wager.amount * 17 } else { -wager.amount }
                    },
                    WagerType::FlopRange(flop_start, flop_end) => match &self.flopped_at {
                        None => -( wager.amount * (flop_end - flop_start) ),
                        Some(flopped_at) => if flopped_at <= flop_end && flopped_at >= flop_start {
                            wager.amount * 17 - wager.amount * (flopped_at - flop_start)
                        } else {
                            -( wager.amount * flop_end - flop_start )
                        }
                    }
                };

                player_payout += payout;
                house_payout -= payout;
            }

            map.insert(player.id.clone(), player_payout);
        }

        // map.insert(self.config.house_id.clone(), house_payout);

        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}