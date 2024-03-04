use std::collections::HashMap;
use crate::player::Player;
use anyhow::Result;
use serde::Deserialize;
use crate::config::Config;
use crate::wager::Wager;

#[derive(Deserialize)]
pub struct Odds {
    full_deck: i32,
    at_flop: i32,
    flop_range: i32
}

impl Odds {
    pub fn get_full_deck(&self) -> &i32 {
        &self.full_deck
    }

    pub fn get_at_flop(&self) -> &i32 {
        &self.at_flop
    }

    pub fn get_flop_range(&self) -> &i32 {
        &self.flop_range
    }
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
    AtFlop(u8),             // This is 0 based
    FlopRange(u8, u8)       // This is also 0 based, end is inclusive
}

#[derive(Deserialize)]
pub struct Fts {
    base_config: Config<FtsWagerType>,
    odds: Odds
}

impl Fts {
    pub fn new(wagers: HashMap<Player, Vec<Wager<FtsWagerType>>>,
               house_id: String,
               opt_odds: Option<Odds>) -> Result<Self> {

        let base_config = Config::new(wagers, house_id)?;

        Ok( Fts{ base_config, odds: opt_odds.unwrap_or(Odds::default()) })
    }

    pub fn get_base_config(&self) -> &Config<FtsWagerType> {
        &self.base_config
    }

    pub fn get_odds(&self) -> &Odds {
        &self.odds
    }
}