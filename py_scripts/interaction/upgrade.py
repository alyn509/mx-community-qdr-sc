import sys
sys.path.append("..")
from py_scripts import config
from py_scripts.context import Context

def main():
    context = Context()
    context.upgrade()

if __name__ == "__main__":
    main()
