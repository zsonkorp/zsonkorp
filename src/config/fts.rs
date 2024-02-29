use std::collections::HashMap;
use crate::player::Player;
use anyhow::Result;
use serde::Deserialize;
use crate::config::Config;
use crate::wager::Wager;

#[derive(Deserialize)]
pub enum FtsWagerType {
    FullDeck,
    AtFlop(u8),
    FlopRange(u8, u8)
}

#[derive(Deserialize)]
pub struct Fts {
    base_config: Config<Wager<FtsWagerType>>
}

impl Fts {
    pub fn new(wagers: HashMap<Player, Vec<Wager<FtsWagerType>>>, house_id: String) -> Result<Self> {
        let base_config = Config::new(wagers, house_id)?;

        Ok(Fts{ base_config })
    }

    pub fn get_base_config(&self) -> &Config<Wager<FtsWagerType>> {
        &self.base_config
    }
}