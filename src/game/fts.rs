use std::cmp;
use crate::deck::Deck;
use crate::state::State;
use crate::config::fts::{Fts as FtsConfig, Wager};
use anyhow::{anyhow, Result};

struct Fts {
    deck: Deck,
    config: FtsConfig,
    state: State,
    max_flop_count: u8
}
impl Fts {
    pub fn init(config: FtsConfig) -> Result<Self> {
        let mut fts = Fts {
            deck: Deck::default(),
            config,
            state: State::Setup,
            max_flop_count: 0,
        };

        fts.apply_config()?;

        Ok(fts)
    }

    fn apply_config(&mut self) -> Result<()> {

        // if self.config.wagers.is_empty() {
        //     Err(anyhow!("No wagers"))
        // }

        let max_possible_flops = self.deck.len() / 3;

        'outer:
        for value in self.config.wagers.values() {
            for wager in value.iter() {
                self.max_flop_count = cmp::max(self.max_flop_count, match wager {
                    Wager::FullDeck => max_possible_flops as u8,
                    Wager::AtFlop(ith) => {
                        // if *ith == 0 || *ith as usize > max_possible_flops {
                        //     Err(anyhow!("Invalid AtFlop at: {}", ith))
                        // }

                        *ith
                    },
                    Wager::FlopRange(startInc, endInc) => {
                        *endInc
                    }
                });

                if self.max_flop_count as usize == max_possible_flops {
                    break 'outer
                }
            }
        }

        Ok(())
    }
}

// impl Game for Fts {
//
//     type ConfigType = FtsConfig;
//     fn setup(&mut self, config: Self::ConfigType) -> Result<(), game::Error> {
//         self.config = Some(config);
//
//
//         Ok(())
//     }
//
//     fn ready(&self) -> bool {
//         todo!()
//     }
//
//     fn start(&mut self) -> Result<State, game::Error> {
//         todo!()
//     }
//
//     fn advance(&mut self) -> Result<State, game::Error> {
//         todo!()
//     }
//
//     fn get_payout(&self) -> Result<Payout, Error> {
//         todo!()
//     }
// }