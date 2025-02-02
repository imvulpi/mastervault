import os
import sys
from text_operations.config import DATABASE_ADDRESS, DB_NAME, ENUMS_ADDRESS, TEMPLATES_ADDRESS, CUSTOM_NEW_REQUIRED_ARGUMENTS_AMOUNT, NEW_REQUIRED_ARGUMENTS_AMOUNT
from text_operations.databases import database_add
from text_operations.enums_operations import new_enum, remove_enum
from text_operations.error_handling.general_error_handling import check_enum_variation_name
from text_operations.string_mapper import MappingError, get_appropriate_column, get_enum_file_name
from text_operations.subcommands.help import custom_help
from sqlite3 import OperationalError    

def new(arguments: list[str]):
    """
    Handles the 'new' subcommand:\n
    new [EnumFile] [EnumVariantName] [String-Content]
    """

    # Gets all names, addresses and other needed 
    enum_variant_name, enum_file_address, string_content, table, column = handle_new_arguments(arguments)
    
    created = new_enum(enum_variant_name, enum_file_address) 
    if created is True:
        try:
            database_add(DATABASE_ADDRESS + DB_NAME, string_content, table, column)
        except Exception as e:
            print("Error: ",e,"\n Deleting created enum...")
            remove_enum(enum_variant_name, enum_file_address)
            sys.exit()
    else:
        print("\nEnum Variant already exists")
        sys.exit()
    print(enum_variant_name, " Created", " at ", enum_file_address, "\nContent: ", string_content)

def handle_new_arguments(arguments: list[str]):
    """
    Returns variables from subcommand 'new' arguments list, checks for errors within these arguments. 
    """
    if len(arguments) < NEW_REQUIRED_ARGUMENTS_AMOUNT:
        print("Unsufficient arguments amount\n")
        custom_help("new")
        sys.exit()
    elif len(arguments) > NEW_REQUIRED_ARGUMENTS_AMOUNT:
        arguments_amount = len(arguments)
        print("Went here")


    enum_variant_name = arguments[1]
    string_content = arguments[2]

    try:
        enum_file = get_enum_file_name(arguments[0])
        (table, column) = get_appropriate_column(enum_file)
    except MappingError as e:
        print(f"Enum file name: {arguments[0]}")
        print(e.value)
        sys.exit()
    
    enum_file_address = ENUMS_ADDRESS + enum_file

    check_enum_variation_name(enum_variant_name)
    return enum_variant_name, enum_file_address, string_content, table, column


def new_custom(arguments):
    """
    Handles the 'new-custom' subcommand:\n
    new-custom [EnumFile] [EnumVariantName] [String-Content] [DB-Table] [DB-Column]
    """

    # Checks whether sufficient amount of arguments were given for this subcommand
    if len(arguments) < CUSTOM_NEW_REQUIRED_ARGUMENTS_AMOUNT:
        return custom_help()
    
    # Gets all names, addresses and other needed 
    enum_file_address = arguments[0]
    enum_variant_name = arguments[1]
    string_content = arguments[2]
    table = arguments[3]
    column = arguments[4]
    check_enum_variation_name(enum_variant_name)

    # Checks whether full path was provided
    if not os.path.exists(enum_file_address):
        # Checks whether full path exists
        if os.path.exists(ENUMS_ADDRESS+enum_file_address):
            enum_file_address = ENUMS_ADDRESS+enum_file_address
        else:
            raise "File doesn't exist! Create it in "+ENUMS_ADDRESS+" OR provide a path to the file."
            

    new_enum(enum_variant_name, enum_file_address)
    try:
        database_add(DATABASE_ADDRESS + DB_NAME, string_content, table, column)
    except Exception as e:
        print("Error: ",e,"\n Deleting created enum...")
        remove_enum(enum_variant_name, enum_file_address)

    print(enum_variant_name, " - Created", " at ", enum_file_address, "\nContent: ", string_content)
