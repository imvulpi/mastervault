import os
import sys

from text_operations.config import DATABASE_ADDRESS, DB_NAME, ENUMS_ADDRESS, REMOVE_REQUIRED_ARGUMENTS_AMOUNT, CUSTOM_REMOVE_REQUIRED_ARGUMENTS_AMOUNT, TEMPLATES_ADDRESS
from text_operations.databases import database_remove
from text_operations.enums_operations import remove_enum
from text_operations.string_mapper import get_appropriate_column, get_enum_file_name

def remove(arguments):
    # Checks whether sufficient amount of arguments were given for this subcommand
    if len(arguments) < REMOVE_REQUIRED_ARGUMENTS_AMOUNT:
        return help()
    
    enum_file = get_enum_file_name(arguments[0])
    enum_file_address = ENUMS_ADDRESS + enum_file
    enum_variant_name = arguments[1]
    (table, column) = get_appropriate_column(enum_file)

    id = remove_enum(enum_variant_name, enum_file_address)
    if id is not None:
        database_remove(DATABASE_ADDRESS + DB_NAME, id, table, column)
    else:
        print("There is no Enum Variant that you provided.")
        sys.exit()

    print(enum_variant_name, " - Removed", " from ", enum_file_address)

def remove_custom(arguments):
    # Checks whether sufficient amount of arguments were given for this subcommand
    if len(arguments) < CUSTOM_REMOVE_REQUIRED_ARGUMENTS_AMOUNT:
        return help() 

    enum_file_address = arguments[0]
    enum_variant_name = arguments[1]
    table = arguments[2]
    column = arguments[3]

    # Checks whether full path was provided
    if not os.path.exists(enum_file_address):
        # Checks whether full path exists
        if os.path.exists(ENUMS_ADDRESS+enum_file_address):
            enum_file_address = ENUMS_ADDRESS+enum_file_address
        else:
            print("File doesn't exist! Create it in " + ENUMS_ADDRESS + " OR provide a path to the file.")
            sys.exit()

    id = remove_enum(enum_variant_name, enum_file_address)
    if id is not None:
        database_remove(DATABASE_ADDRESS + DB_NAME, id, table, column)
    else:
        print("There is no Enum Variant that you provided.")
        sys.exit()

    print(enum_variant_name, " - Removed", " from ", enum_file_address)
