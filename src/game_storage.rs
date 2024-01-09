use std::collections::HashMap;
use uuid::Uuid;
use crate::game::Game;

pub struct GameStorage {
    map: HashMap<String, Box<dyn Game>>
}

impl GameStorage {
    pub fn new() -> Self {
        GameStorage {
            map: HashMap::new()
        }
    }

    pub fn insert_game(&mut self, game: Box<dyn Game>) -> String {
        let key = Uuid::new_v4().to_string();
        self.map.insert(key.clone(), game);
        key
    }

    pub fn get_game(&mut self, id: &str) -> Option<&dyn Game> {
        let mut game = self.map.get_mut(id)?;
        Some(game.as_mut())
    }

    pub fn remove_game(&mut self, id: &str) {
        self.map.remove(id);
    }
}