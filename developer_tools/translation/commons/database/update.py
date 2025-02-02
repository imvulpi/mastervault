import os
from sqlite3 import connect, IntegrityError

def update_entry(path, table_name, column_name, id, value):
    if not os.path.exists(path):
        print("Path to database does not exist: "+str(path))
    
    connection = connect(path)
    update_query = "UPDATE {} SET {} = \"{}\" WHERE id = {}".format(table_name, column_name, value, id)
    
    try:
        connection.execute(update_query)
        connection.commit()
    except IntegrityError as e:
        print(e)