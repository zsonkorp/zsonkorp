use std::error;
use crate::game;
use crate::deck::Deck;
use crate::game::{Error, Game};
use crate::result::Payout;
use crate::state::State;
use crate::config::fts::Fts as FtsConfig;

struct Fts {
    deck: Deck,
    config: Option<FtsConfig>,
    state: State,
    max_flop_count: u8
}
impl Fts {
    pub fn new(config: FtsConfig) -> Result<Self, Box<dyn error::Error>> {
        let mut fts = Fts {
            deck: Deck::default(),
            config: None,
            state: State::Setup,
            max_flop_count: 0,
        };

        fts.setup(config)?;
        Ok(fts)
    }
}
impl Game for Fts {

    type ConfigType = FtsConfig;
    fn setup(&mut self, config: Self::ConfigType) -> Result<(), game::Error> {
        self.config = Some(config);


        Ok(())
    }

    fn ready(&self) -> bool {
        todo!()
    }

    fn start(&mut self) -> Result<State, game::Error> {
        todo!()
    }

    fn advance(&mut self) -> Result<State, game::Error> {
        todo!()
    }

    fn get_payout(&self) -> Result<Payout, Error> {
        todo!()
    }
}