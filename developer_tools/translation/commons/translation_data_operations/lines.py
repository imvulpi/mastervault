import os
import yaml


def get_column_lines(path: str, table_name, column_name):
    if not os.path.exists(path):
        return
    with open(path, "r") as file:
        data = yaml.safe_load(file)
        if table_name == "" or column_name == "":
            return
        else:
            return data['tables'][table_name]['columns'][column_name]['lines']
        
def find_closest_untranslated_line(path: str, table, column):
    """
    Returns an index of closest untranslated line
    """
    if not os.path.exists(path):
        return
    with open(path, "r") as file:
        data = yaml.safe_load(file)
        lines = data["tables"][table]['columns'][column]['lines']
        for index, line in enumerate(lines):
            if line == 0:
                return index
            
def find_untranslated_lines(path: str, table, column):
    """
    Returns all indexes of untranslated lines
    """
    if not os.path.exists(path):
        return
    
    with open(path, "r") as file:
        untranslated_lines = []
        data = yaml.safe_load(file)
        lines = data["tables"][table]['columns'][column]['lines']
        for index, line in enumerate(lines):
            if line == 0:
                untranslated_lines.append(index)
        
        return untranslated_lines
    
def find_translated_lines(path: str, table, column):
    """
    Returns all indexes of translated lines
    """
    if not os.path.exists(path):
        return
    
    with open(path, "r") as file:
        translated_lines = []
        data = yaml.safe_load(file)
        lines = data["tables"][table]['columns'][column]['lines']
        for index, line in enumerate(lines):
            if line == 1:
                translated_lines.append(index)
        
        return translated_lines