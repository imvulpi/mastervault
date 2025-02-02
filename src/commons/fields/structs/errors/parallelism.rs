use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::ParallelismTooSmall};
#[derive(Debug)]
pub enum ParallelismValidationErrors {
    TooSmall,
}

impl ValidationError for ParallelismValidationErrors{
    fn description(&self) -> String {
        match &self {
            ParallelismValidationErrors::TooSmall => ParallelismTooSmall.text(),
        }
    }
}