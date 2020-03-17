import os, sys

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

from bindings import define, math, tool, input, core, graphics, io, logger, sound, tool, window, common
from bindings.CppBindingGenerator import BindingGeneratorRust, CacheMode

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
    math.RectF: "crate::structs::rect::Rect<f32>",
    math.RectI: "crate::structs::rect::Rect<i32>",
    math.Matrix44F: "crate::math::Matrix44<f32>",
    math.Matrix44I: "crate::math::Matrix44<i32>",
}
bindingGenerator.bitFlags = {
    tool.ToolTreeNode,
    tool.ToolTreeNode,
    tool.ToolInputText,
    tool.ToolColorEdit,
    tool.ToolSelectable,
    tool.ToolWindow,
    tool.ToolTabBar,
}

# 注意して扱う！
no_cache_classes = [
    core.Configuration,
    core.Core,
    graphics.Graphics,
    graphics.Renderer,
    graphics.CommandList,
    window.Window,
    io.Resources,
    graphics.Rendered,
    graphics.RenderedCamera,
    graphics.RenderedPolygon,
    graphics.RenderedSprite,
    graphics.RenderedText,
]

for class_ in no_cache_classes:
    if class_.cache_mode != CacheMode.ThreadSafeCache:
        class_.cache_mode = CacheMode.NoCache

# staticメソッドは怖いので隠す
def hide_method(class_, name):
    next(filter(lambda x: x.name == name, class_.funcs), None).is_public = False

hide_method(graphics.Texture2D, 'Load')
hide_method(graphics.Font, 'LoadDynamicFont')
hide_method(graphics.Font, 'LoadStaticFont')
hide_method(sound.Sound, 'Load')
hide_method(io.StaticFile, 'Create')
hide_method(io.StreamFile, 'Create')
hide_method(graphics.BuiltinShader, 'Create')

# コメントをRust向けに修正
descs = []
for c in define.classes:
    if c.brief != None:
        descs += [ c.brief.descs ]
    for f in c.funcs:
        if f.brief != None:
            descs += [ f.brief.descs ]
        for a in f.args:
            if a.brief != None:
                descs += [ a.brief.descs ]
        if f.return_value.brief != None:
            descs += [ f.return_value.brief.descs ]
    for p in c.properties:
        if p.brief != None:
            descs += [ p.brief.descs ]
for e in define.enums:
    if e.brief != None:
        descs += [ e.brief.descs ]
        for v in e.values:
            if v.brief != None:
                descs += [ v.brief.descs ]

import re

pattern_ = r'(.*)<see cref="(Altseed.)?(.*)"/>(.*)'

for d in descs:
    for k in d:
        res = re.findall(pattern_, d[k])
        if res:
            res = res[0]
            name = next(iter([v.split('::')[-1] for k,v in bindingGenerator.structsReplaceMap.items() if k.alias == res[2]]), res[2])
            result = res[0] + name + res[3]
            print('{} => {}'.format(d[k], result))
            d[k] = result

bindingGenerator.generate()

print('generated binding')
print('lang: ' + lang)
print('output_path: ' + os.path.abspath(bindingGenerator.output_path))
print('dll_name: ' + bindingGenerator.dll_name)
print('module: ' + bindingGenerator.module)