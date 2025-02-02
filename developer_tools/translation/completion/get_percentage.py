import os
import yaml

def get_table_percentage(path: str, table_name=""):
    if not os.path.exists(path):
        return
    with open(path, "r") as file:
        data = yaml.safe_load(file)
        if table_name == "":
            return data['header']['overall_completion']
        else:
            return data['tables'][table_name]['completion']
        
def get_column_percentage(path: str, table_name: str, column_name: str):
    if not os.path.exists(path) or table_name == "" or column_name == "":
        return
    
    with open(path, "r") as file:
        data = yaml.safe_load(file)
        return data['tables'][table_name]['columns'][column_name]['completion']