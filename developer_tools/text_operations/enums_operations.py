def remove_enum(enum_variant_name, path):
    with open(path, "r+") as file:
        start_looking = False
        variant_id = -2 # -1 for the enum initialization line and -2 because it goes from 0..
        lines = file.readlines()

        for index, line in enumerate(lines):
            if line.count("enum") and "/" not in line:
                start_looking = True
        
            if start_looking:
                variant_id += 1
                if line.count(enum_variant_name):
                    print(variant_id)
                    lines[index] = ""
                    file.seek(0)
                    file.truncate()
                    file.writelines(lines)
                    return variant_id
                elif line.count("}"):
                    return None

def new_enum(enum_variant_name, path):
    with open(path, "r+") as file:
        start_looking = False
        lines = file.readlines()

        for index, line in enumerate(lines):
            if line.count("enum") and "/" not in line:
                start_looking = True
            
            if start_looking:
                if line.count("}"):
                    if lines[index-1] == "":
                        lines[index-1] = enum_variant_name + ",\n"
                    else:
                        lines[index-1] = lines[index-1]+"    "+enum_variant_name+",\n" 
            
                    file.seek(0)
                    file.truncate()
                    file.writelines(lines)      
                    return True
                elif line.count(enum_variant_name):
                    return None      