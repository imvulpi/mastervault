use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::{EmailAtSignMissing, EmailAtSignNextToDot, EmailConsecutiveDots, EmailConsecutiveSpecialCharacters, EmailDomainElementMissing, EmailLastOrFirstIsDot, EmailMultipleAtSigns, EmailNotAllowedCharacters}};
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

impl ValidationError for EmailValidationErrors{
    fn description(&self) -> String {
        match &self {
            EmailValidationErrors::DomainElementMissing => EmailDomainElementMissing.text(),
            EmailValidationErrors::AtSignMissing => EmailAtSignMissing.text(),
            EmailValidationErrors::ConsecutiveDots => EmailConsecutiveDots.text(),
            EmailValidationErrors::AtSignNextToDot => EmailAtSignNextToDot.text(),
            EmailValidationErrors::LastOrFirstIsDot => EmailLastOrFirstIsDot.text(),
            EmailValidationErrors::MultipleAtSigns => EmailMultipleAtSigns.text(),
            EmailValidationErrors::NotAllowedCharacter => EmailNotAllowedCharacters.text(),
            EmailValidationErrors::ConsecutiveCharacters => EmailConsecutiveSpecialCharacters.text(),
        }
    }
}