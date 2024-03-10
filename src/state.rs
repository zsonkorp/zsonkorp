mod cta;

pub use cta::CtaState;

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    Setup,
    Started,
    Ended
}


#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Game(GameState),
    Cta(CtaState)
}