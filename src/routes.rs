use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{AppState, handlers::create_game_handler};

pub fn create_router(appState : Arc<AppState>)-> Router{
    Router::new()
        .route("/api/game", post(create_game_handler))
        .with_state(appState)
}