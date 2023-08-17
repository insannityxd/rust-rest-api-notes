use std::error::Error;
use std::env;
use sqlx::PgPool;

pub async fn init() -> Result<PgPool, Box<dyn Error>> {
    let database_url = env::var("PG_DATABASE_URL")
        .expect("DATABASE_URL not set in .env");
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database migrations done.");
    println!("Successfully connected to database.");
    Ok(pool)
}