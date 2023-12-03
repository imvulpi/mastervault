use std::{env::{self}, ffi::OsString, io};

pub fn get_user_input_persistent() -> String{
    let mut data = String::new();
    match io::stdin().read_line(&mut data){
        Ok(_) => {
            return data;
        },
        Err(_) => {
            return get_user_input_persistent();
        }
    }
}

pub fn get_string_arguments() -> Vec<String>{
    let args_os = env::args_os();
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