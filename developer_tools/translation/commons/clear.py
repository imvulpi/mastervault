import os

def clear():
    if os.name == "nt":
        os.system("cls")
    else:
        try:
            os.system("clear")
        except:
            print("Screen clearing not supported on this operating system.")