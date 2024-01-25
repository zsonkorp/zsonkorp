use std::sync::{Arc, Mutex};
use crate::game_storage::GameStorage;

#[derive(Clone)]
pub(crate) struct AppState {
    pub game_store: Arc<Mutex<GameStorage>>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            game_store: Arc::new(Mutex::new(GameStorage::new()))
        }
    }
}