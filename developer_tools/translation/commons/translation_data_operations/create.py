import os
from translation.commons.files_operations import get_subfiles
from translation.config import TRANSLATION_DATA_FILE_NAME
from sqlite3 import connect
import yaml

def create_translation_data(language_directory_path: str, language):
    language_path = language_directory_path
    if not language_directory_path.count(TRANSLATION_DATA_FILE_NAME):
        last_char = language_directory_path[len(language_directory_path)-1]
        if last_char != "/" and last_char != "\\":
            language_directory_path+="/" 
        language_directory_path+=TRANSLATION_DATA_FILE_NAME
    
    data = {
        'header': {
            'overall_completion': 0,
            'language': language
        },

        'tables': {

        }
    }    
    create_files_data_part(data, language_path)
    with open(language_directory_path, "w") as file:
        yaml.dump(data, file, sort_keys=False)

def create_files_data_part(data: dict[str, any], path):
    files = get_subfiles(path)
    for file in files:
        if not file.count(".db"):
            continue
        connection = connect(os.path.join(path, file))
        table_names_query = "SELECT name FROM sqlite_master WHERE type = 'table';" # gets all table names from database
        column_names_query = "SELECT name FROM PRAGMA_TABLE_INFO('{}');"
        lines_amount_query = "SELECT {0}, id FROM {1} WHERE {0} NOT NULL ORDER BY id DESC LIMIT 1" # gets the amount of not null entries, 1 - column, 2 - table
        table_names = connection.execute(table_names_query).fetchall()
        
        for table_name in table_names:
            table_name = table_name[0]
            column_names = connection.execute(column_names_query.format(table_name)).fetchall()
            data['tables'][table_name] = {
                "completion": 0,
                "columns": {

                }
            }
            for column_name in column_names:
                column_name = column_name[0]
                max_id = connection.execute(lines_amount_query.format(column_name, table_name)).fetchone()[1]
                if not column_name == "id":
                    print(column_name)
                    zero_array = create_zero_list(max_id+1)

                    data['tables'][table_name]['columns'][column_name] = {
                            "completion": 0,
                            "lines": zero_array,
                    }
            
def create_zero_list(amount):
    zeros_list = []
    for i in range(amount):
        zeros_list.append(0)
    return zeros_list