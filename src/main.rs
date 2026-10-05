use axum::{Router, routing::get};
mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(handlers::hello_dunia));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server iko tayari kupitia '0.0.0.0:8000'");

    axum::serve(listener, app).await.unwrap();
}
