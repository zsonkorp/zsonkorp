use std::collections::HashMap;
use serde::Deserialize;
use crate::player::Player;
use crate::wager::Wager;

#[derive(Deserialize)]
pub struct ConfigDto<T> {
    #[serde(rename = "wagers")]
    pub wager_map: HashMap<Player, Vec<Wager<T>>>,
    pub house_id: Option<String>
}