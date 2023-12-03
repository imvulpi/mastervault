use std::io::Error;

use crate::{commons::{file_ops::read::get_lines_as_string_vec, fields::structs::{config::Config, errors::general::GeneralValidationErrors}}, constants::config::defaults::get_config_template};

pub fn string_vector_from_config(data: Config) -> Vec<(String, String)>{
    let mut string_vector: Vec<(String, String)> = Vec::new();
    for (a, b) in data.config{
        string_vector.push((a.to_string(), b.get_raw_value().to_string()));
    }

    return string_vector;
}

pub fn string_vector_from_file(path: &str, seperator: &str) -> Result<Vec<(String, String)>, Error>{
    match get_lines_as_string_vec(path) {
        Ok(data) =>{
            let mut string_vector: Vec<(String, String)> = Vec::new();
            for line in data{
                let split_lines : Vec<&str> = line.split(seperator).collect();
                if split_lines.len() > 1{
                    string_vector.push((split_lines[0].to_string(), split_lines[1].to_string()));
                }
            }
            return Ok(string_vector);
        }
        Err(err) => {
            return Err(err);
        },
    }
}

pub fn config_from_string_vector(data: &mut Vec<(String, String)>) -> Result<Config, GeneralValidationErrors>{
    let mut config_template = get_config_template();
    for (setting, option) in data{
        let mut is_invalid = true;
        for (default_setting, default_option) in &mut config_template{
            if default_setting.contains(setting.trim()){
                let copied_option = option.clone();
                default_option.put_value(copied_option);
                if default_option.validate().0 == true {
                    is_invalid = false;
                }else{
                    return Err(GeneralValidationErrors::ValidationFailed)
                }
            }
        }
        if is_invalid{
            return Err(GeneralValidationErrors::IsInvalid);
        }
    }

    return Ok(Config { config: get_config_template() });
}