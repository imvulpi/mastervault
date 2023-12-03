//pub const MASTERVAULT_INTRO: &str = "MasterVault - Your trusted password manager.";
pub const ERROR_OCCURRED: &str = "Oops, an error occurred";
pub const USE_DEFAULT: &str = "send 'n' to use default";
pub const TRY_AGAIN: &str = "try again";

pub const ERROR_MESSAGE_MARKER: &str = "Error message";
pub const SUCCESSFUL_MARKER: &str = "Success";
pub const ENTER_MARKER: &str = "Enter";
pub const ENTER_YOUR_MARKER: &str = "Enter your";
pub const OR_MARKER: &str = "or";
pub const REQUIRED_MARKER: &str = "required";
pub const SETTING_MARKER: &str = "setting";
pub const OPTION_MARKER: &str = "option";
pub const MINIMUM_MARKER: &str = "minimum";

pub const CONFIG_EMPTY_OR_MISSING: &str = "Config file is either empty or missing";
pub const KEY_EMPTY_OR_MISSING: &str = "Key file is either empty or missing";
pub const CREATING_VAULT_FILE: &str = "Creating vault file";
pub const CREATED_CONFIG: &str = "Created config";
pub const ENTER_MASTER_PASSWORD: &str = "Please enter a secure master password";

// PasswordHashErrors
pub const PASSWORD_TOO_SHORT: &str = "Too short";
pub const PASSWORD_NOT_ENOUGH_SPECIAL: &str = "Not enough special characters";
pub const PASSWORD_NOT_ENOUGH_LOWERCASE: &str = "Not enough lowercase characters";
pub const PASSWORD_NOT_ENOUGH_UPPERCASE: &str = "Not enough uppercase characters";
pub const PASSWORD_OTHER: &str = "Other error";
pub const PASSWORD_NOT_STRONG: &str = "The password isn't strong enough";
pub const MASTER_PASSWORD_REMINDER: &str = "[IMPORTANT] Remember your MASTER PASSWORD!\n\nPlease READ information below!:\n\n1. This master password is used to access passwords stored in the vault.\n2. Keep the master password in a secure place. DON'T store it unsecurely (bad storage example: txt file)\n3. There is no recovery option. It's crucial to memorize or securely store it.";

// Struct implementing ValidationError:
pub const ALGORITHM_NOT_FOUND: &str = "NotFound: The algorithm was not found, existing algorithms: [argon2, argon2i, argon2id] (Recommended: argon2id).";

pub const EMAIL_DOMAIN_ELEMENT_MISSING: &str = "DomainElementMissing: TLD (Top Level Domain) or DOMAIN is missing. (NO DOT)";
pub const EMAIL_AT_SIGN_MISSING: &str = "AtSignMissing: The @ is missing";
pub const EMAIL_CONSECUTIVE_DOTS: &str = "ConsecutiveDots: The email contains consecutive dots (..), which are not allowed.";
pub const EMAIL_AT_SIGN_NEXT_TO_DOT: &str = "AtSignNextToDot: The email contains @ that has dot next to it (@. or .@)";
pub const EMAIL_LAST_OR_FIRST_IS_DOT: &str = "LastOrFirstIsDot: The email begins or ends with a dot (.email or email.)";
pub const EMAIL_MULTIPLE_AT_SIGNS: &str = "MultipleAtSigns: The email contains multiple @ signs, which is not allowed.";
pub const EMAIL_NOT_ALLOWED_CHARACTERS: &str = "NotAllowedCharacters: Email contains characters that are not allowed in the local part. Allowed characters: [a-z],[A-Z],[1-9],[.],[-],[_]";
pub const EMAIL_CONSECUTIVE_SPECIAL_CHARACTERS: &str = "ConsecutiveCharacters: Special characters are consecutive.";

pub const GENERAL_PARSING_FAILED: &str = "ParsingFailed: Could not get the value from string.";
pub const GENERAL_SETTING_NOT_FOUND: &str = "SettingNotFound: Setting not found in config settings.";
pub const GENERAL_U32_CONVERSION_FAILED: &str = "U32ConversionFailed: Conversion to number (u32) failed. Please enter a number without any other characters.";
pub const GENERAL_VALIDATION_FAILED: &str = "ValidationFailed: The validation on this option failed";
pub const GENERAL_IS_INVALID: &str = "IsInvalid: some entryies are invalid";
pub const GENERAL_OS_ERROR: &str = "OsError: Error relating operating system occurred.";

pub const HASH_LENGTH_TOO_SMALL: &str = "TooSmall: the minimum size of hash_length is 4. (Recommended: 32)";
pub const MEMORY_TOO_SMALL: &str = "TooSmall: the minimum size of memory_in_kib is 16384. (Recommended: 32768)";
pub const PARALLELISM_TOO_SMALL: &str = "TooSmall: the minimum size of parallelism is 1. (Recommended: 1)";
pub const USERNAME_TOO_SHORT: &str = "TooShort: The username is too short. The length needs to be at least 3 characters long";