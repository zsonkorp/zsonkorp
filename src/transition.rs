mod cta;

pub use cta::CtaTransition;
pub enum GameTransition<T> {
    StartGame,
    EndGame,
    AdvanceGame(T)
}

impl Transition for GameTransition<()> {

}

pub trait Transition {

}

