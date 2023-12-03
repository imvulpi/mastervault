use crate::{commons::fields::traits::validation_error, constants::general::strings::MEMORY_TOO_SMALL};
#[derive(Debug)]
pub enum MemoryValidationErrors{
    TooSmall,
}

impl validation_error::ValidationError for MemoryValidationErrors{
    fn description(&self) -> &'static str {
        match &self {
            MemoryValidationErrors::TooSmall => MEMORY_TOO_SMALL,
        }
    }
}