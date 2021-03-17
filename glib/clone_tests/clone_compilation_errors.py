import json
import os
import subprocess
import sys


TEST_FILENAME = "tmp_py_file"
TEST_FOLDER = "clone_tests"
# Some explanations here: `TESTS` is an array containing tuples of two elements.
# * The first element is the code to be compiled.
# * The second element is the expected error message. If the code is suppose to work, then we use
#   `None` instead of a string.
TESTS = [
    ("clone!(@strong v => @default-return None::<i32>, move || {println!(\"foo\"); 1});", None),
    ("clone!( => move || {})",
        "If you have nothing to clone, no need to use this macro!"),
    ("clone!(|| {})",
        "If you have nothing to clone, no need to use this macro!"),
    ("clone!(|a, b| {})",
        "If you have nothing to clone, no need to use this macro!"),
    ("clone!(@weak a, @weak b => |x| {})",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a, @weak b => || {})",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a, @weak b => |x| println!(\"a\"))",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a, @weak b => || println!(\"a\"))",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => |x| {})",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => || {})",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => |x| println!(\"a\"))",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak a => || println!(\"a\"))",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@strong self => move |x| {})",
        "Can't use `self` as variable name. Try storing it in a temporary variable or rename it using `as`."),
    ("clone!(@strong self.v => move |x| {})",
        "Field accesses are not allowed as is, you must rename it!"),
    ("clone!(@weak v => @default-return false, || {})",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => @default-return false, || println!(\"a\"))",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => @default-return false, |bla| {})",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => @default-return false, |bla| println!(\"a\"))",
        "Closure needs to be \"moved\" so please add `move` before closure"),
    ("clone!(@weak v => default-return false, move || {})",
        "Missing `@` before `default-return`"),
    ("clone!(@weak v => @default-return false move || {})",
        "Expected `,` after `@default-return false`"),
    ("clone!(@yolo v => move || {})",
        "Unknown keyword `yolo`, only `weak`, `weak-allow-none` and `strong` are allowed"),
    ("clone!(v => move || {})",
        "Unexpected ident `v`: you need to specify if this is a weak or a strong clone."),
    ("clone!(@strong v => {println!(\"foo\");});",
        "Missing `move` and closure declaration"),
    ("clone!(@strong v, @default-return lol => move || {println!(\"foo\");});",
        "`@default-return` should be after `=>`"),
    ("clone!(@default-return lol, @strong v => move || {println!(\"foo\");});",
        "`@default-return` should be after `=>`"),
    # The async part!
    ("clone!(@strong v => async || {println!(\"foo\");});",
        "Expected `move` after `async`, found `|`"),
    ("clone!(@strong v => async {println!(\"foo\");});",
        "Expected `move` after `async`, found `{`"),
    ("clone!(@strong v => move || async {println!(\"foo\");});",
        "Expected `move` after `async`, found `{`"),
    ("clone!(@strong v => move || async println!(\"foo\"););",
        "Expected `move` after `async`, found `println`"),
    ("clone!(@strong v => move || async move println!(\"foo\"););",
        "Expected block after `| async move`"),
]


def convert_to_string(s):
    if s.__class__.__name__ == 'bytes':
        return s.decode('utf-8')
    return s


def exec_command(command, folder=None):
    child = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=folder)
    stdout, stderr = child.communicate()
    return (child.returncode == 0, convert_to_string(stdout), convert_to_string(stderr))


def run_test(code, expected_str):
    with open("{}/{}.rs".format(TEST_FOLDER, TEST_FILENAME), 'w') as f:
        f.write('use glib::clone;use std::rc::Rc;fn main(){{let v = Rc::new(1);{};}}'.format(code))
    code, stdout, stderr = exec_command(["cargo", "build", "--message-format", "json"], TEST_FOLDER)
    os.remove("{}/{}.rs".format(TEST_FOLDER, TEST_FILENAME))
    if expected_str is None:
        if code is True:
            return None
        return "This was supposed to compile!"
    if code is True:
        return "This isn't supposed to compile!"
    parts = stdout.split('}\n{')
    compiler_message = None
    for (pos, part) in enumerate(parts):
        try:
            if pos > 0:
                part = "{" + part
            if pos + 1 < len(parts):
                part += "}"
            x = json.loads(part)
            if (x["reason"] != "compiler-message"
                or x["message"]["message"] == "aborting due to previous error"):
                continue
            compiler_message = x["message"]
            break
        except Exception:
            continue
    if compiler_message is None:
        return "Weird issue: no compiler-message found..."
    if expected_str == "":
        return "failed: `{}`".format(compiler_message)
    err_message = []
    if expected_str in compiler_message["message"]:
        return None
    err_message.append(compiler_message["message"])
    for child in compiler_message["children"]:
        if "message" not in child:
            continue
        if expected_str in child["message"]:
            return None
        err_message.append(child["message"])
    return "`{}` not found in `{}`".format(expected_str, err_message)


def run_tests():
    print("About to start the tests on the clone! macro.")
    print("It might be slow to run the first one since cargo has to build dependencies...")
    print("")
    errors = 0
    with open('{}/Cargo.toml'.format(TEST_FOLDER), 'w') as f:
        f.write("""[package]
name = "test"
version = "0.0.1"
authors = ["gtk-rs developers"]
edition = "2018"

[dependencies]
glib = {{ path = ".." }}

[[bin]]
name = "{0}"
path = "{0}.rs"

[workspace]
""".format(TEST_FILENAME))
    for (code, expected_str) in TESTS:
        sys.stdout.write('Running `{}`...'.format(code))
        sys.stdout.flush()
        err = run_test(code, expected_str)
        if err is not None:
            print(" FAILED\n{}".format(err))
            errors += 1
        else:
            print(" OK")
    print("Ran {} tests, got {} failure{}".format(len(TESTS), errors, "s" if errors > 1 else ""))
    os.remove("{}/Cargo.toml".format(TEST_FOLDER))
    return errors


if __name__ == "__main__":
    sys.exit(run_tests())
