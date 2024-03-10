use serde::{Deserialize, Serialize};
use crate::transition::{GameTransition, Transition};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CtaTransition {
    Cut {
        deck_index: u8,
        position: u8
    }
}