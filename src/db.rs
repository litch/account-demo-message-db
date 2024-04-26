use sqlx::{Pool, Postgres};
use tracing::{info, instrument};

pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        // Set the search path to use the message_store schema
        sqlx::query("SET search_path TO message_store, public;")
            .execute(&pool)
            .await?;
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
