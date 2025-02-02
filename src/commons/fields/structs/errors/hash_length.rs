use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::HashLengthTooSmall};
#[derive(Debug)]
pub enum HashLengthValidationErrors{
    TooSmall,
}

impl ValidationError for HashLengthValidationErrors{
    fn description(&self) -> String {
        match &self {
            HashLengthValidationErrors::TooSmall => HashLengthTooSmall.text(),
        }
    }
}