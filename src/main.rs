use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(hello_dunia));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server iko tayari kupitia '0.0.0.0:8000'");

    axum::serve(listener, app).await.unwrap();
}

async fn hello_dunia() -> impl IntoResponse{
 let my_response = json!({
    "status": "success",
    "message": "Niajee wanangu"
 });

 Json(my_response)
}
