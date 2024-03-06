use crate::state::{GameState, State};

pub enum CtaState {
    AwaitCut
}

impl State for CtaState {

}

impl State for GameState<CtaState> {

}