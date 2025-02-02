from translation.commons.directories_operations import get_subdirectories, pop_non_language_directories
from translation.config import TRANSLATIONS_ADDRESS, TRANSLATION_DATA_FILE_NAME
from translation.completion.get_percentage import get_table_percentage

def list_languages():
    print("\nLanguage | Completion\n")
    subdirectories = get_subdirectories(TRANSLATIONS_ADDRESS)
    pop_non_language_directories(subdirectories, TRANSLATIONS_ADDRESS)
    for directory in subdirectories:
        percentange = get_table_percentage(TRANSLATIONS_ADDRESS+directory+"/"+TRANSLATION_DATA_FILE_NAME)
        print(directory+" - "+str(percentange)+"%")
    print()