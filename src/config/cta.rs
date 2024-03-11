use std::collections::HashMap;
use crate::config::Config;
use crate::wager::Wager;
use anyhow::Result;
use serde::Deserialize;
use crate::player::Player;

pub struct Odds {
    forward: i32,
    reverse: i32,
    pair: i32

}

impl Odds {
    pub fn get_forward(&self) -> &i32 {
        &self.forward
    }

    pub fn get_reverse(&self) -> &i32 {
        &self.reverse
    }

    pub fn get_pair(&self) -> &i32 {
        &self.pair
    }
}

impl Default for Odds {
    fn default() -> Self {
       Odds {
            forward: 4,
            reverse: 5,
            pair: 20
        }
    }
}

#[derive(Eq, PartialEq, Deserialize)]
pub enum CtaWagerType {
    Forward,
    Reverse
}

pub struct Cta {
    base_config: Config<CtaWagerType>,
    odds: Odds,
    fast: bool      // fast cta, not yet supported
}

impl Cta {
    pub fn new(wagers: HashMap<Player, Vec<Wager<CtaWagerType>>>, house_id: String) -> Result<Self> {
        let base_config = Config::new(wagers, house_id)?;

        Ok(Cta{ base_config, odds: Odds::default(), fast: false })
    }

    pub fn get_base_config(&self) -> &Config<CtaWagerType> {
        &self.base_config
    }

    pub fn get_odds(&self) -> &Odds {
        &self.odds
    }
}