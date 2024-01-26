use thiserror::Error;
use anyhow::{anyhow, Result};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

pub(crate) mod fts;
pub use fts::Fts;
use crate::{config, game};

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Generic game error")]
    Generic,
    #[error("Error parsing game configuration: {0}")]
    ParseConfig(String),
    #[error("Unknown game type")]
    UnknownGame
}

#[derive(Deserialize)]
pub(crate) enum GameType {
    Fts
}

pub trait Game: Sync + Send {
    fn my_type(&self) -> GameType;
    fn start(&mut self) -> Result<()>;
    fn get_result(&self) -> String;
}

pub fn create_game(game_type: &GameType, payload: &str) -> Result<Box<dyn Game>> {
    match game_type {
        GameType::Fts => {
            match serde_json::from_str(payload) {
                Ok(config) => Ok(Box::new(Fts::init(config)?)),
                Err(e) => Err(anyhow!(Error::ParseConfig(e.to_string())))
            }
        }
    }
}