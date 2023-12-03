use crate::{commons::fields::traits::validation_error, constants::general::strings::ALGORITHM_NOT_FOUND};
#[derive(Debug)]
pub enum AlgorithmValidationErrors{
    NotFound,
}

impl validation_error::ValidationError for AlgorithmValidationErrors{
    fn description(&self) -> &'static str {
        match &self {
            AlgorithmValidationErrors::NotFound => ALGORITHM_NOT_FOUND,
        }
    }
}