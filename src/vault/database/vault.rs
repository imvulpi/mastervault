use rusqlite::Connection;

use crate::constants::database::{CREDENTIALS_ID, CREDENTIALS_LOGIN, CREDENTIALS_NAME, CREDENTIALS_NOTES, CREDENTIALS_TABLE_NAME, PASSWORDS_CREDENTIALS_ID, PASSWORDS_ID_NAME, PASSWORDS_PASSWORD_NAME, PASSWORDS_SALT_NAME, PASSWORDS_TABLE_NAME};

pub fn create_vault(db_name: &str) -> Result<&str, rusqlite::Error> {
    let sqlconn = rusqlite::Connection::open(db_name)?; // this also creates the DB :p
    create_credentials_table(&sqlconn);
    let db_key_sql = format!("CREATE TABLE IF NOT EXISTS {} (
            '{}'	INTEGER PRIMARY KEY AUTOINCREMENT,
            '{}'	TEXT,
            '{}'	TEXT,
            '{}' INTEGER NULL,
            FOREIGN KEY ({}) REFERENCES {}(id) ON DELETE SET NULL
        );", PASSWORDS_TABLE_NAME, PASSWORDS_ID_NAME, PASSWORDS_PASSWORD_NAME, PASSWORDS_SALT_NAME, PASSWORDS_CREDENTIALS_ID, PASSWORDS_CREDENTIALS_ID, CREDENTIALS_TABLE_NAME);
    println!("{}", &db_key_sql);
    sqlconn.execute(&db_key_sql, []);
    return Ok("Vault succesfully created.");
}

fn create_credentials_table(db_connection: &Connection){
    let credentials_table_sql = format!("CREATE TABLE IF NOT EXISTS {}(
        '{}'    INTEGER PRIMARY KEY AUTOINCREMENT,
        '{}'    TEXT,
        '{}'    TEXT,
        '{}'    TEXT
    );", CREDENTIALS_TABLE_NAME, CREDENTIALS_ID, CREDENTIALS_NAME, CREDENTIALS_LOGIN, CREDENTIALS_NOTES);
    println!("{}", &credentials_table_sql);
    db_connection.execute(&credentials_table_sql, []);
}