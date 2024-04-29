use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use sqlx::Executor;

use tracing::{info, instrument};

#[derive(Debug, Clone)]
pub struct Db {
    pool: Pool<Postgres>,
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

    #[instrument(skip(self))]
    pub async fn execute_query(&self) -> Result<(), sqlx::Error> {
        info!("Executing a sample query");
        // Example query
        sqlx::query!("SELECT 1 AS test")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
