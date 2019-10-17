import os, shutil, ntpath, glob

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

outDir = './binding'
cbgDir = './cbg'

def createIfNotFound(dir):
    if not os.path.exists(dir):
        os.mkdir(dir)

createIfNotFound(outDir)
createIfNotFound(cbgDir)

def copyFile(path, dir):
    shutil.copy(path, '{}/{}'.format(dir, ntpath.split(path)[1]))

copyFile('../Core/scripts/wrapper/definitions.py', outDir)

for path in glob.glob('../CBG/cbg/*.py'):
    copyFile(path, './cbg')
