pub mod fts;
mod cta;

use std::collections::HashMap;
pub use cta::Cta;
pub use fts::Fts;
use crate::player::Player;

pub struct Config<W> {
    wagers: HashMap<Player, Vec<W>>,
    house_id: String
}

impl<W> Config<W> {
    pub fn new(wagers: HashMap<Player, Vec<W>>, house_id: String) -> Self {
        Config { wagers, house_id }
    }
}