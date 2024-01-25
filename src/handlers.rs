use axum::extract::{Path, State};
use axum::Json;
use crate::config;
use crate::game;
use crate::app_state::AppState;

pub(crate) async fn create_game(State(state): State<AppState>,
                                Json(config) : Json<config::Fts>) -> String {

    let mut storage = state.game_store.lock().unwrap();
    storage.insert_game(Box::new(game::Fts::init(config).unwrap()))
}

pub(crate) async fn start_game(Path(id): Path<String>,
                               State(state): State<AppState>) {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).unwrap();
    game.start().unwrap();

}

pub(crate) async fn get_game_result(Path(id): Path<String>,
                                    State(state): State<AppState>) -> String {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).unwrap();
    game.get_result()
}