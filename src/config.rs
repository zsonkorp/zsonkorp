pub mod fts;
mod cta;

use std::collections::HashMap;
pub use cta::Cta;
pub use cta::CtaWagerType;
use crate::player::Player;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Wagers must not be empty")]
    EmptyWager,

    #[error("Players: {0} have no wagers")]
    EmptyWagerForPlayers(String)
}

#[derive(Deserialize)]
pub struct Config<W> {
    wagers: HashMap<Player, Vec<W>>,
    house_id: String
}

impl<W> Config<W> {
    pub fn new(wagers: HashMap<Player, Vec<W>>, house_id: String) -> Result<Self> {
        if wagers.is_empty() {
            return Err(anyhow!(ConfigError::EmptyWager));
        }

        let mut empty_player: Vec<&Player> = Vec::new();
        wagers.iter().for_each( |(player, wagers)| if wagers.is_empty() {
            empty_player.push(player);
        });

        if !empty_player.is_empty() {
            return Err(anyhow!(ConfigError::EmptyWagerForPlayers(
                    format!("Players: {:?} have no wagers", empty_player)
                )));
        }

        Ok(Config { wagers, house_id })
    }

    pub fn get_wagers(&self) -> &HashMap<Player, Vec<W>> {
        &self.wagers
    }

    pub fn get_house_id(&self) -> &str {
        &self.house_id
    }
}