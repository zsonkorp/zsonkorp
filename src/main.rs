use axum::{routing::get, routing::post, Router};
use crate::app_state::AppState;

mod card;
mod deck;
mod game;
mod config;
mod state;
mod player;
mod result;
mod game_storage;
mod handlers;
mod app_state;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/game", post(handlers::create_game))
        .route(
            "/game/:game_id", get(handlers::get_game_result).post(handlers::start_game)
        )
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}