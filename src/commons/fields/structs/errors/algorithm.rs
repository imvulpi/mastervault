use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::AlgorithmNotFound};
#[derive(Debug)]
pub enum AlgorithmValidationErrors{
    NotFound,
}

impl ValidationError for AlgorithmValidationErrors{
    fn description(&self) -> String {
        match &self {
            AlgorithmValidationErrors::NotFound => AlgorithmNotFound.text(),
        }
    }
}