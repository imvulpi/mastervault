from enum import Enum;
from text_operations.config import rust_reserved_keywords

class GeneralError(Exception, Enum):
    ENUM_VARIATION_NAME_CONTAINS_SPACE = "The provided enum variation name contains spaces",
    ENUM_VARIATION_NAME_CONTAINS_RESERVED_KEYWORDS = "The provided enum variations contain a reserved keyword {}, which can't be used as variation name"

def check_enum_variation_name(enum_name: str):
    if enum_name.count(" "):
        raise GeneralError(GeneralError.ENUM_VARIATION_NAME_CONTAINS_SPACE)
    
    if rust_reserved_keywords.count(enum_name):
        raise GeneralError(GeneralError.ENUM_VARIATION_NAME_CONTAINS_RESERVED_KEYWORDS)
    
    return True

def check_string_content(content: str):
    print()