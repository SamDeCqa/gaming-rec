use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState, handlers::{create_game_handler, delete_game_handler, edit_game_handle, game_list_handler, get_game_handler},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/game",
            post(create_game_handler).get(game_list_handler),
        )
        .route(
            "/api/game/{game}", 
            get(get_game_handler)
            .delete(delete_game_handler)
            .patch(edit_game_handle)
        )
        .with_state(app_state)
}
