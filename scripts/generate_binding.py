import os, sys

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

from binding.definitions import define
from cbg import BindingGeneratorRust

args = sys.argv
lang = 'en'
if len(args) >= 3 and args[1] == '-lang':
    if args[2] in ['ja', 'en']:
        lang = args[2]
    else:
        print('python rust.py -lang [ja|en]')

bindingGenerator = BindingGeneratorRust(define, lang)
bindingGenerator.output_path = '../src/binding.rs'
bindingGenerator.dll_name = 'Altseed_Core'
bindingGenerator.namespace = 'Altseed2'
bindingGenerator.structsReplaceMap = {
}
bindingGenerator.generate()