def custom_help(subcommand=""):
    space = "    "
    subcommands = [
        "new [EnumFile] [EnumVariantName] [String-Content]\n{0}desc. Adds an enum and entry to a database\n{0}ex. new db_strings.rs DontMarker \"Don't do it!\"".format(space),
        "remove [EnumFile] [EnumVariantName]\n{0}desc. deletes an enum and entry from the database\n{0}ex. new db_strings.rs DontMarker".format(space),
        "new-custom [EnumFile] [EnumVariantName] [String-Content] [DB-Table] [DB-Column]\n{0}desc. creates a new EnumVarian in provided file and writes provided string-content in a database column in provided table\n{0}ex. new-custom path.../some_file.rs Variant \"That's a example\" table_name column_name".format(space),
        "remove-custom [EnumFile] [EnumVariantName] [DB-Table] [DB-Column]\n{0}desc. deletes an EnumVariant from provided file, a row in provided database table\n{0}ex. remove-custom db_strings.rs DontMarker string_data\n{0}ex2. remove-custom path.../some_file.rs Variant table_name".format(space),
    ]
    if subcommand == "":
        print("\nCommands:\n\n")
        for subcommand in subcommands:
            print(subcommand+"\n")
    else:
        if subcommand == "new":
            print(subcommands[0])
        if subcommand == "remove":
            print(subcommand[1])