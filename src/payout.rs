use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Payout<'a> {
    player_id: &'a str,
    wager_id: u32,
    amount: i32
}

impl<'a> Payout<'a> {
    pub fn new(player_id: &'a str, wager_id: u32, amount: i32) -> Self {
        Payout { player_id, wager_id, amount }
    }
}