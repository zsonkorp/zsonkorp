use crate::transition::{GameTransition, Transition};

pub enum CtaTransition {
    Cut(u8, u8),
    Quit
}

impl Transition for GameTransition<CtaTransition> {

}

impl Transition for CtaTransition {

}