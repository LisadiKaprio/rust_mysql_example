pub mod Commands {
    use sqlx::{migrate, mysql::*, query, Row};
    use std::{error::Error, str::FromStr};

    use crate::{Character, Season};

    use super::*;

    #[derive(PartialEq)] 
    pub enum Command {
        Add,
        Read,
        Change,
        Quit,
    }
    
    pub async fn execute_command(pool: &MySqlPool, command: &str, arguments: Vec<&str>) -> Option<Command> {
        match command {
            "add" => {
                println!("TODO: add character - {}", arguments[0]);
                Some(Command::Add)
            },
            "read" => {
                handle_read_command(pool, arguments).await;
                Some(Command::Read)
            },
            "quit" => {
                println!("Quitting the program.");
                Some(Command::Quit)
            },
            _ => {
                println!("❓ Command does not exist.");
                None
            }
        }
    }

    async fn handle_read_command(pool: &MySqlPool, arguments: Vec<&str>) -> Result<(), Box<dyn Error>> {
        if arguments[0] == "all" {
            read_all(pool).await
        } else {
            // read_character(pool, arguments).await?
            println!("Trying to read character");
            Ok(())
        }
    }
    
    async fn read_all(pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
        let read_query = "SELECT * FROM characters";
    
        let rows = query(read_query).fetch_all(pool).await?;
    
        let mut characters: Vec<Character> = vec![];
    
        for row in rows {
            characters.push(Character {
                name: row.get("name"),
                birthday_season: Season::from_str(row.get("birthday_season")).unwrap(),
                birthday_day: row.get::<i32, &str>("birthday_day") as u8,
                is_bachelor: row.get::<bool, &str>("is_bachelor") as bool,
                best_gift: row.get("best_gift")
            });
        }
    
        for character in characters {
            character.print_info();
        }
    
        Ok(())
    }
}