use thiserror::Error;
use anyhow::{anyhow, Result};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::config;
use crate::config::{CtaWagerType, FtsWagerType};

pub(crate) mod fts;
mod cta;

use crate::dto::ConfigDto;
use crate::game::cta::Cta;
use crate::game::fts::Fts;
use crate::payout::Payout;
use crate::transition::Transition;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Generic game error")]
    Generic,
    #[error("Error parsing game configuration: {0}")]
    ParseConfig(String),
    #[error("Unknown game type")]
    UnknownGame,
    #[error("Invalid transition")]
    InvalidTransition,
    #[error("Game not found: {0}")]
    NotFound(String)
}

#[derive(Deserialize)]
pub enum GameType {
    Fts, Cta
}

#[typetag::serialize]
pub trait Game: Sync + Send {
    fn get_type(&self) -> GameType;
    fn transition(&mut self, transition: Transition) -> Result<()>;
    fn get_valid_transitions(&self) -> Vec<Transition>;
    fn get_payout(&self) -> Result<Vec<Payout>>;
}

pub fn create_game(game_type: &GameType, payload: &str) -> Result<Box<dyn Game>> {
    match game_type {
        GameType::Fts => {

            match serde_json::from_str::<ConfigDto<FtsWagerType>>(payload) {
                Ok(config_dto) => {

                    if config_dto.wager_map.is_empty() {
                        return Err(Error::ParseConfig("Empty wager map".to_string()).into());
                    }

                    let fts_config = config::Fts::new(
                        config_dto.wager_map,
                        config_dto.house_id.
                            ok_or(anyhow!(Error::ParseConfig("This is an edged game, house id must exist".to_string())))?,
                        None
                    )?;
                    Ok(Box::new(Fts::new(fts_config)?))
                },
                Err(e) => Err(Error::ParseConfig(e.to_string()).into())
            }
        },

        GameType::Cta => {
            match serde_json::from_str::<ConfigDto<CtaWagerType>>(payload) {
                Ok(config_dto) => {

                    if config_dto.wager_map.is_empty() {
                        return Err(Error::ParseConfig("Empty wager map".to_string()).into());
                    }

                    let cta_config = config::Cta::new(
                        config_dto.wager_map,
                        config_dto.house_id.
                            ok_or(anyhow!(Error::ParseConfig("This is an edged game, house id must exist".to_string())))?
                    )?;
                    Ok(Box::new(Cta::new(cta_config)?))
                },
                Err(e) => Err(Error::ParseConfig(e.to_string()).into())
            }
        }
        _ => Err(Error::UnknownGame.into())
    }
}