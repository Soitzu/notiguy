use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{MySqlPool};
use std::env;


pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;

        Ok( Self { pool })
    }
}
