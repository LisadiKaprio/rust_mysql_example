pub mod terminal_commands {
    use crate::{print_aesthetic_message, Character, Season};
    use sqlx::{mysql::*, query, Row};
    use std::{error::Error, str::FromStr};
    use strum_macros::{AsRefStr, EnumString};

    #[derive(PartialEq)]
    pub enum Command {
        Add,
        Read,
        Change,
        Quit,
        None,
    }

    #[allow(non_camel_case_types)]
    #[derive(AsRefStr, EnumString)]
    enum DbValue {
        #[strum(ascii_case_insensitive)]
        Name,
        #[strum(ascii_case_insensitive)]
        Birthday_Season,
        #[strum(ascii_case_insensitive)]
        Birthday_Day,
        #[strum(ascii_case_insensitive)]
        Is_Bachelor,
        #[strum(ascii_case_insensitive)]
        Best_Gift,
    }

    pub async fn execute_command(
        pool: &MySqlPool,
        command: &str,
        arguments: Vec<&str>,
    ) -> Result<Command, Box<dyn Error>> {
        match command {
            "add" => {
                let executed_command = handle_adding_character(pool, arguments).await;
                match executed_command {
                    Ok(_) => Ok(Command::Add),
                    Err(e) => Err(e),
                }
            }
            "read" => {
                let executed_command = handle_read_command(pool, arguments).await;
                match executed_command {
                    Ok(_) => Ok(Command::Read),
                    Err(e) => Err(e),
                }
            }
            "change" => {
                let executed_command = handle_changing_character(pool, arguments).await;
                match executed_command {
                    Ok(_) => Ok(Command::Change),
                    Err(e) => Err(e),
                }
            }
            "quit" => {
                println!("Quitting the program.");
                Ok(Command::Quit)
            }
            _ => {
                println!("❓ Command does not exist.");
                Ok(Command::None)
            }
        }
    }

    async fn convert_row_to_character(row: MySqlRow) -> Character {
        let name = row.get::<String, _>("name");
        let birthday_season = Season::from_str(&row.get::<String, _>("birthday_season")).unwrap();
        let birthday_day = row.get::<i32, _>("birthday_day") as u8;
        let is_bachelor = row.get::<bool, _>("is_bachelor");
        let best_gift = row.get::<String, _>("best_gift");
        Character {
            name,
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift,
        }
    }

    async fn handle_read_command(
        pool: &MySqlPool,
        arguments: Vec<&str>,
    ) -> Result<(), Box<dyn Error>> {
        if arguments.is_empty() {
            println!("Provide an argument, like 'all' to read all characters or 'Abigail' to read specific character.");
            return Ok(());
        }
        if arguments[0] == "all" {
            read_all(pool).await
        } else {
            read_character(pool, arguments.join("")).await
        }
    }

    async fn read_all(pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
        let read_query = "SELECT * FROM characters";

        let rows = query(read_query).fetch_all(pool).await?;

        let mut characters: Vec<Character> = vec![];

        for row in rows {
            characters.push(convert_row_to_character(row).await);
        }

        for character in characters {
            character.print_info();
        }

        Ok(())
    }

    async fn read_character(
        pool: &MySqlPool,
        character_name: String,
    ) -> Result<(), Box<dyn Error>> {
        let read_query = "SELECT * FROM characters WHERE name = ?";

        let row = query(read_query)
            .bind(&character_name)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(existing_row) => {
                let character = convert_row_to_character(existing_row).await;
                character.print_info();
            }
            None => print_aesthetic_message(vec![format!(
                "Sorry, I can't find {} in the database!",
                &character_name
            )]),
        }

        Ok(())
    }

    async fn handle_adding_character(
        pool: &MySqlPool,
        arguments: Vec<&str>,
    ) -> Result<(), Box<dyn Error>> {
        if arguments.is_empty() || arguments.len() < 5 {
            println!("‼ Please provide arguments in the following order: name, birthday season, birthday day, bachelor status, best gift. Right now, I see you only entered {} arguments, whereas I need 5.", arguments.len());
            println!(
                "For example for bachelorette Abigail this would be: abigail fall 13 true amethyst"
            );
            println!("Give it a try! :)");
            return Ok(());
        }

        let name = arguments[0].to_string();

        let birthday_season_result = string_to_season(arguments[1]);
        let birthday_season = match birthday_season_result {
            Some(s) => s,
            None => return Ok(()),
        };

        let birthday_day_result = string_to_day(arguments[2]);
        let birthday_day: u8 = match birthday_day_result {
            Some(d) => d,
            None => return Ok(()),
        };

        let is_bachelor_result = string_to_bachelor_bool(arguments[3]);
        let is_bachelor: bool = match is_bachelor_result {
            Some(b) => b,
            None => return Ok(()),
        };

        let best_gift = arguments[4..].join(" ");

        let character_to_add = Character {
            name,
            birthday_season,
            birthday_day,
            is_bachelor,
            best_gift,
        };
        character_to_add.add_to_database(pool, true, true).await?;

        Ok(())
    }

    async fn handle_changing_character(
        pool: &MySqlPool,
        arguments: Vec<&str>,
    ) -> Result<(), Box<dyn Error>> {
        if arguments.is_empty() || arguments.len() < 3 {
            println!("‼ Please provide arguments in the following order: name of the character you want to change, name of the value you want to change, then the value you want to set it to. Right now, I see you only entered {} arguments, whereas I need 3.", arguments.len());
            println!("For example , if you want to change Abigail's birthday season, you need to write the following arguments: 'abigail birthday_season summer'.");
            println!("The following value names are available: name, birthday_season, birthday_day, is_bachelor, best_gift");
            println!("Give it a try! :)");
            return Ok(());
        }

        let character_name = arguments[0].to_string();

        let value_name_result = DbValue::from_str(&arguments[1].to_lowercase());
        let value_name = match value_name_result {
            Ok(v) => v,
            Err(_) => {
                println!("Couldn't recognize value name you typed!");
                println!("The following value names are available: name, birthday_season, birthday_day, is_bachelor, best_gift");
                return Ok(());
            }
        };

        let change_query: query::Query<MySql, MySqlArguments>;
        let query_string;
        match value_name {
            DbValue::Name | DbValue::Best_Gift => {
                query_string = format!(
                    "UPDATE characters SET {} = ? WHERE name = ?",
                    value_name.as_ref().to_lowercase()
                );
                change_query = query(&query_string)
                    .bind(arguments[2..].join(" "))
                    .bind(&character_name);
            }
            DbValue::Birthday_Season => {
                let birthday_season_result = string_to_season(arguments[2]);
                match birthday_season_result {
                    Some(s) => {
                        query_string = format!(
                            "UPDATE characters SET {} = ? WHERE name = ?",
                            value_name.as_ref().to_lowercase()
                        );
                        change_query = query(&query_string)
                            .bind(s.as_ref().to_lowercase())
                            .bind(&character_name);
                    }
                    None => return Ok(()),
                }
            }
            DbValue::Birthday_Day => {
                let day = string_to_day(arguments[2]);
                match day {
                    Some(d) => {
                        query_string = format!(
                            "UPDATE characters SET {} = ? WHERE name = ?",
                            value_name.as_ref().to_lowercase()
                        );
                        change_query = query(&query_string).bind(d).bind(&character_name);
                    }
                    None => return Ok(()),
                }
            }
            DbValue::Is_Bachelor => {
                let bool = string_to_bachelor_bool(arguments[2]);
                match bool {
                    Some(b) => {
                        query_string = format!(
                            "UPDATE characters SET {} = ? WHERE name = ?",
                            value_name.as_ref().to_lowercase()
                        );
                        change_query = query(&query_string).bind(b).bind(&character_name);
                    }
                    None => return Ok(()),
                }
            }
        }

        let result = change_query.execute(pool).await;
        match result {
            Ok(_) => {
                print_aesthetic_message(vec!["✅ The change took place! Try the command 'read' with the character's name to check out your changes."]);
                Ok(())
            }
            Err(_e) if matches!(sqlx::Error::RowNotFound, _e) => {
                print_aesthetic_message(vec![format!(
                    "Can't find character with the name '{}' in the database!",
                    &character_name
                )]);
                Ok(())
            }
            Err(e) => {
                print_aesthetic_message(vec![
                    "Error happened when trying to change character in database!",
                    &format!("{e}"),
                ]);
                Ok(())
            }
        }
    }

    fn string_to_season(string: &str) -> Option<Season> {
        let birthday_season_result = Season::from_str(string);
        match birthday_season_result {
            Ok(season) => Some(season),
            Err(_) => {
                println!("‼ Please provide a proper argument for 'birthday_season' value. I received '{}', meanwhile I can only interpret 'spring', 'summer', 'fall' or 'winter'.", string);
                None
            }
        }
    }

    fn string_to_day(string: &str) -> Option<u8> {
        let birthday_day_result = string.parse::<u8>();
        match birthday_day_result {
            Ok(number) if number <= 28 => Some(number),
            _ => {
                println!("‼ Please provide a proper number for the 'birthday_day' value. Remember that Stardew Valley Seasons only have 28 days! (I received {} from you.)", string);
                None
            }
        }
    }

    fn string_to_bachelor_bool(string: &str) -> Option<bool> {
        if string.to_lowercase() == "true" {
            Some(true)
        } else if string.to_lowercase() == "false" {
            Some(false)
        } else {
            println!("‼ Please provide a proper argument for 'is_bachelor' value. I received '{}', meanwhile I can only interpret 'true' or 'false'.", string);
            None
        }
    }
}
