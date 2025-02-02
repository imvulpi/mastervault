use std::fmt::Display;
use crate::constants::general::values::{MIN_PASSWORD_LENGTH, MIN_PASSWORD_LOWERCASE, MIN_PASSWORD_UPPERCASE, MIN_PASSWORD_SPECIAL};
use crate::commons::fields::enums::database::{db_error_messages::DatabaseErrors::{PasswordTooShort, PasswordNotEnoughSpecial, PasswordNotEnoughUppercase, PasswordNotEnoughLowercase, PasswordOther}, db_strings::DatabaseStrings::MinimumMarker};

#[derive(Debug)]
pub enum PasswordHashError{
    TooShort,
    NotEnoughSpecialCharacters,
    NotEnoughUpperCase,
    NotEnoughLowerCase,
    Other,
}

impl Display for PasswordHashError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            PasswordHashError::TooShort => {
                f.write_fmt(format_args!("{}. {}: {}", PasswordTooShort.text(), MinimumMarker.text(), MIN_PASSWORD_LENGTH))
            },
            PasswordHashError::NotEnoughSpecialCharacters => {
                f.write_fmt(format_args!("{}. {}: {}", PasswordNotEnoughSpecial.text(), MinimumMarker.text(), MIN_PASSWORD_SPECIAL))
            },
            PasswordHashError::NotEnoughUpperCase => {
                f.write_fmt(format_args!("{}. {}: {}", PasswordNotEnoughUppercase.text(), MinimumMarker.text(), MIN_PASSWORD_UPPERCASE))
            },
            PasswordHashError::NotEnoughLowerCase => {
                f.write_fmt(format_args!("{}. {}: {}", PasswordNotEnoughLowercase.text(), MinimumMarker.text(), MIN_PASSWORD_LOWERCASE))
            },
            PasswordHashError::Other => {
                f.write_fmt(format_args!("{}.", PasswordOther.text()))
            },
        }
    }
}

