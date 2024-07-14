# Rust MySQL Example

A simple Rust application that uses SQLx to interact with a MySQL database.

More specifically, it manages a database of Stardew Valley characters (for the sake of an example). Each character has a name, a birthday, a favourite gift item and whether or not they're available for marriage.

You can read and write new characters in the database using terminal commands (todo).

## Installation

To run this project, you need to have Rust and MySQL / MariaDB installed on your machine.

1. [Install Rust.](https://www.rust-lang.org/tools/install)

2. [Install MySQL](https://dev.mysql.com/doc/refman/8.0/en/installing.html) or [Install MariaDB](https://mariadb.com/kb/en/getting-installing-and-upgrading-mariadb/)

3. Copy the already existing `.env.example` file in the root folder and rename it to `.env`. Change the credentials in the first line to be the ones that you have previously set up in your MySQL environment.

4. Run the setup script to create the database:
    ```console
    cargo run --bin setup_db
    ```

    Alternatively, you can create a database manually and just fill in the proper values in the `.env` file.

5. Finally, you can run the main application:
    ```console
    cargo run --bin main
    ```

## Usage

The application is used to manage a database of Stardew Valley characters. The following commands are available:

- Add a new character to the database:
    ```console
    add <name> <birthday_season> <birthday_day> <is_bachelor> <best_gift>
    ```
     Example: 
    ```console
    add abigail fall 13 true amethyst
    ```
- Reads a character from the database:
    ```console
    read <character_name>
    ```
    Example: 
    ```console
    read abigail
    ```
- Read all the characters from the database:
    ```console
    read all
    ```
-  Change a field of a character in the database:
    ```console
    change <character_name> <field_to_change> <new_value>
    ```
    Example: 
    ```console
    change abigail best_gift pizza
    ```
- Exit the application:
    ```console
    quit
    ``` 
