use std::collections::HashMap;
use anyhow::Error;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use crate::config;
use crate::game;
use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct GameTypeQuery {
    kind: game::GameType
}

pub struct AnyhowError(Error);

impl IntoResponse for AnyhowError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AnyhowError where E: Into<Error>{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

pub(crate) async fn create_game(State(state): State<AppState>,
                                Query(typeParam) : Query<GameTypeQuery>,
                                body: String) -> Result<String, AnyhowError> {

    let game = game::create_game(&typeParam.kind, &body)?;
    let mut storage = state.game_store.lock().unwrap();
    Ok(storage.insert_game(game))
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