use std::collections::HashMap;
use crate::player::Player;
use anyhow::{anyhow, Result};

pub struct Wager {
    pub wager_type: WagerType,
    pub amount: i32
}
pub enum WagerType {
    FullDeck,
    AtFlop(u8),
    FlopRange(u8, u8)
}

pub(crate) struct Fts {
    pub wagers: HashMap<Player, Vec<Wager>>,
    pub house_id: Option<String>
}

impl Fts {
    pub fn new(wagers_map: HashMap<Player, Vec<Wager>>, house_id: Option<String>) -> Self {
        Fts {
            wagers: wagers_map,
            house_id
        }
    }

    pub fn set_house(&mut self, house_id: String) {
        self.house_id = Some(house_id);
    }

    pub fn add_player(&mut self) -> Result<()> {
        todo!()
    }

    pub fn remove_player(&mut self) -> Result<()> {
        todo!()
    }

    pub fn validate(&self) -> Result<()> {
        if self.wagers.is_empty() {
            Err(anyhow!("No wagers"))
        }

        if self.house_id == None {
            Err(anyhow!("No house"))
        }

        Ok(())
    }
}