use crate::{commons::fields::{traits::validatable::Validatable, structs::{username::UsernameData, email::EmailData, hash_length::HashLengthData, memory::MemoryData, parallelism::ParallelismData, algorithm::AlgorithmData, user_number::UserNumber}, enums::database::db_settings::DatabaseSettings}, constants::general::defaults::{SETTING_USERNAME, SETTING_EMAIL, SETTING_HASH_LENGTH, SETTING_MEMORY, SETTING_PARALLELISM, SETTING_ALGORITHM, SETTING_ITERATIONS}};

pub fn get_config_template() -> Vec<(&'static str, Box<dyn Validatable>)>{
    let mut configuration: Vec<(&str, Box<dyn Validatable>)> = Vec::new();
    add_config::<UsernameData>(&mut configuration, SETTING_USERNAME,"username".to_string());
    add_config::<EmailData>(&mut configuration, SETTING_EMAIL,"email".to_string());
    add_config::<HashLengthData>(&mut configuration, SETTING_HASH_LENGTH,"32".to_string());
    add_config::<MemoryData>(&mut configuration, SETTING_MEMORY,"32768".to_string());
    add_config::<ParallelismData>(&mut configuration, SETTING_PARALLELISM,"1".to_string());
    add_config::<AlgorithmData>(&mut configuration, SETTING_ALGORITHM,"argon2id".to_string());
    add_config::<UserNumber>(&mut configuration, SETTING_ITERATIONS, "6".to_string());

    configuration
}

fn add_config<'a, T: Validatable + 'static>(config: &mut Vec<(&'a str, Box<dyn Validatable>)>, setting: &'a str, option: String){
    config.push((setting, Box::new(T::new(option))));
}

pub fn get_required_settings() -> [&'static str; 2] {
    let required_values: [&str; 2] = ["username","email"];
    required_values
}

pub fn translate_setting(setting: &str) -> String{
    match setting {
        SETTING_USERNAME => {DatabaseSettings::Username.text()}
        SETTING_EMAIL => {DatabaseSettings::Email.text()}
        SETTING_HASH_LENGTH => {DatabaseSettings::HashLength.text()}
        SETTING_MEMORY => {DatabaseSettings::Memory.text()}
        SETTING_PARALLELISM => {DatabaseSettings::Parallelism.text()}
        SETTING_ALGORITHM => {DatabaseSettings::Algorithm.text()}
        SETTING_ITERATIONS => {DatabaseSettings::Iterations.text()}
        _ => {
            DatabaseSettings::NotFound.text()
        }
    }
}
