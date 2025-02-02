ENUMS_ADDRESS = "../src/commons/fields/enums/database/"
DATABASE_ADDRESS = "../src/"
DB_NAME = "constant_data.db"
ENUM_FILE_NAMES = ["db_strings.rs", "db_settings.rs", "db_error_messages.rs"]
TEMPLATES_ADDRESS = "../translations/template/"

NEW_REQUIRED_ARGUMENTS_AMOUNT = 3
CUSTOM_NEW_REQUIRED_ARGUMENTS_AMOUNT = 5
REMOVE_REQUIRED_ARGUMENTS_AMOUNT = 2
CUSTOM_REMOVE_REQUIRED_ARGUMENTS_AMOUNT = 4


# Errors:
rust_reserved_keywords = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "async", "await", "try", "dyn"
]