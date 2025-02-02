from sqlite3 import connect
import os

def get_max_id_from_table(path, table):  
    if not os.path.exists(path):
        return

    max_id_query = "SELECT ID FROM {} ORDER BY ID DESC LIMIT 1;".format(table) 
    connection = connect(path)
    return connection.execute(max_id_query).fetchone()[0]

