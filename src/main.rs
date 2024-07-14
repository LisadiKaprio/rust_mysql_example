use dotenv::dotenv;
use sqlx::{migrate, mysql::*, query};
use std::env;
use std::error::Error;
use std::io;
use strum_macros::{AsRefStr, EnumString, IntoStaticStr};
mod commands;
use commands::terminal_commands::*;

#[derive(Debug, IntoStaticStr, AsRefStr, EnumString)]
enum Season {
    #[strum(ascii_case_insensitive)]
    Spring,
    #[strum(ascii_case_insensitive)]
    Summer,
    #[strum(ascii_case_insensitive)]
    Fall,
    #[strum(ascii_case_insensitive)]
    Winter,
}

struct Character {
    name: String,
    birthday_season: Season,
    birthday_day: u8,
    is_bachelor: bool,
    best_gift: String,
}

impl Character {
    fn _new(
        name: String,
        birthday_season: Season,
        birthday_day: u8,
        is_bachelor: bool,
        best_gift: String,
    ) -> Character {
        Character {
            name,
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift,
        }
    }

    async fn add_to_database(
        &self,
        pool: &MySqlPool,
        notify_success: bool,
        notify_error: bool,
    ) -> Result<(), Box<dyn Error>> {
        let creation_query = "INSERT INTO characters (name, birthday_season, birthday_day, is_bachelor, best_gift) VALUES (?, ?, ?, ?, ?)";

        let query_result = query(creation_query)
            .bind(&self.name)
            .bind(&self.birthday_season.as_ref())
            .bind(&self.birthday_day)
            .bind(&self.is_bachelor)
            .bind(&self.best_gift)
            .execute(pool)
            .await;

        match query_result {
            Ok(_) => {
                if notify_success {
                    let message =
                        format!("{} was successfully added to the database! :)", &self.name);
                    print_aesthetic_message(message);
                }
                Ok(())
            }
            Err(e) => {
                if notify_error {
                    let message = format!(
                        "An error occured when adding character {}! {}",
                        &self.name,
                        e.to_string()
                    );
                    print_aesthetic_message(message);
                }
                Ok(())
            }
        }
    }

    fn print_info(&self) {
        println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
        println!(" ");
        println!(
            "{}' birthday: {} {}",
            &self.name,
            self.birthday_season.as_ref(),
            &self.birthday_day
        );
        println!("{}'s favourite gift: {}", &self.name, &self.best_gift);
        let can_get_married = if self.is_bachelor {
            "can get married to the player! ❤"
        } else {
            "can NOT get married to the player! 💔"
        };
        println!("{} {}", &self.name, can_get_married);
        println!(" ");
        println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
    }
}

// add character by user input
// user should type: "add leah spring 3 pizza true"

// read all characters
// user should type: "read all"

// read character by name
// user should type: "read abigail"
// output will be:  Abigail's birthday: Spring 27
//                  Abigail's favourite gift: Amethyst
//                  Abigail can get married to the player!

// edit character by name
// user should type: "change abigail best_gift pizza"
// UPDATE characters SET ? = ? WHERE name = ? (bind parameter, updated value, name)

fn print_aesthetic_message(message: String) {
    println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
    println!(" ");
    println!("{}", message);
    println!(" ");
    println!("•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•°•");
}

async fn connect_to_db() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let url =
        env::var("ROOT_DATABASE_URL").expect("ROOT_DATABASE_URL must be set in your .env file");

    let db_protocol = env::var("DB_PROTOCOL").expect("DB_PROTOCOL must be set in your .env file");
    let db_user = env::var("DB_USER").expect("DB_USER must be set in your .env file");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in your .env file");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set in your .env file");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set in your .env file");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set in your .env file");

    let full_db_path = format!(
        // example: mysql://test_user:password@localhost:3306/test_db3
        "{}://{}:{}@{}:{}/{}",
        db_protocol, db_user, db_password, db_host, db_port, db_name
    );

    MySqlPool::connect(&full_db_path).await
}

async fn setup_initial_values(pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
    let existing_characters: Vec<Character> = vec![
        Character {
            name: "Abigail".to_string(),
            birthday_season: Season::Fall,
            birthday_day: 13,
            is_bachelor: true,
            best_gift: "Amethyst".to_string(),
        },
        Character {
            name: "Caroline".to_string(),
            birthday_season: Season::Winter,
            birthday_day: 7,
            is_bachelor: false,
            best_gift: "Fish Taco".to_string(),
        },
        Character {
            name: "Haley".to_string(),
            birthday_season: Season::Spring,
            birthday_day: 14,
            is_bachelor: true,
            best_gift: "Coconut".to_string(),
        },
        Character {
            name: "Lewis".to_string(),
            birthday_season: Season::Spring,
            birthday_day: 7,
            is_bachelor: false,
            best_gift: "Autumn's Beauty".to_string(),
        },
        Character {
            name: "Leah".to_string(),
            birthday_season: Season::Winter,
            birthday_day: 23,
            is_bachelor: true,
            best_gift: "Goat Cheese".to_string(),
        },
    ];

    for character in &existing_characters {
        character.add_to_database(pool, false, false).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = connect_to_db().await?;
    migrate!("./migrations").run(&pool).await?;
    setup_initial_values(&pool).await?;

    loop {
        println!("Type your command here:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let parts: Vec<_> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        };

        let command = parts[0];
        let arguments;
        if parts.len() <= 1 {
            arguments = vec![];
        } else {
            arguments = parts[1..].to_vec();
        }

        let executed_command = execute_command(&pool, command, arguments).await;
        if let Ok(Command::Quit) = executed_command {
            break;
        }
    }

    Ok(())
}
