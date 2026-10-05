use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn hello_dunia() -> impl IntoResponse{
 let my_response = json!({
    "status": "success",
    "message": "Niajee wanangu"
 });

 Json(my_response)
}