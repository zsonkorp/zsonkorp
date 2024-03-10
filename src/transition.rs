mod cta;

use serde::{Deserialize, Serialize};
pub use cta::CtaTransition;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameTransition {
    Start,
    End
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Transition {
    Game(GameTransition),
    Cta(CtaTransition)
}