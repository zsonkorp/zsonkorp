use axum::{
    routing::get,
    routing::post,
    Router,
};
use axum::extract::State;
use crate::game_storage::GameStorage;

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
    let game_store = GameStorage::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/game", post(create_game));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_game(body: String) {
    todo!()
}