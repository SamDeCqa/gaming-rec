use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::{create_game_handler, game_list_handler}};

pub fn create_router(app_state : Arc<AppState>)-> Router{
    Router::new()
        .route("/api/game", post(create_game_handler))
        .route("/api/game", get(game_list_handler))
        .with_state(app_state)
}