use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{
    routing::get,
    routing::post,
    Router,
};
use axum::extract::{Path, State};
use crate::config::fts::{Fts, Wager};
use crate::config::fts::WagerType::FullDeck;
use crate::game_storage::GameStorage;
use crate::player::Player;

mod card;
mod deck;
mod game;
mod config;
mod state;
mod player;
mod result;
mod game_storage;

#[tokio::main]
async fn main() {
    let game_store = Arc::new(Mutex::new(GameStorage::new()));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/game", post(create_game))
        .route("/game/:game_id", get(get_game_result).post(start_game))
        .with_state(game_store);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_game(State(state): State<Arc<Mutex<GameStorage>>>) -> String {
    let player = Player{ id: "player1".to_string() };
    let wager_map: HashMap<Player, Vec<Wager>> = HashMap::from(
        [
            (player, vec![Wager{ wager_type: FullDeck, amount: 100}])
        ]
    );

    let config = Fts::new(wager_map, "house".to_string());

    let mut storage = state.lock().unwrap();
    storage.insert_game(Box::new(game::fts::Fts::init(config).unwrap()))
}

async fn start_game(Path(id): Path<String>,
                    State(state): State<Arc<Mutex<GameStorage>>>) {
    let mut storage = state.lock().unwrap();
    let game = storage.get_game(&id).unwrap();
    game.start().unwrap();

}

async fn get_game_result(   Path(id): Path<String>,
                            State(state): State<Arc<Mutex<GameStorage>>>) -> String {
    let mut storage = state.lock().unwrap();
    let game = storage.get_game(&id).unwrap();
    game.get_result()
}