import os
from sqlite3 import connect

def get_table_names(path):
    if not os.path.exists(path):
        return

    connection = connect(path)
    table_names_query = "SELECT name FROM sqlite_master WHERE type = 'table';" # gets all table names from database
    table_names = connection.execute(table_names_query).fetchall()
    return table_names


def get_column_names(path, table):
    if not os.path.exists(path):
        return
    
    column_names_query = "SELECT name FROM PRAGMA_TABLE_INFO('{}');".format(table)
    connection = connect(path)
    column_names = connection.execute(column_names_query).fetchall()
    return column_names
