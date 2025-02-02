use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::MemoryTooSmall};
#[derive(Debug)]
pub enum MemoryValidationErrors{
    TooSmall,
}

impl ValidationError for MemoryValidationErrors{
    fn description(&self) -> String {
        match &self {
            MemoryValidationErrors::TooSmall => MemoryTooSmall.text(),
        }
    }
}