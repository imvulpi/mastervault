use crate::{commons::fields::traits::validation_error, constants::general::strings::PARALLELISM_TOO_SMALL};
#[derive(Debug)]
pub enum ParallelismValidationErrors {
    TooSmall,
}

impl validation_error::ValidationError for ParallelismValidationErrors{
    fn description(&self) -> &'static str {
        match &self {
            ParallelismValidationErrors::TooSmall => PARALLELISM_TOO_SMALL,
        }
    }
}