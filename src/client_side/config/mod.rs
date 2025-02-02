use crate::{commons::{fields::{traits::validatable::Validatable, structs::config::Config, enums::database::db_strings::DatabaseStrings::{CreatedConfig, ErrorOccurred, TryAgain, ErrorMessageMarker, EnterMarker, YourMarker, OrMarker, UseDefault, RequiredMarker, SettingMarker, OptionMarker}}, cli::user_input::get_user_input_persistent, file_ops::write::create_file_key_value}, constants::{config::defaults::{get_config_template, get_required_settings, translate_setting}, defaults::negative_responses, general::placeholders::NOT_IMPLEMENTED}, vault::config::conversions::string_vector_from_config};
pub mod repairment;


pub fn handle_config_subcommand(arguments: Vec<String>){
    println!("{}", NOT_IMPLEMENTED);
}

pub fn user_create_config_file(){
    let configuration = user_get_config_data();

    println!("\n{}\n", CreatedConfig.text());
    display_config(&configuration.config);

    let string_vector = string_vector_from_config(configuration);
    match create_file_key_value(string_vector, "config.txt"){
        Ok(_) => {},
        Err(e) =>{
            println!("{} - {}\n{}: {}", ErrorOccurred.text(), TryAgain.text(), ErrorMessageMarker.text(), e);
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
                println!("\n{} {}: {} {} {} {}", EnterMarker.text(), YourMarker.text(), translate_setting(setting), OrMarker.text(), UseDefault.text(), option.get_raw_value());
            }else{
                println!("\n{} {}: {} - [{}]", EnterMarker.text(), YourMarker.text(), translate_setting(setting), RequiredMarker.text());
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
                        println!("{} - {}\n{}: {}", ErrorOccurred.text(), TryAgain.text(), ErrorMessageMarker.text(), err.unwrap().description());
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
        println!("{} - {} | {} - {}", SettingMarker.text(), setting, OptionMarker.text(), option.get_raw_value());
    }
}