use std::collections::HashMap;
use crate::player::Player;

enum Wager {
    FullDeck,
    AtFlop(u8),
    FlopRange(u8)
}

pub(crate) struct Fts {
    players: Vec<Player>,
    wagers: HashMap<Player, Vec<Wager>>
}