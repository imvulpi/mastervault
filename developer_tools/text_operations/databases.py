from sqlite3 import connect

def database_remove(path, id: int, table, column):
    connection = connect(path)
    set_null_query = f"UPDATE {table} SET {column} = NULL WHERE id={id};"
    get_query = f"SELECT {column} FROM {table} WHERE id={id}"
    get_row_maxid = f"SELECT MAX(ID) FROM {table}"

    connection.execute(set_null_query)
    max_id = connection.execute(get_row_maxid).fetchone()[0]

    for i in range(max_id+1):
        value = connection.execute(get_query).fetchone()
        if value is None:
            print("Value is none!")
    connection.commit()
    connection.close()

def database_add(path, string_content: str, table, column):
    connection = connect(path)
    get_id_query = f"SELECT id, {column} FROM {table} WHERE {column} NOT NULL AND {column} != \"\" ORDER BY id DESC LIMIT 1" # gets the biggest id in the column
    id = connection.execute(get_id_query).fetchone()[0] # executes, but only to get one id (we need one biggest id)

    table_max_id_query = f"SELECT MAX(ID) FROM {table}"

    max_id = connection.execute(table_max_id_query).fetchone()[0]
    new_row_query = f"INSERT INTO {table} (id, {column}) VALUES (?, ?)" # query to add a new entry with
    update_query = f"UPDATE {table} SET {column} = \"{string_content}\" WHERE id = {id+1}"
    if max_id > id+1:
        connection.execute(update_query)
    else:
        connection.execute(new_row_query, (id+1, string_content))
    # Commited and closed
    connection.commit()
    connection.close()