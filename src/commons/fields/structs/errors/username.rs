use crate::{commons::fields::traits::validation_error, constants::general::strings::USERNAME_TOO_SHORT};
#[derive(Debug)]
pub enum UsernameValidationErrors{
    TooShort,
}

impl validation_error::ValidationError for UsernameValidationErrors{
    fn description(&self) -> &'static str {
        match &self {
            UsernameValidationErrors::TooShort => USERNAME_TOO_SHORT,
        }
    }
}