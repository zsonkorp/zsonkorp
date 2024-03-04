pub mod fts;
mod cta;

use std::collections::HashMap;
pub use cta::Cta;
pub use cta::CtaWagerType;
pub use fts::Fts;
pub use fts::FtsWagerType;
use crate::player::Player;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use thiserror::Error;
use crate::config::ConfigError::DuplicatedWagerId;
use crate::wager::Wager;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Wagers must not be empty")]
    EmptyWager,

    #[error("Players: {0} have no wagers")]
    EmptyWagerForPlayers(String),

    #[error("Duplicated wager id: {0}")]
    DuplicatedWagerId(u32)
}

#[derive(Deserialize)]
pub struct Config<T> {
    wagers: HashMap<Player, Vec<Wager<T>>>,
    house_id: String
}

impl<T> Config<T> {
    pub fn new(wagers: HashMap<Player, Vec<Wager<T>>>, house_id: String) -> Result<Self> {
        if wagers.is_empty() {
            return Err(anyhow!(ConfigError::EmptyWager));
        }

        let mut empty_player: Vec<&Player> = Vec::new();
        let mut wager_ids: Vec<u32> = Vec::new();

        for (player, wagers) in &wagers {
            if wagers.is_empty() {
                empty_player.push(player);
            } else {
                for wager in wagers {
                    let id = wager.get_id();

                    if wager_ids.contains(id) {
                        return Err(anyhow!(DuplicatedWagerId(*id)));
                    } else {
                        wager_ids.push(*id);
                    }
                }
            }
        }

        if !empty_player.is_empty() {
            return Err(anyhow!(ConfigError::EmptyWagerForPlayers(
                    format!("Players: {:?} have no wagers", empty_player)
            )));
        }

        Ok(Config { wagers, house_id })
    }

    pub fn get_wagers(&self) -> &HashMap<Player, Vec<Wager<T>>> {
        &self.wagers
    }

    pub fn get_house_id(&self) -> &str {
        &self.house_id
    }
}