use axum::{Router, routing::get, routing::post};
use tower_http::cors::{Any, CorsLayer};
mod db;
mod handler;
mod model;

use crate::handler::note::{create_note, get_notes};
use db::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = Database::new().await?;

    let router = Router::new()
        .route("/notes", get(get_notes))
        .route("/note", post(create_note))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Started Server at: {:?}", listener.local_addr().unwrap());
    println!("Waiting for incomming connections...");

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
