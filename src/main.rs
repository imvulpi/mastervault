mod config;
pub mod tests;
use std::{env::{self, ArgsOs}, ffi::OsString, fs::File, io::{self},};
use config::FileChecker;

fn main() {
    check_and_fix_required_files();
    
    let command_arguments = env::args_os();
    let string_array = get_string_arguments(command_arguments);
}

fn ask_user_for_string_data(message: &str) -> String{
    println!("{}", message);
    let mut data = String::new();
    match io::stdin().read_line(&mut data){
        Ok(_) => {},
        Err(err) => {
            print!("Error: {}", err);
            return ask_user_for_string_data(message);
        }
    }
    return data;
}

fn check_and_fix_required_files() {
    if FileChecker::is_empty("config.txt") {
        println!("Config file appears to be missing, or is empty.\n--- IF this is your first time launching this app - DON'T WORRY! ---");
        let username = ask_user_for_string_data("Please enter your username.");
        let email = ask_user_for_string_data("Please enter your email.");
        match config::create_config_file(vec![username, email]){
            Ok(_) => {},
            Err(e) =>{
                println!("An error occured while creating a config file, please try again.\nError message: {}", e);
                check_and_fix_required_files();
            }
        }
    }

    if !FileChecker::file_exists("vault.json"){
        println!("Creating vault file.");
        match File::create("vault.json") {
            Ok(_) => {},
            Err(e) => {
                println!("An error occured while creating the vault file. Trying again... \nError message: {}", e);
                check_and_fix_required_files();
            }
        }
    }

    if FileChecker::is_empty("key.txt"){
        println!("Key file appears to be missing, or is empty.");
    }
}

fn get_string_arguments(args_os: ArgsOs) -> Vec<String>{
    let args_os_array: Vec<_> = args_os.collect();
    let mut string_arguments_array: Vec<String> = Vec::new();
    for argument in args_os_array{
        match argument.into_string() {
            Ok(converted_string) => {
                string_arguments_array.push(converted_string.to_owned());
            },
            Err(o) => {
                string_arguments_array.push(safe_string_conversion(o.to_owned()).unwrap());
            },
        };
    }

    return string_arguments_array;
}

fn safe_string_conversion(os_string: OsString) -> Result<String, std::convert::Infallible>{
    return os_string.to_string_lossy().parse::<String>();
}