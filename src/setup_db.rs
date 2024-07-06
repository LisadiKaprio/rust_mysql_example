use sqlx::mysql::*;
use std::env;
use std::error::Error;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in your .env file");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set in your .env file");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set in your .env file");
    let db_user = env::var("DB_USER").expect("DB_USER must be set in your .env file");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in your .env file");

    let pool = MySqlPool::connect(&url).await?;

    sqlx::query(
        &format!("CREATE DATABASE IF NOT EXISTS {}", db_name)
    ).execute(&pool)
    .await?;

    sqlx::query(
        &format!("CREATE USER IF NOT EXISTS '{}'@'{}' IDENTIFIED BY '{}'", db_user, db_host, db_password)
    ).execute(&pool)
    .await?;

    sqlx::query(
        &format!("GRANT ALL PRIVILEGES ON {}.* TO '{}'@'{}'", db_name, db_user, db_host)
    ).execute(&pool)
    .await?;

    sqlx::query(
        "FLUSH PRIVILEGES"
    ).execute(&pool)
    .await?;

    println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
    println!(" ");
    println!("Database setup complete! You can find it under the name '{}'", db_name);
    println!(" ");
    println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
    Ok(())
}
