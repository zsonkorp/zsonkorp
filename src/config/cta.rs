use std::collections::HashMap;
use crate::config::Config;
use crate::wager::Wager;
use anyhow::Result;
use serde::Deserialize;
use crate::player::Player;

#[derive(Eq, PartialEq, Deserialize)]
pub enum CtaWagerType {
    Forward,
    Reverse
}

pub struct Cta {
    base_config: Config<CtaWagerType>
}

impl Cta {
    pub fn new(wagers: HashMap<Player, Vec<Wager<CtaWagerType>>>, house_id: String) -> Result<Self> {
        let base_config = Config::new(wagers, house_id)?;

        Ok(Cta{ base_config })
    }

    pub fn get_base_config(&self) -> &Config<CtaWagerType> {
        &self.base_config
    }
}