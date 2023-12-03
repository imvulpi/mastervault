use std::io::Error;

use crate::{commons::fields::traits::validation_error::ValidationError, constants::general::strings::{GENERAL_PARSING_FAILED, GENERAL_SETTING_NOT_FOUND, GENERAL_U32_CONVERSION_FAILED, GENERAL_VALIDATION_FAILED, GENERAL_IS_INVALID, GENERAL_OS_ERROR}};

pub enum GeneralValidationErrors {
    ParsingFailed,
    SettingNotFound,
    U32ConversionFailed,
    ValidationFailed,
    IsInvalid,
    OsError(Error),
}

impl ValidationError for GeneralValidationErrors{
    fn description(&self) -> &'static str {
        match self {
            GeneralValidationErrors::ParsingFailed => GENERAL_PARSING_FAILED,
            GeneralValidationErrors::SettingNotFound => GENERAL_SETTING_NOT_FOUND,
            GeneralValidationErrors::U32ConversionFailed => GENERAL_U32_CONVERSION_FAILED,
            GeneralValidationErrors::ValidationFailed => GENERAL_VALIDATION_FAILED,
            GeneralValidationErrors::IsInvalid => GENERAL_IS_INVALID,
            GeneralValidationErrors::OsError(_) => GENERAL_OS_ERROR,
        }
    }
}