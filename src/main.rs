use sqlx::mysql::*;
use std::env;
use std::error::Error;
use dotenv::dotenv;
use strum_macros::{EnumString, AsRefStr};

#[derive(Debug, AsRefStr, EnumString)]
enum Season {
    Spring,
    Summer,
    Fall,
    Winter
}

struct Character {
    name: String,
    birthday_season: Season,
    birthday_day: u8,
    is_bachelor: bool,
    best_gift: String
}


async fn connect_to_db() -> Result<MySqlPool, sqlx::Error>{
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in your .env file");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set in your .env file");

    let full_db_path = format!("{}/{}", url, db_name);
    
    MySqlPool::connect(&full_db_path).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let pool = connect_to_db().await?;

    println!("Following seasons are available: {}, {}, {} and {}.", Season::Spring.as_ref(), Season::Summer.as_ref(), Season::Fall.as_ref(), Season::Winter.as_ref());

    Ok(())
}
