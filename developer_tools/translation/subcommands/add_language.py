import os
from translation.commons.directories_operations import get_subdirectories, is_language_unique
from translation.commons.files_operations import get_subfiles
from translation.commons.translation_data_operations.create import create_translation_data
from translation.config import ADD_LANGUAGE_REQUIRED_ARGUMENTS, TEMPLATE_FILE_NAME, TRANSLATION_SUBCOMMAND, TRANSLATIONS_ADDRESS, created_system_objects
import shutil

from translation.subcommands.help import translation_help

def add_language(arguments: list[str]):
    if len(arguments) < ADD_LANGUAGE_REQUIRED_ARGUMENTS:
        print("Missing arguments")
        return translation_help()
    
    language = arguments.pop(0) 
    languages = get_subdirectories(TRANSLATIONS_ADDRESS)
    template_path = TRANSLATIONS_ADDRESS+TEMPLATE_FILE_NAME
    language_path = TRANSLATIONS_ADDRESS+language

    if is_language_unique(language, languages) == False:
        if not language.count("_"):
            print("Please follow the pattern -> (3 letter country code)+_+Language in native. Ex. pol_polski (NOT pol_polish!)")        
        return

    if not os.path.exists(language_path):
        os.makedirs(language_path)
        created_system_objects.append(language_path)

    template_files = get_subfiles(template_path)
    print(len(template_files))
    for file in template_files:
        print("file: ", file)
        shutil.copy(template_path+"/"+file, language_path+"/")
    
    create_translation_data(language_path, language)
    print("To begin translating use command: " + TRANSLATION_SUBCOMMAND)