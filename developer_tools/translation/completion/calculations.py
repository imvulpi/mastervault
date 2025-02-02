from translation.commons.directories_operations import get_subdirectories, pop_non_language_directories
from translation.commons.translation_data_operations.get_file_names import get_column_names, get_table_names
from translation.commons.translation_data_operations.lines import find_translated_lines, get_column_lines
from translation.config import TRANSLATIONS_ADDRESS
import yaml

def recalculate(path):
    """
    Recalculates completions in translation_data.yaml
    """
    tables = get_table_names(path)

    with open(path, "r+") as file:
        data = yaml.safe_load(file)
        all_lines = 0
        all_completed_lines = 0
        for table in tables:
            table_lines = 0
            table_completed_lines = 0
            columns = get_column_names(path, table)
            for column in columns:
                lines = get_column_lines(path, table, column)
                completed_lines = find_translated_lines(path, table, column)
                table_completed_lines += len(completed_lines)
                table_lines += len(lines)
                data['tables'][table]['columns'][column]['completion'] = int(round((len(completed_lines)/len(lines))*100)) # Updates column completion
            all_lines += table_lines
            all_completed_lines += table_completed_lines
            data['tables'][table]['completion'] = int(round((table_completed_lines/table_lines)*100))
        data['header']['overall_completion'] = int(round((all_completed_lines/all_lines)*100))

        file.seek(0)
        file.truncate()
        yaml.dump(data, file, sort_keys=False)