import os

from translation.config import TRANSLATION_DATA_FILE_NAME

def get_subdirectories(path: str):
    subdirectories = []
    entries = os.listdir(path)
    for entry in entries:
        if os.path.isdir(os.path.join(path, entry)):
            subdirectories.append(entry)
    return subdirectories

def pop_non_language_directories(list: list, path: str):
    for index, directory in enumerate(list):
        if not os.path.isfile(os.path.join(path, str(directory),"/",TRANSLATION_DATA_FILE_NAME)):
            list.pop(index)
    return list

def get_correct_language_name(language: str, languages: list):
    for entry in languages:
        string_entry = str(entry)
        language_parts = language.split("_")
        for part in language_parts:
            if string_entry.count(part):
                return str(entry)
            
def is_language_unique(language: str, languages: list[str]):
    for entry in languages:
        string_entry = str(entry)
        language_parts = language.split("_")
        for part in language_parts:
            if string_entry.count(part):
                if entry != language:
                    print("similiar language already exists ["+part+" - "+entry+"]") 
                    answer = input("Do you want to continue? Y/N : ")
                    if answer.lower() == "y":
                        return True
                    return False
                print("The language already exists: " + entry)
                return False
    return True