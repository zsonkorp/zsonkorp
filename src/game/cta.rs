use crate::deck::Deck;
use crate::config;
use crate::game::{Game, GameType};

struct Cta {
    deck_pool: Vec<Deck>,
    config: config::Cta
}

impl Game for Cta {
    fn my_type(&self) -> GameType {
        GameType::Cta
    }

    fn start(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    fn get_result(&self) -> String {
        todo!()
    }
}