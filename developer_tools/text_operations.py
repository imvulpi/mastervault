import sys;
from text_operations.subcommands.new import *
from text_operations.subcommands.remove import *
from text_operations.subcommands.help import *

def main():
    arguments = sys.argv
    if len(arguments) > 1:
        arguments.pop(0)
        subcommand = arguments.pop(0) 
        match subcommand:
            case "new":
                new(arguments)
            case "remove":
                remove(arguments)
            case "new-custom":
                new_custom(arguments)
            case "remove-custom":
                remove_custom(arguments)
            case other:
                custom_help()
    else:
        custom_help()
if __name__ == "__main__":
    main()
