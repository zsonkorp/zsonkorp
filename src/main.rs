use axum::{routing::get, routing::post, Router};
use crate::app_state::AppState;

mod card;
mod deck;
mod game;
mod config;
mod state;
mod player;
mod game_storage;
mod handlers;
mod app_state;
mod wager;
mod payout;
mod dto;
mod transition;
mod serializers;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(app_ascii_art))
        .route("/game", post(handlers::create_game))
        .route("/game/:id", get(handlers::retrieve_game))
        .route("/game/:id/payout", get(handlers::get_payout))
        .route(
            "/game/:id/transitions",
            get(handlers::get_transitions).post(handlers::transition_game)
        )
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn app_ascii_art() -> &'static str {
    r#"
                     _
 _______  ___  _ __ | | _____  _ __ _ __         __ _  __ _ _ __ ___   ___
|_  / __|/ _ \| '_ \| |/ / _ \| '__| '_ \ _____ / _` |/ _` | '_ ` _ \ / _ \
 / /\__ \ (_) | | | |   < (_) | |  | |_) |_____| (_| | (_| | | | | | |  __/
/___|___/\___/|_| |_|_|\_\___/|_|  | .__/       \__, |\__,_|_| |_| |_|\___|
                                   |_|          |___/
"#
}