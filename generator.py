#!/usr/bin/env python3

from os import listdir
from os.path import isfile, isdir, join
from subprocess import call
import sys

need_rebuild = False

def update_workspace():
    try:
        call(['bash', '-c', 'cd gir && cargo build --release'])
    except:
        return False
    return True


def def_check_submodule(submodule_path):
    if len(listdir(submodule_path)) != 0:
        return False
    print('=> Initializing gir submodule...')
    call(['bash', '-c', 'git submodule update --init'])
    print('<= Done!')

    question = 'Do you want to update gir submodule? [y/N] '
    if sys.version_info[0] < 3:
        line = raw_input(question)
    else:
        line = input(question)
    line = line.strip()
    if line.lower() == 'y':
        print('=> Updating gir submodule...')
        call(['bash', '-c', 'cd gir && git reset --hard HEAD && git pull -f origin master'])
        print('<= Done!')
        return True
    return False


def build_gir_if_needed(updated_submodule):
    if updated_submodule is True or not isfile('./gir/target/release/gir'):
        print('=> Building gir...')
        if update_workspace() is True:
            print('<= Done!')
        else:
            print('<= Failed...')
            return False
    return True


def regen_crates(path, level=1):
    for entry in [f for f in listdir(path)]:
        entry_file = join(path, entry)
        if isdir(entry_file):
            if level == 1 and not regen_crates(entry_file, 2):
                return False
        elif entry == 'Gir.toml':
            print('==> Regenerating "{}"...'.format(entry_file))

            args = ['./gir/target/release/gir', '-c', entry_file, '-o', path, '-d', 'gir-files']
            if level > 1:
                args.append('-m')
                args.append('sys')
            try:
                call(args)
            except Exception as err:
                print('The following error occurred: {}'.format(err))
                line = input('Do you want to continue? [y/N] ').strip().lower()
                if line != 'y':
                    return False
            print('<== Done!')


def main():
    def_check_submodule("gir-files")
    if not build_gir_if_needed(def_check_submodule("gir")):
        return 1

    print('=> Regenerating crates...')
    if not regen_crates("."):
        return 1
    call(['cargo', 'fmt'])
    print('<= Done!')
    print("Don't forget to check if everything has been correctly generated!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
