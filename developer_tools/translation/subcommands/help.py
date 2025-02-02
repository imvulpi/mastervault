def translation_help():
    print("\nCommands:\n\n")
    space = "    "
    subcommands = [
        "translate [language] [file]\n{0}desc. Use this subcommand to translate text from a file.\n{0}ex. translate pol other".format(space),
        "add_lang [language]\n{0}desc. Adds a language translation, Creates required directories and files\n{0}ex. add_language prt_portugues".format(space),
        "list_langs\n{0}desc. List all added languages and lists their completion percentage".format(space),
        "validate [language]\n{0}desc. Validates if criteria are met.".format(space),
        "check [language] [optional: file]\n{0}desc. Checks completion of language and lines in a file".format(space),
    ]
    for subcommand in subcommands:
        print(subcommand + "\n")