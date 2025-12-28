use tower_http::cors::{CorsLayer, Any};
use axum::{routing::get, routing::post, Router};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
mod database;
mod note;

use note::{get_notes, create_note};
use database::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let db= Database::new().await?;
    
    
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

