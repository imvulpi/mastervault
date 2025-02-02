from enum import Enum;

class MappingError(Exception, Enum):
    APPROPRIATE_COLUMN_ERROR = "Could not get the appropriate column and table from this file name."
    ENUM_FILE_NAME_ERROR = "Could not get the appropriate file name from the provided string."

def get_appropriate_column(enum_file_name):
    """Returns a table and columng from a file name.\n
    ex. passing "db_strings.rs" returns:  ("string_data", "string_value") WHERE: string_data IS table AND string_value IS column"""
    match enum_file_name:
        case "db_strings" | "db_strings.rs" | "string" | "strings":
            return ("string_data", "string_value")
        case "db_settings" | "db_settings.rs" | "setting" | "settings":
            return ("other_translations", "setting")
        case "db_error_messages" | "db_error_messages.rs" | "db_errors" | "errors" | "error":
            return ("other_translations", "error")
        case other:
            raise MappingError(MappingError.APPROPRIATE_COLUMN_ERROR)
            
def get_enum_file_name(enum_file_name):
    """Returns an exact file name from similiar names.\n
    ex. passing strings returns db_strings.rs which is the exact file name"""
    match enum_file_name:
        case "db_strings" | "db_strings.rs" | "string" | "strings":
            return "db_strings.rs"
        case "db_settings" | "db_settings.rs" | "setting" | "settings":
            return "db_settings.rs"
        case "db_error_messages" | "db_error_messages.rs" | "db_errors" | "errors" | "error":
            return "db_error_messages.rs"
        case other:
            raise MappingError(MappingError.ENUM_FILE_NAME_ERROR)

