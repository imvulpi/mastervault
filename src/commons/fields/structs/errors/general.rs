use std::io::Error;

use crate::commons::fields::{traits::validation_error::ValidationError, enums::database::db_error_messages::DatabaseErrors::{GeneralIsInvalid, GeneralOsError, GeneralParsingFailed, GeneralSettingNotFound, GeneralU32ConversionFailed, GeneralValidationFailed}};

pub enum GeneralValidationErrors {
    ParsingFailed,
    SettingNotFound,
    U32ConversionFailed,
    ValidationFailed,
    IsInvalid,
    OsError(Error),
}

impl ValidationError for GeneralValidationErrors{
    fn description(&self) -> String {
        match self {
            GeneralValidationErrors::ParsingFailed => GeneralParsingFailed.text(),
            GeneralValidationErrors::SettingNotFound => GeneralSettingNotFound.text(),
            GeneralValidationErrors::U32ConversionFailed => GeneralU32ConversionFailed.text(),
            GeneralValidationErrors::ValidationFailed => GeneralValidationFailed.text(),
            GeneralValidationErrors::IsInvalid => GeneralIsInvalid.text(),
            GeneralValidationErrors::OsError(_) => GeneralOsError.text(),
        }
    }
}