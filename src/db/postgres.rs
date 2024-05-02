use sqlx::{Pool, Postgres, postgres::PgPoolOptions, Error as SqlxError};
use sqlx::Executor;
use axum::async_trait;
use tracing::{info, instrument};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Database {
    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

#[async_trait]
impl Database for Db {
    #[instrument]
    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        info!("Executing query: {}", query);
        sqlx::query(query)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
impl Db {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)  // Customize based on your load requirements.
            .after_connect(|conn, _meta| Box::pin(async move {
                // Execute multiple statements with one call.
                conn.execute("SET search_path TO message_store, public;")
                    .await?;

                Ok(())
            }))

            .connect(database_url)
            .await?;

        info!("Database connection pool created");
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}