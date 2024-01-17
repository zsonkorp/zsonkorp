use std::collections::HashMap;
use crate::player::Player;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Wager {
    pub wager_type: WagerType,
    pub amount: i32
}

#[derive(Deserialize)]
pub enum WagerType {
    FullDeck,
    AtFlop(u8),
    FlopRange(u8, u8)
}

#[derive(Deserialize)]
pub(crate) struct Fts {
    pub wagers: HashMap<Player, Vec<Wager>>,
    pub house_id: String
}

impl Fts {
    pub fn new(wagers_map: HashMap<Player, Vec<Wager>>, house_id: String) -> Self {
        Fts {
            wagers: wagers_map,
            house_id
        }
    }

    pub fn set_house(&mut self, house_id: String) {
        self.house_id = house_id;
    }

    pub fn add_player(&mut self) -> Result<()> {
        todo!()
    }

    pub fn remove_player(&mut self) -> Result<()> {
        todo!()
    }

    pub fn validate(&self) -> Result<()> {
        if self.wagers.is_empty() {
            return Err(anyhow!("No wagers"));
        }

        Ok(())
    }
}