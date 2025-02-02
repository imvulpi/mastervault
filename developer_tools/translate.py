import shutil
import sys
import traceback
from translation.subcommands.check import check
from translation.subcommands.help import translation_help
from translation.subcommands.translate import translate
from translation.subcommands.add_language import add_language
from translation.subcommands.list_languages import list_languages
from translation.subcommands.validate import validate
from translation.config import created_system_objects

def main():
    arguments = sys.argv

    if len(arguments) > 1:
        arguments.pop(0)
        subcommand = arguments.pop(0) 
        match (subcommand):
            case "translate":
                translate(arguments)
            case "add_lang":
                try:
                    add_language(arguments)
                except Exception as e:
                    traceback.print_exc()
                    print("\nSomething went wrong, There are objects which were created before program failed\nHere are those objects: \n")
                    list_objects_to_deletion(created_system_objects)
                    print("\nMake sure there are no personal files. Remove them manually or send 'y' to let the program remove it.")
                    option = input()
                    if option.lower() == 'y':
                        delete_objects(created_system_objects)
            case "list_langs" | "list_lang":
                list_languages()
            case "validate":
                validate(arguments)
            case "check":
                check(arguments)
            case other:
                translation_help() 
    else:
        translation_help()

def delete_objects(objects: []):
    for index, object in enumerate(objects):
        shutil.rmtree(object)
        print("removed: " + str(index+1) + " - " + object)

def list_objects_to_deletion(objects: []):
    if len(objects) < 0:
        return
    
    for index, object in enumerate(objects):
        print(str(index+1) + " - " + object)

if __name__ == "__main__":
    main()

