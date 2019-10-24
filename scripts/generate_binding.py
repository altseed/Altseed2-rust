import os, sys

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

from bindings import define, common
from bindings.CppBindingGenerator import BindingGeneratorRust

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
bindingGenerator.module = ''
bindingGenerator.structsReplaceMap = {
    # common.Vector2DI, "crate::Vec2<i32>"
}
bindingGenerator.generate()

print('generated binding')
print('lang: ' + lang)
print('output_path: ' + os.path.abspath(bindingGenerator.output_path))
print('dll_name: ' + bindingGenerator.dll_name)
print('module: ' + bindingGenerator.module)