use std::{str::FromStr, ops::Index};
use argon2::{Params, Algorithm};
use crate::commons::fields::structs::{config::Config, errors::general::GeneralValidationErrors};

use self::conversions::{config_from_string_vector, string_vector_from_file};
pub mod conversions;
pub mod repair;

pub fn get_config_from_file(path: &str) -> Result<Config, GeneralValidationErrors> {
    let outcome = string_vector_from_file(path, "=");
    match outcome{
        Ok(mut string_tuples) => {
            match config_from_string_vector(&mut string_tuples){
                Ok(config) => {
                    return Ok(config);
                },
                Err(error) => {
                    return Err(error)
                },
            }
        },
        Err(err) => {
            return Err(GeneralValidationErrors::OsError(err));
        }
    }
}

pub fn get_params_from_config(data: Config) -> Result<Params, GeneralValidationErrors>{
    let mut setting_values = Vec::new();
    let settings = [
        get_value_from_option::<u32>(&data, "memory"),
        get_value_from_option::<u32>(&data, "iterations"),
        get_value_from_option::<u32>(&data, "parallelism"),
        get_value_from_option::<u32>(&data, "hash_length"),
    ];

    for setting in settings{
        match setting{
            Ok(good_setting) => {
                setting_values.push(good_setting);
            },
            Err(error) => {
                return Err(error);
            }
        }
    }

    match Params::new(*setting_values.index(0),*setting_values.index(1), *setting_values.index(2), Some(*setting_values.index(3) as usize)) {
        Ok(params) => {
            return Ok(params);
        },
        Err(_) => {
            return Err(GeneralValidationErrors::ParsingFailed);
        },
    }
}

pub fn get_algorithm_from_config(data: &Config) -> Result<Algorithm, GeneralValidationErrors>{
    match get_value_from_option::<String>(data, "algorithm"){
        Ok(value) =>{
            let outcome = Algorithm::from_str(&value.as_str());
            match outcome{
                Ok(algorithm) => {
                    Ok(algorithm)
                },
                Err(_) => {
                    return Err(GeneralValidationErrors::ValidationFailed);
                }
            }
        }

        Err(error) =>{
            return Err(error);
        }
    }
}

pub fn get_value_from_option<'a, T: 'static>(data: &'a Config, setting: &str) -> Result<T, GeneralValidationErrors> where T: FromStr{
    let option = data.get_option(setting);
    match option {
        Ok(unwrapped) => {
            match unwrapped.get_raw_value().parse::<T>(){
                Ok(a) => {
                    return Ok(a);
                },
                Err(_err) => {
                    return Err(GeneralValidationErrors::ParsingFailed);
                },
            }
            
        }
        Err(_err) => {
            return Err(GeneralValidationErrors::SettingNotFound);
        }
    }
}