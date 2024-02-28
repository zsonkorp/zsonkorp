pub struct Wager<T> {
    id: u32,
    wager_type: T,
    amount: u32
}

impl<T> Wager<T> {
    pub fn new(id: u32, wager_type: T, amount: u32) -> Self {
        Wager { id, wager_type, amount }
    }
}