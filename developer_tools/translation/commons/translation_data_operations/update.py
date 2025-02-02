import os
import yaml

def update_line_completion(path, table_name, column, line, value):
    if not os.path.exists(path):
        print("Path does not exist!")
        return
    
    if value != 1 and value != 0:
        print("Value is not in correct form")

    with open(path, "r+") as file:
        data = yaml.safe_load(file)
        try:
            data['tables'][table_name]['columns'][column]['lines'][line] = value
        except KeyError as e:
            print(f"Error: Missing key in data structure - {e}")
        
        file.seek(0)
        file.truncate()
        yaml.dump(data, file, sort_keys=False)