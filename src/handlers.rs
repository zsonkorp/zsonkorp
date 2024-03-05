use anyhow::{anyhow, Error};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
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
                                Query(type_param) : Query<GameTypeQuery>,
                                body: String) -> Result<String, AnyhowError> {

    let game = game::create_game(&type_param.kind, &body)?;
    let mut storage = state.game_store.lock().unwrap();
    Ok(storage.insert_game(game))
}

pub(crate) async fn start_game(Path(id): Path<String>,
                               State(state): State<AppState>) {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).unwrap();
    game.advance_state().unwrap();

}

pub(crate) async fn get_game_result(Path(id): Path<String>,
                                    State(state): State<AppState>) -> Result<String, AnyhowError> {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).unwrap();
    match serde_json::to_string(&game.get_payout()?) {
        Ok(payout_json) => Ok(payout_json),
        Err(e) => Err(AnyhowError::from(e))
    }
}