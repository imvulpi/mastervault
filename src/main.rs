use client_side::{argument_redirector::identify_function_from_arguments, help::handle_help_subcommand};

mod vault;
mod constants;
mod commons;
mod client_side;
pub mod tests;

fn main() {
    client_side::general::check_and_fix_required_files();
    let mut string_array = commons::cli::user_input::get_string_arguments();

    if string_array.len() > 1{
        identify_function_from_arguments(&mut string_array);
    }else{
        handle_help_subcommand(string_array);
    }
}
