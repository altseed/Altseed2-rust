import os, sys, shutil, subprocess

from generate_example import generate_example

def generate_docs():
    docs = [
        'engine',
        'node',
        'graphics',
        'keyboard_mouse',
        'joystick',
        'sound',
        'file',
        'log',
    ]

    target_dir = '../src/documents'

    if os.path.exists(target_dir):
        shutil.rmtree(target_dir)
    os.mkdir(target_dir)

    auto_generated_msg = '''// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

'''

    with open('{}/mod.rs'.format(target_dir), 'w') as f:
        f.write(auto_generated_msg)

        index = 0
        for _name in docs:
            name = '_{:0=2}_{}'.format(index, _name)
            f.write('pub mod {};\n'.format(name))
            index += 1

    index = 0
    for _name in docs:
        name = '_{:0=2}_{}'.format(index, _name)
        index += 1

        filename = '{}/{}.rs'.format(target_dir, name)

        content = []
        with open('../docs/contents/{}.md'.format(_name), 'r') as f:
            content = f.readlines()

        with open(filename, 'w') as f:
            f.write(auto_generated_msg)

            for line in content:
                f.write('//! ' + line)


if __name__ == '__main__':
    # move to source directory
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    generate_docs()
    subprocess.run('cargo fmt', shell=True)
    subprocess.run('cargo rustdoc', shell=True)