use crate::vault::database::get;

/// The ids of `DatabaseStrings` must be kept the same as the ids in the database.
/// 
/// Example: `CreatedConfig` will have an id of 1
///
/// Unless you know what you are doing:
/// DO NOT change these texts manually! (use text_operations.py in developer tools)
pub enum DatabaseStrings {
    ConfigEmptyOrMissing,
    CreatedConfig,
    CreatingVaultFile,
    EnterMasterPassword,
    EnterMarker,
    YourMarker,
    ErrorMessageMarker,
    ErrorOccurred,
    KeyEmptyOrMissing,
    MinimumMarker,
    OptionMarker,
    OrMarker,
    RequiredMarker,
    SettingMarker,
    SuccessfulMarker,
    TryAgain,
    UseDefault,
    MasterPasswordReminder,
    HelpCommandText,
}

impl Into<u32> for DatabaseStrings {
    fn into(self) -> u32 {
        self as u32
    }
}

impl DatabaseStrings{
    /// Gets string_data from `DATABASE_NAME` using enum `Strings` id.
    /// 
    /// # Example:
    /// ```
    /// let a = SuccessfulMarker.text()
    /// assert_eq!(a, "Success") // Whatever is in the id of SuccessfulMarker in the `DATABASE_NAME` 
    /// ```
    pub fn text(self) -> String{
        match get(Into::<u32>::into(self), "string_value", "string_data"){
            Ok(data) => {
                return data;
            },
            Err(error) => {
                return format!("Database Access Error\nDetailed: {}", error);
            },
        }
    }
}