import os, sys

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

from bindings import define, math
from bindings.CppBindingGenerator import BindingGeneratorRust

args = sys.argv
lang = 'ja'
if len(args) >= 3 and args[1] == '-lang':
    if args[2] in ['ja', 'en']:
        lang = args[2]
    else:
        print('python rust.py -lang [ja|en]')

bindingGenerator = BindingGeneratorRust(define, lang)
bindingGenerator.output_path = '../src/auto_generated_core_binding.rs'
bindingGenerator.dll_name = 'Altseed_Core'
bindingGenerator.module = ''
bindingGenerator.structModName = 'crate::structs'
bindingGenerator.structsReplaceMap = {
    math.Vector2I: "crate::math::Vector2<i32>",
    math.Vector2F: "crate::math::Vector2<f32>",
    math.Vector3F: "crate::math::Vector3<f32>",
    math.Vector3I: "crate::math::Vector3<i32>",
    math.Vector4F: "crate::math::Vector4<f32>",
    math.Vector4I: "crate::math::Vector4<i32>",
    math.BatchVertex: "crate::structs::vertex::Vertex",
    math.Color: "crate::structs::color::Color",
    math.RectF: "crate::structs::rect::Rect<f32>",
    math.RectI: "crate::structs::rect::Rect<i32>",
    math.Matrix44F: "crate::math::Matrix44<f32>",
    math.Matrix44I: "crate::math::Matrix44<i32>",
}
bindingGenerator.generate()

print('generated binding')
print('lang: ' + lang)
print('output_path: ' + os.path.abspath(bindingGenerator.output_path))
print('dll_name: ' + bindingGenerator.dll_name)
print('module: ' + bindingGenerator.module)