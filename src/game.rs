use thiserror::Error;
use crate::result::Payout;
use crate::state::State;

mod fts;
#[derive(Error, Debug)]
enum Error {
    #[error("Generic game error")]
    Generic
}

trait Game {
    type ConfigType;
    fn setup(&mut self, config: Self::ConfigType) -> Result<(), Error>;
    fn ready(&self) -> bool;
    fn start(&mut self) -> Result<State, Error>;

    fn advance(&mut self) -> Result<State, Error>;
    fn get_payout(&self) -> Result<Payout, Error>;
}