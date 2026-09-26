use sqlx::SqlitePool;


pub struct DatabaseConnection {
    pub pool: SqlitePool,
}

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> Result<Self, sqlx:Error> {
        let pool = SqlitePool::connection(database_url).await?;

        Ok(Self { pool })
    }
}
