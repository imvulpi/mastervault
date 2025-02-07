use std::{error::Error, fs::File, io::{LineWriter, Write}};
use rusqlite::{params, types::Null};
use crate::{constants::database::{PASSWORDS_CREDENTIALS_ID, PASSWORDS_ID_NAME, PASSWORDS_PASSWORD_NAME, PASSWORDS_SALT_NAME, VAULT_DB_NAME}, vault::{database::vault::create_vault, key::master_key::MasterKey}};
use crate::constants::database::PASSWORDS_TABLE_NAME;

pub fn create_file_key_value(data : Vec<(String, String)>, path: &str) -> Result<(), std::io::Error>{
    match File::options().write(true).create(true).open(path){
        Ok(file) => {
            let mut writer = LineWriter::new(file);
            for (key, data) in data{
                let line = format!("{}={}\n",key, data);
                match writer.write_all(&line.into_bytes()) {
                    Ok(_) => {},
                    Err(e) => {return Err(e);}
                }
            }
        },
        Err(e) => {return Err(e);}
    };
    return Ok(());
}

pub fn create_db_key_value(data : &MasterKey, path: &str) -> Result<(), Box<dyn Error + 'static>>{
    create_vault(VAULT_DB_NAME);
    let sqlconn = rusqlite::Connection::open(VAULT_DB_NAME)?;
    let key_insert_sql = format!("INSERT INTO {} ({}, {}, {}, {}) VALUES(?1, ?2, ?3, ?4)", PASSWORDS_TABLE_NAME, PASSWORDS_ID_NAME, PASSWORDS_PASSWORD_NAME, PASSWORDS_SALT_NAME, PASSWORDS_CREDENTIALS_ID);
    println!("{}", &key_insert_sql);
    sqlconn.execute(&key_insert_sql, params![0, data.password, data.salt, Null]).unwrap(); // ID 0 is the master key
    return Ok(());
}