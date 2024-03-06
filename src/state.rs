mod cta;

pub use cta::CtaState;

#[derive(Debug, Eq, PartialEq)]
pub enum GameState<T> {
    Setup,
    Started(T), //T is intermediary states games may have once they have started
    Ended,
}

impl State for GameState<()> {

}

pub trait State {

}