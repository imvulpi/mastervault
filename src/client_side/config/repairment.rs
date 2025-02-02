use crate::{vault::config::{conversions::string_vector_from_file, repair::{remove_invalid_tuples, get_missing_tuples}, get_value_from_option}, commons::{fields::{structs::{config::Config, errors::general::GeneralValidationErrors}, traits::validation_error::ValidationError, enums::database::db_strings::DatabaseStrings::{EnterMarker, OrMarker, UseDefault, ErrorOccurred, ErrorMessageMarker}}, cli::user_input::get_user_input_persistent, file_ops::write::create_file_key_value}, constants::{config::defaults::get_config_template, general:: placeholders::UNKNOWN, defaults::negative_responses}};

pub fn fix_config(){
    match string_vector_from_file("config.txt", "=") {
        Ok(mut data) => {
            remove_invalid_tuples(&mut data);            
            let missing = get_missing_tuples(&mut data);

            let default_config = Config {config: get_config_template() };
            let mut reconstructed_tuples: Vec<(String, String)> = Vec::new();

            for element in missing{
                let default_option = match get_value_from_option::<String>(&default_config, element){
                    Ok(value) => {
                        value
                    },
                    Err(_error) => {
                        UNKNOWN.to_string()
                    }
                };
                println!("\n{}: {} {} {} {}", EnterMarker.text(), element, OrMarker.text(), UseDefault.text(), default_option);
                let input = get_user_input_persistent().trim().to_string();
                if !negative_responses().contains(&input.to_lowercase().trim()){
                    reconstructed_tuples.push((element.to_string(), input));
                }else{
                    reconstructed_tuples.push((element.to_string(), default_option));
                }
            }

            data.append(&mut reconstructed_tuples);

            match create_file_key_value(data, "config.txt") {
                Ok(_) => {},
                Err(error) => {
                    println!("{}\n{}: {}", ErrorOccurred.text(), ErrorMessageMarker.text(), error);
                }
            }
        }
        Err(error) => {
            println!("{}\n{}: {}", ErrorOccurred.text(), ErrorMessageMarker.text(), error);
            fix_config();
        }
    }
     
}

pub fn error_handler(error: &GeneralValidationErrors){
    match error{
        GeneralValidationErrors::OsError(err) => {
            println!("{} - {}\n{}: {}", ErrorOccurred.text(), error.description(), ErrorMessageMarker.text(), err.to_string());
        },
        _ => {
            fix_config();
        },
    }
}