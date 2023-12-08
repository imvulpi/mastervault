use argon2::{Params, Algorithm};
use crate::{commons::{self, cli::user_input::get_user_input_persistent, fields::{traits::validation_error::ValidationError, structs::errors::general::GeneralValidationErrors}}, vault::{config::{get_config_from_file, get_algorithm_from_config, get_params_from_config}, key::{generate_hash_password, complexity::check_password_complexity}}, constants::{general::strings::{ERROR_OCCURRED, ERROR_MESSAGE_MARKER, TRY_AGAIN, ENTER_MASTER_PASSWORD, SUCCESSFUL_MARKER, PASSWORD_NOT_STRONG, MASTER_PASSWORD_REMINDER}, defaults::negative_responses}, client_side::config::repairment::error_handler};

pub fn user_create_master_password(current_try: u8, max_tries: u8){
    let password = handle_getting_password();
    match get_config_from_file("config.txt"){    
        Ok(config) =>{
            let algorithm = match get_algorithm_from_config(&config){
                Ok(algorithm) => {
                    algorithm
                },
                Err(error) => {
                    error_handler(&error);
                    handle_error_and_retry(error, current_try, max_tries);
                    return;
                },
            };
            let params = match get_params_from_config(config) {
                Ok(params) => {params}
                Err(error) => {
                    error_handler(&error);
                    handle_error_and_retry(error, current_try, max_tries);
                    return;
                }
            };
            handle_password_hashing(password, params, algorithm);
        }

        Err(error) => {
            println!("{}\n{}: {}",ERROR_OCCURRED, ERROR_MESSAGE_MARKER, error.description());
            error_handler(&error)
        }
    }  
}

fn handle_getting_password() -> String{
    let mut is_good_password = false;
    let mut password = String::new();
    while !is_good_password{
        println!("{}", ENTER_MASTER_PASSWORD);
        password = get_user_input_persistent();
        match check_password_complexity(&password){
            Ok(_) => {
                is_good_password = true;
            },
            Err(error) => {
                println!("{}: {}", PASSWORD_NOT_STRONG, error)
            },
        }
    }
    return password;
}

fn handle_error_and_retry(error: GeneralValidationErrors, current_try: u8, max_tries: u8) {
    error_handler(&error);
    if current_try < max_tries {
        user_create_master_password(current_try, max_tries);
    }else{
        if !negative_responses().contains(&get_user_input_persistent().trim()){
            user_create_master_password(0, max_tries)
        }
    }
}

fn handle_password_hashing(input: String, params: Params, algorithm: Algorithm){
    match generate_hash_password(input, params, algorithm) {
        Ok(data) => {handle_succesful_password_hashing(data)}
        Err(_) => {handle_unsuccesful_password_hashing(GeneralValidationErrors::IsInvalid)}
    }
}

fn handle_succesful_password_hashing(data: Vec<(String, String)>){
    match commons::file_ops::write::create_file_key_value(data, "key.txt"){
        Ok(_) => {
            println!("{}!", SUCCESSFUL_MARKER);
            println!("\n{}", MASTER_PASSWORD_REMINDER);
        },
        Err(err) => {
            println!("{} - {}\n{}: {}", ERROR_OCCURRED, TRY_AGAIN, ERROR_MESSAGE_MARKER, err);
            error_handler(&GeneralValidationErrors::OsError(err));
        }
    }
}

fn handle_unsuccesful_password_hashing(error: GeneralValidationErrors){
    println!("{}\n{}: {}", ERROR_OCCURRED, ERROR_MESSAGE_MARKER, error.description());
    error_handler(&GeneralValidationErrors::IsInvalid);
}