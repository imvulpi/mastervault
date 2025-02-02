import os
from sqlite3 import connect
def get_content(path, table_name, column_name, id):
    if not os.path.exists(path):
        print("Path not found! (path: "+str(path)+")")
        return
        
    connection = connect(path)
    get_content_query = "SELECT {} FROM {} WHERE id == {}".format(column_name, table_name, id)
    return connection.execute(get_content_query).fetchone()[0]