use crate::{commons::fields::traits::validation_error, constants::general::strings::HASH_LENGTH_TOO_SMALL};
#[derive(Debug)]
pub enum HashLengthValidationErrors{
    TooSmall,
}

impl validation_error::ValidationError for HashLengthValidationErrors{
    fn description(&self) -> &'static str {
        match &self {
            HashLengthValidationErrors::TooSmall => HASH_LENGTH_TOO_SMALL,
        }
    }
}