use std::sync::Arc;

use axum::{Router, routing::get};
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::routes::create_router;
mod handlers;
mod model;
mod schema;
mod routes;

pub struct AppState {
    db: PgPool
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Please set rhe DB URL");

    let pool = match PgPoolOptions::new()
                .max_connections(10)
                .connect(&db_url)
                .await
     {
        Ok(pool)=> {
            println!("Successfully connected to the DataBase");
            pool
        },
        Err(err) => {
            println!("Failed to connect to the DB: {}", err);
            std::process::exit(1)
        }
    };

    let app =create_router(Arc::new(AppState { db: pool.clone()}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server iko tayari kupitia '0.0.0.0:8000'");

    axum::serve(listener, app).await.unwrap();
}
