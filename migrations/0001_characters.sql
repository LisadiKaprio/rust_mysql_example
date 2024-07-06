CREATE TABLE characters (
    name VARCHAR(50) PRIMARY KEY NOT NULL,
    birthday_season VARCHAR(10) NOT NULL,
    birthday_day INTEGER NOT NULL,
    is_bachelor BOOLEAN NOT NULL
    best_gift VARCHAR(50) NOT NULL,
)