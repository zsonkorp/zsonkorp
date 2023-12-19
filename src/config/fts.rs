use std::collections::HashMap;
use crate::player::Player;

pub enum Wager {
    FullDeck,
    AtFlop(u8),
    FlopRange(u8, u8)
}

pub(crate) struct Fts {
    players: Vec<Player>,
    pub wagers: HashMap<Player, Vec<Wager>>
}