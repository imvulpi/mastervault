use crate::commons::fields::{traits::validatable::Validatable, structs::{username::UsernameData, email::EmailData, hash_length::HashLengthData, memory::MemoryData, parallelism::ParallelismData, algorithm::AlgorithmData, user_number::UserNumber}};

pub fn get_config_template() -> Vec<(&'static str, Box<dyn Validatable>)>{
    let mut configuration: Vec<(&str, Box<dyn Validatable>)> = Vec::new();
    add_config::<UsernameData>(&mut configuration,"username","username".to_string());
    add_config::<EmailData>(&mut configuration,"email","email".to_string());
    add_config::<HashLengthData>(&mut configuration,"hash_length","32".to_string());
    add_config::<MemoryData>(&mut configuration,"memory","32768".to_string());
    add_config::<ParallelismData>(&mut configuration,"parallelism","1".to_string());
    add_config::<AlgorithmData>(&mut configuration,"algorithm","argon2id".to_string());
    add_config::<UserNumber>(&mut configuration, "iterations", "6".to_string());

    configuration
}

fn add_config<'a, T: Validatable + 'static>(config: &mut Vec<(&'a str, Box<dyn Validatable>)>, setting: &'a str, option: String){
    config.push((setting, Box::new(T::new(option))));
}

pub fn get_required_settings() -> [&'static str; 2] {
    let required_values: [&str; 2] = ["username","email"];
    required_values
}