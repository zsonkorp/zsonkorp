use serde::Serialize;
use crate::wager::Wager;
use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("Amount must not be zero")]
    ZeroAmount,
    #[error("Player({0})'s loss: {1} must not exceeds wager amount: {2}")]
    LossExceedsWagerAmount(String, i32, i32),
}

#[derive(Debug, Serialize)]
pub struct Payout<'a> {
    player_id: &'a str,
    wager_id: Option<u32>,
    amount: i32     //positive -> player wins, negative -> player loses
}

impl<'a> Payout<'a> {
    pub fn new<T>(player_id: &'a str, opt_wager: Option<&Wager<T>>, amount: i32) -> Result<Self> {

        if amount == 0 {
            return Err(anyhow!(Error::ZeroAmount));
        }

        if let Some(wager) = opt_wager {
            if amount < 0 && amount < *wager.get_amount() {
                return Err(anyhow!(Error::LossExceedsWagerAmount(player_id.to_string(), amount, *wager.get_amount())))
            } else {
                return Ok( Payout{ player_id, wager_id: Some(*wager.get_id()), amount } );
            }
        }

        // No wager? This happens when we are creating payout for the house
        // Maybe separate into another function to skip Option<wager> check
        Ok(Payout { player_id, wager_id: None, amount })
    }
}