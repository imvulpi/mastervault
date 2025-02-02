import os
import yaml
from translation.commons.clear import clear
from translation.commons.database.get_content import get_content
from translation.commons.database.update import update_entry
from translation.commons.translation_data_operations.lines import find_untranslated_lines
from translation.commons.translation_data_operations.update import update_line_completion
from translation.completion.calculations import recalculate
from translation.config import DATABASE_FILE_NAME, TRANSLATE_REQUIRED_ARGUMENTS, TRANSLATION_DATA_FILE_NAME, TRANSLATIONS_ADDRESS
from translation.subcommands.help import translation_help


def translate(arguments: list): 
    if len(arguments) < TRANSLATE_REQUIRED_ARGUMENTS:
        return translation_help()
    
    language = arguments.pop(0)

    if len(arguments) > 1:
        table = arguments.pop(0)
        if len(arguments) > 1:
            column = arguments.pop(0)
            translate_three_argument(language,table,column)
        translate_two_argument(language, table)
    else:
        translate_one_argument(language)
            
def translate_one_argument(language):
    translation_data_path = os.path.join(TRANSLATIONS_ADDRESS + language + TRANSLATION_DATA_FILE_NAME)
    database_path = os.path.join(TRANSLATIONS_ADDRESS + language +"/"+ DATABASE_FILE_NAME)
    if not os.path.exists(translation_data_path):
        print("Language directory does not exist")

    with open(translation_data_path, "r") as file:
        data = yaml.safe_load(file)
        print(data)
        tables = data['tables']
        for table in tables:
            columns = data['tables'][table]['columns']
            for column in columns:
                indexes = find_untranslated_lines(translation_data_path, table, column)
                for index in indexes:
                    clear()
                    content = get_content(database_path, table, column, index)
                    print("Translate: " + str(content)+"\n")
                    translation = input("Press enter to not change anything\nPlease input the translation: ")
                    if translation != "":
                        update_line_completion(translation_data_path, table, column, index, 1)
                        update_entry(database_path, table, column, index, translation)
                        recalculate(translation_data_path)

def translate_two_argument(language, table):
    print()

def translate_three_argument(language, table, column):
    print()