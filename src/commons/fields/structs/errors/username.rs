use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::UsernameTooShort};
#[derive(Debug)]
pub enum UsernameValidationErrors{
    TooShort,
}

impl ValidationError for UsernameValidationErrors{
    fn description(&self) -> String {
        match &self {
            UsernameValidationErrors::TooShort => UsernameTooShort.text(),
        }
    }
}