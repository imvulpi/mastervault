use crate::{commons::fields::traits::validation_error, constants::general::strings::{EMAIL_DOMAIN_ELEMENT_MISSING, EMAIL_AT_SIGN_MISSING, EMAIL_CONSECUTIVE_DOTS, EMAIL_AT_SIGN_NEXT_TO_DOT, EMAIL_LAST_OR_FIRST_IS_DOT, EMAIL_MULTIPLE_AT_SIGNS, EMAIL_NOT_ALLOWED_CHARACTERS, EMAIL_CONSECUTIVE_SPECIAL_CHARACTERS}};
#[derive(Debug)]
pub enum EmailValidationErrors{
    DomainElementMissing,
    AtSignMissing,
    ConsecutiveDots,
    AtSignNextToDot,
    LastOrFirstIsDot,
    MultipleAtSigns,
    NotAllowedCharacter,
    ConsecutiveCharacters,
}

impl validation_error::ValidationError for EmailValidationErrors{

    fn description(&self) -> &'static str {
        match &self {
            EmailValidationErrors::DomainElementMissing => EMAIL_DOMAIN_ELEMENT_MISSING,
            EmailValidationErrors::AtSignMissing => EMAIL_AT_SIGN_MISSING,
            EmailValidationErrors::ConsecutiveDots => EMAIL_CONSECUTIVE_DOTS,
            EmailValidationErrors::AtSignNextToDot => EMAIL_AT_SIGN_NEXT_TO_DOT,
            EmailValidationErrors::LastOrFirstIsDot => EMAIL_LAST_OR_FIRST_IS_DOT,
            EmailValidationErrors::MultipleAtSigns => EMAIL_MULTIPLE_AT_SIGNS,
            EmailValidationErrors::NotAllowedCharacter => EMAIL_NOT_ALLOWED_CHARACTERS,
            EmailValidationErrors::ConsecutiveCharacters => EMAIL_CONSECUTIVE_SPECIAL_CHARACTERS,
        }
    }
}