use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::transition::{GameTransition, Transition};
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid cut at deck: {0}, position: {1}")]
    InvalidCut(usize, usize),
    #[error("Unoptimal cut at position: {0}")]
    UnoptimalCut(usize)
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CtaTransition {
    Cut {
        deck_index: usize,
        position: usize
    }
}