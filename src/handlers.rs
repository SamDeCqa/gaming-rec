use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};
use sqlx::query_as;
use uuid::Uuid;

use crate::{AppState, model::{GameModel, UpdateGameModel}, schema::GameSchema};

pub async fn create_game_handler(
   State(data) : State<Arc<AppState>>,
   Json(body) : Json<GameSchema>
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
   let id = uuid::Uuid::new_v4();

   let game = sqlx::query_as!(
      GameModel,
      r#"INSERT INTO games (id, name, creator, plays) VALUES ($1, $2, $3, $4) RETURNING *"#,
      &id,
      &body.name,
      &body.creator,
      &body.plays,
   ).fetch_one(&data.db)
      .await
      .map_err(|e| e.to_string());

   if let Err(err) = game{
      if err.to_string().contains("duplicate key value") {
         let error_response = json!({
            "status" : "error",
            "message" : "Game already exixsts"
         });

         return  Err((StatusCode::CONFLICT, Json(error_response)));
      }

      return Err((
         StatusCode::INTERNAL_SERVER_ERROR,
         Json(json!({"status" : "error", "message" : format!("{:?}", err)}))
      ));
   }

   let game_response = json!({
      "status" : "success",
      "data" : json!({
         "game" : game
      })
   });

   Ok(Json(game_response))
}

pub async fn game_list_handler(
   State(data) : State<Arc<AppState>>
)-> Result<impl IntoResponse, (StatusCode, Json<Value>)>{
   let games = sqlx::query_as!(
      GameModel,
      r#"SELECT * FROM games ORDER BY created_at"#
   ).fetch_all(&data.db)
   .await
   .map_err(|e|{
      let error_response = json!({
         "status" : "error",
         "message" : format!("Database Error: {}", e)
      });

      (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
   })?;

   let games_response = json!({
      "status" : "success",
      "count" : games.len(),
      "data" : games
   });

   Ok(Json(games_response))
}

pub async fn get_game_handler(
   Path(game_id) : Path<Uuid>,
   State(data) : State<Arc<AppState>>
)-> Result<impl IntoResponse, (StatusCode, Json<Value>)>{

   let query_result = query_as!(
      GameModel,
      r#"SELECT * FROM games WHERE id = $1"#,
      &game_id
   ).fetch_one(&data.db)
   .await;

   match query_result {
      Ok(game) => {
         let game_response = json!({
            "status" : "success",
            "data" : json!({
               "game" : game
            })
         });

         Ok(Json(game_response))
      },
      Err(sqlx::Error::RowNotFound) => {
         let error_response = json!({
            "status" : "error",
            "message" : format!("The Game record with the id {:?} was not found", game_id)
         });

         Err((StatusCode::NOT_FOUND, Json(error_response)))
      },

      Err(e) => Err((
         StatusCode::INTERNAL_SERVER_ERROR,
         Json(json!({"status" : "error", "message" : format!("{:?}", e)}))
       ))
   }
}

pub async fn delete_game_handler(
   Path(game_id) : Path<Uuid>,
   State(data) : State<Arc<AppState>>
)->Result<impl IntoResponse, (StatusCode, Json<Value>)>{

   let query_result = sqlx::query_as!(
      GameModel,
      r#"DELETE FROM games WHERE id = $1 RETURNING *"#,
      game_id
   ).fetch_one(&data.db)
   .await;

   match query_result {
      Ok(_res) => Ok(Json(json!({
                  "status" : "success",
                  "message" : "Game record was deleted successfully"
               }))),

      Err(sqlx::Error::RowNotFound) => {
         let error_response = json!({
            "status" : "error",
            "message" : "No game with such id that was found"
         });

         Err((StatusCode::NOT_FOUND, Json(error_response)))
      },

      Err(e) => {
         let error_response = json!({
            "status" : "error",
            "message" : format!("Database error: {:?}", e)
         });

         Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
      }
   }
}

pub async fn edit_game_handle(
   Path(game_id) : Path<Uuid>,
   State(data) : State<Arc<AppState>>,
   Json(body) : Json<UpdateGameModel>
)->Result<impl IntoResponse, (StatusCode, Json<Value>)>
{
   let query_result = sqlx::query_as!(
      GameModel, 
      r#"SELECT * FROM games WHERE id = $1"#,
       &game_id
      ).fetch_one(&data.db)
      .await;

   let game = match query_result {
      Ok(game) => game,

      Err(sqlx::Error::RowNotFound) => {
         return Err((
            StatusCode::NOT_FOUND, 
            Json(json!({
               "status" : "error", 
               "message" : "No game record with such id that was found"
            }))
         ))
      },

      Err(e) => return Err((
         StatusCode::INTERNAL_SERVER_ERROR, 
         Json(json!({
         "status" : "error",
         "message" : format!("Could not delete a game record: {:?}", e)
         }))
      ))
   };

   let new_name = &body.name.as_ref().unwrap_or(&game.name);
   let new_creator = &body.creator.as_ref().unwrap_or(&game.creator);
   let new_plays = &body.plays.as_ref().unwrap_or(&game.plays);

   let updated_game = sqlx::query_as!(
      GameModel,
      r#"UPDATE games SET name = $1, creator = $2, plays = $3 WHERE id = $4 RETURNING *"#,
      new_name,
      new_creator,
      new_plays,
      &game.id
   ).fetch_one(&data.db)
   .await
   .map_err(|e| {
      let error_response = json!({
         "status" : "error",
         "message" : format!("Failed to Update game record: {:?}", e)
      });

      (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
   })?;

   let response = json!({
      "status" : "success",
      "message" : "Game record updated successfully",
      "data" : updated_game
   });

   Ok(Json(response))
}