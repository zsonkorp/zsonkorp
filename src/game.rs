use thiserror::Error;
use anyhow::Result;

pub(crate) mod fts;
#[derive(Error, Debug)]
enum Error {
    #[error("Generic game error")]
    Generic
}

enum GameType {
    Fts
}
pub trait Game: Sync + Send {
    fn my_type(&self) -> GameType;
    fn start(&mut self) -> Result<()>;
    fn get_result(&self) -> String;
}