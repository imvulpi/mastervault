import os

def get_subfiles(path: str) -> list[str]:
    subfiles = []
    entries = os.listdir(path)
    for entry in entries:
        if os.path.isfile(os.path.join(path, entry)):
            subfiles.append(entry)
    return subfiles 