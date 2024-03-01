use std::collections::HashMap;
use crate::player::Player;
use anyhow::Result;
use serde::Deserialize;
use crate::config::Config;
use crate::wager::Wager;

#[derive(Deserialize)]
pub struct Odds {

    #[serde(rename = "FullDeck")]
    full_deck: u8,

    #[serde(rename = "AtFlop")]
    at_flop: u8,

    #[serde(rename = "FlopRange")]
    flop_range: u8
}

impl Default for Odds {
    fn default() -> Self {
        Odds {
            full_deck: 17,
            at_flop: 17,
            flop_range: 17
        }
    }
}

#[derive(Deserialize)]
pub enum FtsWagerType {
    FullDeck,
    AtFlop(u8),
    FlopRange(u8, u8)
}

#[derive(Deserialize)]
pub struct Fts {
    base_config: Config<FtsWagerType>,
    odds: Odds
}

impl Fts {
    pub fn new(wagers: HashMap<Player, Vec<Wager<FtsWagerType>>>, house_id: String) -> Result<Self> {
        let base_config = Config::new(wagers, house_id)?;

        Ok( Fts{ base_config, odds: Odds::default() })
    }

    pub fn get_base_config(&self) -> &Config<FtsWagerType> {
        &self.base_config
    }
}