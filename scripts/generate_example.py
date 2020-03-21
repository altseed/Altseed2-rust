import os, sys, shutil

# move to source directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

timer_frame = 10

examples = [
    'hello_world',
    'sprite',
    'text',
    'sound',
    'log',
    'load_async',
    'custom_node',
]

target_dir = '../src/examples'

shutil.rmtree(target_dir)
os.mkdir(target_dir)

auto_generated_msg = '''// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。
'''

with open('{}/mod.rs'.format(target_dir), 'w') as f:
    f.write(auto_generated_msg)

    index = 0
    for _name in examples:
        name = '_{:0=2}_{}'.format(index, _name)
        f.write('pub mod {};\n'.format(name))
        index += 1

    index = 0
    for _name in examples:
        name = '_{:0=2}_{}'.format(index, _name)
        f.write('mod test{};\n'.format(name))
        index += 1

index = 0
for _name in examples:
    name = '_{:0=2}_{}'.format(index, _name)
    index += 1

    filename = '{}/{}.rs'.format(target_dir, name)
    filename_test = '{}/test{}.rs'.format(target_dir, name)

    content = []
    with open('../examples/{}.rs'.format(_name), 'r') as f:
        content = f.readlines()

    with open(filename, 'w') as fd:
        with open(filename_test, 'w') as ft:
            fd.write(auto_generated_msg)
            ft.write(auto_generated_msg)

            enabled_timer = False
            added_timer = False

            for line in content:
                if line.startswith('//!'):
                    fd.write(line)

            fd.write('//! ```no_run\n')
            ft.write('//! ```\n')

            for line in content:
                if (not line.startswith('//!')):
                    if not enabled_timer and 'add-timer' in line:
                        ft.write('//! include!("../tests/timer.rs");\n')
                        enabled_timer = True
                        continue

                    fd.write('//! ' + line)
                    ft.write('//! ' + line)

                    if enabled_timer and (not added_timer) and 'Engine::initialize' in line:
                        ft.write('//!     engine.add_node(timer::TimerNode::new({}))?;\n'.format(timer_frame))
                        added_timer = True
            
            fd.write('''//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example {}
//! ```
'''.format(_name))


