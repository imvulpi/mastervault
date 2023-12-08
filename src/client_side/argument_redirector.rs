use crate::constants::general::defaults::{SUBCOMMAND_ADD, SUBCOMMAND_CHANGE_MASTER, SUBCOMMAND_HELP, SUBCOMMAND_GET, SUBCOMMAND_UPDATE, SUBCOMMAND_DELETE, SUBCOMMAND_LIST, SUBCOMMAND_SEARCH, SUBCOMMAND_CONFIG};

use super::{help::handle_help_subcommand, add::handle_add_subcommand, config::handle_config_subcommand, get::handle_get_subcommand, update::handle_update_subcommand, delete::handle_delete_subcommand, list::handle_list_subcommand, search::handle_search_subcommand, change_master::handle_change_master_subcommand};

pub fn identify_function_from_arguments(arguments_array: &mut Vec<String>){
    let argument = arguments_array.remove(1);
    println!("argument: {}", argument);
    match argument.as_str(){
        SUBCOMMAND_HELP => {handle_help_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_ADD => {handle_add_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_CONFIG => {handle_config_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_GET => {handle_get_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_UPDATE => {handle_update_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_DELETE => {handle_delete_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_LIST => {handle_list_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_SEARCH => {handle_search_subcommand(arguments_array.to_vec())}
        SUBCOMMAND_CHANGE_MASTER => {handle_change_master_subcommand(arguments_array.to_vec())}
        _ => {

        }
    }
}

