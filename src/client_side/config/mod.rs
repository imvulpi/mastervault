use crate::{commons::{fields::{traits::validatable::Validatable, structs::config::Config}, cli::user_input::get_user_input_persistent, file_ops::write::create_file_key_value}, constants::{config::defaults::{get_config_template, get_required_settings}, defaults::negative_responses, general::{strings::{ENTER_MARKER, OR_MARKER, USE_DEFAULT, ENTER_YOUR_MARKER, REQUIRED_MARKER, ERROR_OCCURRED, TRY_AGAIN, ERROR_MESSAGE_MARKER, CREATED_CONFIG, SETTING_MARKER, OPTION_MARKER}, placeholders::NOT_IMPLEMENTED}}, vault::config::conversions::string_vector_from_config};
pub mod repairment;


pub fn handle_config_subcommand(arguments: Vec<String>){
    println!("{}", NOT_IMPLEMENTED);
}

pub fn user_create_config_file(){
    let configuration = user_get_config_data();

    println!("\n{}\n", CREATED_CONFIG);
    display_config(&configuration.config);

    let string_vector = string_vector_from_config(configuration);
    match create_file_key_value(string_vector, "config.txt"){
        Ok(_) => {},
        Err(e) =>{
            println!("{} - {}\n{}: {}", ERROR_OCCURRED, TRY_AGAIN, ERROR_MESSAGE_MARKER, e);
        }
    }
    println!();
}

fn user_get_config_data() -> Config{
    let required_values = get_required_settings();
    let mut configuration: Vec<(&str, Box<dyn Validatable>)> = get_config_template();
    for (setting, ref mut option) in configuration.iter_mut(){
        let mut is_validated = false;
        while !is_validated {
            if !required_values.contains(&setting){
                println!("\n{}: {} {} {} {}", ENTER_MARKER, setting, OR_MARKER, USE_DEFAULT, option.get_raw_value());
            }else{
                println!("\n{}: {} - [{}]", ENTER_YOUR_MARKER, setting, REQUIRED_MARKER);
            }

            let original_option = option.get_raw_value().to_owned();
            let response: String = get_user_input_persistent();
            if (!negative_responses().contains(&response.trim()) && !required_values.contains(&setting)) || (required_values.contains(&setting)){
                option.put_value(response.trim().to_string());
                let (result, err) = option.validate();
                match result{
                    true => {
                        is_validated = true;
                    }
                    false => {
                        println!("{} - {}\n{}: {}", ERROR_OCCURRED, TRY_AGAIN, ERROR_MESSAGE_MARKER, err.unwrap().description());
                        option.put_value(original_option);
                    }
                }
            }else{
                is_validated = true;
                continue;
            }
        }
    }
    Config { config: configuration }
}

fn display_config(config: &Vec<(&str, Box<dyn Validatable>)>){
    for (setting, option) in config {
        println!("{} - {} | {} - {}", SETTING_MARKER, setting, OPTION_MARKER, option.get_raw_value());
    }
}