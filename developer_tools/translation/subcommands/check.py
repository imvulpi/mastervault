from translation.commons.directories_operations import get_correct_language_name, get_subdirectories
from translation.commons.translation_data_operations.get_file_names import get_column_names, get_table_names
from translation.commons.translation_data_operations.lines import get_column_lines
from translation.completion.get_percentage import get_column_percentage, get_table_percentage
from translation.config import CHECK_REQUIRED_ARGUMENTS, TRANSLATION_DATA_FILE_NAME, TRANSLATIONS_ADDRESS
from colorama import Fore, Style, init

from translation.subcommands.help import translation_help

def check(arguments: list):
    if len(arguments) < CHECK_REQUIRED_ARGUMENTS:
        return translation_help()
    
    language = str(arguments.pop(0))
    languages = get_subdirectories(TRANSLATIONS_ADDRESS)
    language = get_correct_language_name(language, languages)
    if not language.count("_"):
        print()

    path = TRANSLATIONS_ADDRESS+language+TRANSLATION_DATA_FILE_NAME
    overall_completion = get_table_percentage(path)
    print("\nOverall Completion: "+str(overall_completion)+"%\n")
    if len(arguments) > 0:  
        table_name = arguments.pop(0)
        columns = get_column_names(path, table_name)
        if len(columns) > 0:
            table_progress = get_table_percentage(path, table_name)
            print("Table: "+str(table_name)+"\nTable Completion: "+str(table_progress)+"%"+"\nColumns: \n")
        for column in columns:
            lines = get_column_lines(path, table_name, column)
            data = get_column_percentage(path, table_name, column)
            pretty_printing_check(column, data, lines)
    else:
        names = get_table_names(path)
        
        for name in names:
            columns = get_column_names(path, name)
            if len(columns) > 0:
                table_progress = get_table_percentage(path, name)
                print("Table: "+str(name)+"\nTable Completion: "+str(table_progress)+"%"+"\nColumns: \n")
            for column in columns:
                progress = get_column_percentage(path, name, column)
                lines = get_column_lines(path, name, column)
                pretty_printing_check(column, progress, lines)

def pretty_printing_check(name, completion, lines):
    progress_bar = create_progress_bar(completion)
    print(name+" - "+str(completion)+"% "+ progress_bar+"\n")    
    init()
    for number, line in enumerate(lines):
        if line == 0:
            print(Fore.RED + "Line "+str(number+1)+": "+str(line)+" (Not translated)"+Fore.RESET)  
        else:
            print(Fore.GREEN + "Line "+str(number+1)+": "+str(line)+" (Translated)"+Fore.RESET)
    print()

def create_progress_bar(completion):
    progress = ""
    for i in range(10):
        if completion >= 10:
            progress += "_ "
        else:
            progress += "  "
        completion -= 10
    progress_bar = "[ " + progress + "]"
    return progress_bar