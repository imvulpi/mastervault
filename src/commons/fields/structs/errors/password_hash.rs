use std::fmt::Display;

use crate::constants::general::{values::{MIN_PASSWORD_LENGTH, MIN_PASSWORD_LOWERCASE, MIN_PASSWORD_UPPERCASE, MIN_PASSWORD_SPECIAL}, strings::{PASSWORD_TOO_SHORT, MINIMUM_MARKER, PASSWORD_NOT_ENOUGH_SPECIAL, PASSWORD_NOT_ENOUGH_UPPERCASE, PASSWORD_NOT_ENOUGH_LOWERCASE, PASSWORD_OTHER}};

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
                f.write_fmt(format_args!("{}. {}: {}", PASSWORD_TOO_SHORT, MINIMUM_MARKER, MIN_PASSWORD_LENGTH))
            },
            PasswordHashError::NotEnoughSpecialCharacters => {
                f.write_fmt(format_args!("{}. {}: {}", PASSWORD_NOT_ENOUGH_SPECIAL, MINIMUM_MARKER, MIN_PASSWORD_SPECIAL))
            },
            PasswordHashError::NotEnoughUpperCase => {
                f.write_fmt(format_args!("{}. {}: {}", PASSWORD_NOT_ENOUGH_UPPERCASE, MINIMUM_MARKER, MIN_PASSWORD_UPPERCASE))
            },
            PasswordHashError::NotEnoughLowerCase => {
                f.write_fmt(format_args!("{}. {}: {}", PASSWORD_NOT_ENOUGH_LOWERCASE, MINIMUM_MARKER, MIN_PASSWORD_LOWERCASE))
            },
            PasswordHashError::Other => {
                f.write_fmt(format_args!("{}.", PASSWORD_OTHER))
            },
        }
    }
}

