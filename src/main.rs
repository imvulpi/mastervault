use std::{env::{self, ArgsOs}, ffi::OsString,};
fn main() {
    let command_arguments = env::args_os();
    let string_array = get_string_arguments(command_arguments);
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