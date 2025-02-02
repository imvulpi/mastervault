import os
import yaml

def get_table_names(path):
    if not os.path.exists(path):
        return
    with open(path, "r") as file:
        data = yaml.safe_load(file)
        return data["tables"]
    
def get_column_names(path, table_name):
    if not os.path.exists(path):
        return
    with open(path, "r") as file:
        data = yaml.safe_load(file)
        return data["tables"][table_name]['columns']