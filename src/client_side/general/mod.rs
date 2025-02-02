use std::fs::File;
use crate::{commons::{file_ops::controller::FileChecker, fields::enums::database::db_strings::DatabaseStrings::{ConfigEmptyOrMissing, CreatingVaultFile, SuccessfulMarker, ErrorOccurred, ErrorMessageMarker, KeyEmptyOrMissing}}, client_side::{self, key::user_create_master_password}, constants::general::values::MAX_TRIES};

pub fn check_and_fix_required_files() {
    if FileChecker::is_empty("config.txt") {
        println!("{}:", ConfigEmptyOrMissing.text());
        client_side::config::user_create_config_file();
    }

    if !FileChecker::file_exists("vault.json"){
        println!("{}", CreatingVaultFile.text());
        match File::create("vault.json") {
            Ok(_) => {
                println!("{}!", SuccessfulMarker.text());
            },
            Err(error) => {
                println!("{} {} {}", ErrorOccurred.text(), ErrorMessageMarker.text(), error);
            }
        }
    }

    if FileChecker::is_empty("key.txt"){
            
        println!("{}", KeyEmptyOrMissing.text());
        user_create_master_password(0, MAX_TRIES);
    }
}