use std::{error::Error, fs::File};
use rusqlite::Connection;

use crate::{client_side::{self, key::user_create_master_password}, commons::{fields::{enums::database::{self, db_strings::DatabaseStrings::{ConfigEmptyOrMissing, CreatingVaultFile, ErrorMessageMarker, ErrorOccurred, KeyEmptyOrMissing, SuccessfulMarker}}, structs::errors}, file_ops::controller::FileChecker}, constants::{database::{PASSWORDS_ID_NAME, PASSWORDS_TABLE_NAME, VAULT_DB_NAME}, general::values::MAX_TRIES}, vault::{self, key::master_key::MasterKey}};

pub fn check_and_fix_required_files() -> Result<&'static str, Box<dyn Error>> {
    if FileChecker::is_empty("config.txt") {
        println!("{}:", ConfigEmptyOrMissing.text());
        client_side::config::user_create_config_file();
    }

    if !FileChecker::file_exists(VAULT_DB_NAME){
        println!("{}", CreatingVaultFile.text());
        match vault::database::vault::create_vault(VAULT_DB_NAME) {
            Ok(_) => {
                println!("{}!", SuccessfulMarker.text());
            },
            Err(error) => {
                println!("{} {} {}", ErrorOccurred.text(), ErrorMessageMarker.text(), error);
            }
        }
    }else{
        
    }

    let vault_conn = rusqlite::Connection::open(VAULT_DB_NAME);
    match vault_conn {
        Ok(db_conn) => {
            let master_sql = format!("SELECT * FROM {} WHERE {}.{} == 0;", PASSWORDS_TABLE_NAME, PASSWORDS_TABLE_NAME, PASSWORDS_ID_NAME);
            let master_result = db_conn.query_row(&master_sql, [], |row| {
                Ok(MasterKey {
                    id: row.get(0)?,
                    password: row.get(1)?,
                    salt: row.get(2)?,
                })
            });
            match (master_result) {
                Ok(_) => {
                    return Ok("");
                }
                Err(error) => {
                    println!("{}", KeyEmptyOrMissing.text());
                    user_create_master_password(0, MAX_TRIES);    
                }
            }
        },
        Err(_) => {
            vault::database::vault::create_vault(VAULT_DB_NAME);
            user_create_master_password(0, MAX_TRIES);
        },
    }
    
    Ok("")
}