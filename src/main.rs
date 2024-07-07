use sqlx::{migrate, mysql::*, query};
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

impl Character {
    fn new(name: String, birthday_season: Season, birthday_day: u8, is_bachelor: bool, best_gift: String) -> Character {
        Character {
            name, 
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift
        }
    }

    async fn add_to_database(&self, pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
        let creation_query = "INSERT INTO characters (name, birthday_season, birthday_day, is_bachelor, best_gift) VALUES (?, ?, ?, ?, ?)";
    
        let query_result = query(creation_query).bind(&self.name)
            .bind(&self.birthday_season.as_ref())
            .bind(&self.birthday_day)
            .bind(&self.is_bachelor)
            .bind(&self.best_gift)
            .execute(pool)
            .await;
    
        match query_result {
            Ok(_) => {
                let message = format!("{} was successfully added to the database! :)", &self.name);
                print_aesthetic_message(message);
                Ok(())
            },
            Err(e) => {
                let message = format!("An error occured when adding character {}! {}", &self.name, e.to_string());
                print_aesthetic_message(message);
                Ok(())
            }
        }    
    }
}

fn print_aesthetic_message (message: String) {
    println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
    println!(" ");
    println!("{}", message);
    println!(" ");
    println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
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
    migrate!("./migrations").run(&pool).await?;

    println!("Following seasons are available: {}, {}, {} and {}.", Season::Spring.as_ref(), Season::Summer.as_ref(), Season::Fall.as_ref(), Season::Winter.as_ref());

    let existing_characters: Vec<Character> = vec![
        Character {
            name: "Abigail".to_string(),
            birthday_season: Season::Fall,
            birthday_day: 13,
            is_bachelor: true,
            best_gift: "Amethyst".to_string()
        },
        Character {
            name: "Caroline".to_string(),
            birthday_season: Season::Winter,
            birthday_day: 7,
            is_bachelor: false,
            best_gift: "Fish Taco".to_string()
        },
        Character {
            name: "Haley".to_string(),
            birthday_season: Season::Spring,
            birthday_day: 14,
            is_bachelor: true,
            best_gift: "Coconut".to_string()
        },
        Character {
            name: "Lewis".to_string(),
            birthday_season: Season::Spring,
            birthday_day: 7,
            is_bachelor: false,
            best_gift: "Autumn's Beauty".to_string()
        },
        Character {
            name: "Leah".to_string(),
            birthday_season: Season::Winter,
            birthday_day: 23,
            is_bachelor: true,
            best_gift: "Goat Cheese".to_string()
        }
    ];

    for character in &existing_characters {
        character.add_to_database(&pool).await?;
    };

    Ok(())
}
