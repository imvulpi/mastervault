use crate::{commons::fields::enums::database::db_strings::DatabaseStrings, constants::general::placeholders::NOT_IMPLEMENTED};

pub fn handle_help_subcommand(arguments: Vec<String>){
    println!("{}", DatabaseStrings::text(DatabaseStrings::HelpCommandText));
}