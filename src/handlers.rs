use anyhow::{anyhow, Error};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use crate::game;
use crate::app_state::AppState;
use crate::game::Error::NotFound;
use crate::game::GameType::*;
use crate::transition::Transition;

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

pub async fn create_game(State(state): State<AppState>,
                         Query(type_param) : Query<GameTypeQuery>,
                         body: String) -> Result<String, AnyhowError> {

    let game = game::create_game(&type_param.kind, &body)?;
    let mut storage = state.game_store.lock().unwrap();
    Ok(storage.insert_game(game))
}

pub async fn retrieve_game(Path(id): Path<String>,
                           State(state): State<AppState>) -> Result<String, AnyhowError> {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).ok_or(NotFound(id))?;

    Ok(serde_json::to_string(game)?)
}

pub async fn transition_game(Path(id): Path<String>,
                             State(state): State<AppState>,
                             body: String)  -> Result<String, AnyhowError> {

    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).ok_or(NotFound(id))?;
    match game.get_type() {
        Fts => game.transition(serde_json::from_str(&body)?)?,
        Cta => game.transition(serde_json::from_str(&body)?)?
    }

    Ok(serde_json::to_string(game)?)
}

pub async fn get_transitions(Path(id): Path<String>,
                             State(state): State<AppState>) -> Result<Json<Vec<Transition>>, AnyhowError> {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).ok_or(NotFound(id))?;
    Ok(Json(game.get_valid_transitions()))
}

pub async fn get_payout(Path(id): Path<String>,
                        State(state): State<AppState>) -> Result<String, AnyhowError> {
    let mut storage = state.game_store.lock().unwrap();
    let game = storage.get_game(&id).ok_or(NotFound(id))?;
    match serde_json::to_string(&game.get_payout()?) {
        Ok(payout_json) => Ok(payout_json),
        Err(e) => Err(AnyhowError::from(e))
    }
}