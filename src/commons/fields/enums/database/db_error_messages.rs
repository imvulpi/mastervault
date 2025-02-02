use crate::vault::database::get;

/// The ids of `DatabaseErrors` must be kept the same as the ids in the database.
/// 
/// These are the errors stored in a database. 
/// 
/// Example: `PasswordNotEnoughSpecial` will have an id of 1
/// 
/// Unless you know what you are doing:
/// DO NOT change these texts manually! (use text_operations.py in developer tools)
pub enum DatabaseErrors {
    PasswordTooShort,
    PasswordNotEnoughSpecial,
    PasswordNotEnoughLowercase,
    PasswordNotEnoughUppercase,
    PasswordOther,
    PasswordNotStrong,
    AlgorithmNotFound,
    EmailDomainElementMissing,
    EmailAtSignMissing,
    EmailConsecutiveDots,
    EmailAtSignNextToDot,
    EmailLastOrFirstIsDot,
    EmailMultipleAtSigns,
    EmailNotAllowedCharacters,
    EmailConsecutiveSpecialCharacters,
    GeneralParsingFailed,
    GeneralSettingNotFound,
    GeneralU32ConversionFailed,
    GeneralValidationFailed,
    GeneralIsInvalid,
    GeneralOsError,
    HashLengthTooSmall,
    MemoryTooSmall,
    ParallelismTooSmall,
    UsernameTooShort,
}

impl Into<u32> for DatabaseErrors{
    fn into(self) -> u32 {
        self as u32
    }
}

impl DatabaseErrors{
    pub fn text(self) -> String {
        match get(self.into(), "error", "other_translations"){
            Ok(data) => {
                return data;
            },
            Err(error) => {
                return format!("Database Access Error\nDetailed: {}", error);
            },
        }
    }
}