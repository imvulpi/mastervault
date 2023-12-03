use std::fs::File;
use crate::{commons::file_ops::controller::FileChecker, client_side::{self, key::user_create_master_password}, constants::general::{strings::{CONFIG_EMPTY_OR_MISSING, CREATING_VAULT_FILE, ERROR_OCCURRED, ERROR_MESSAGE_MARKER, SUCCESSFUL_MARKER, KEY_EMPTY_OR_MISSING}, values::MAX_TRIES}};

pub fn check_and_fix_required_files() {
    if FileChecker::is_empty("config.txt") {
        println!("{}:", CONFIG_EMPTY_OR_MISSING);
        client_side::config::user_create_config_file();
    }

    if !FileChecker::file_exists("vault.json"){
        println!("{}", CREATING_VAULT_FILE);
        match File::create("vault.json") {
            Ok(_) => {
                println!("{}!", SUCCESSFUL_MARKER);
            },
            Err(error) => {
                println!("{} {} {}", ERROR_OCCURRED, ERROR_MESSAGE_MARKER, error);
            }
        }
    }

    if FileChecker::is_empty("key.txt"){    
        println!("{}", KEY_EMPTY_OR_MISSING);
        user_create_master_password(0, MAX_TRIES);
    }
}