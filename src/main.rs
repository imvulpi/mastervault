mod vault;
mod constants;
mod commons;
mod client_side;
pub mod tests;

fn main() {
    client_side::general::check_and_fix_required_files();
    let _string_array = commons::cli::user_input::get_string_arguments();
}
