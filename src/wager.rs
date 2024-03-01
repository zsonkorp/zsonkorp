use serde::Deserialize;
use anyhow::{anyhow, Result};

#[derive(Deserialize)]
pub struct Wager<T> {
    id: u32,
    wager_type: T,
    pub amount: i32
}

impl<T> Wager<T> {
    pub fn new(id: u32, wager_type: T, amount: i32) -> Result<Self> {
        if amount == 0 {
            return Err(anyhow!("0 wager amount is not allowed"));
        }

        if amount < 0 {
            return Err(anyhow!("negative wager amount is not allowed"));
        }

        Ok(Wager { id, wager_type, amount })
    }

    pub fn get_wager_type(&self) -> &T {
        &self.wager_type
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }
}