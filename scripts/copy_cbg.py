import os, shutil, ntpath, glob

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

shutil.copytree('../Core/scripts/bindings', './bindings')
