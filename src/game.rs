use thiserror::Error;
use crate::result::Payout;
use crate::state::State;

mod fts;
#[derive(Error, Debug)]
enum Error {
    #[error("Generic game error")]
    Generic
}

enum GameType {
    Fts
}
pub trait Game {
    fn my_type(&self) -> GameType;
}