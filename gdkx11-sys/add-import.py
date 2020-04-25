import sys
import os

if len(sys.argv) < 2:
    print("Missing target lib path")
    os._exit(1)

if not os.path.isfile(sys.argv[1]):
    print("`{}`: file not found".format(os.path.realpath(sys.argv[1])))
    os._exit(1)

with open(sys.argv[1]) as f:
    content = f.read()
with open(sys.argv[1], "w") as f:
    f.write(content.replace(
        "\nextern crate pango_sys as pango;\n",
        "\nextern crate pango_sys as pango;\n// manual import\nextern crate x11;\nuse x11::xlib;\n"))
print("Replaced content in `{}`".format(sys.argv[1]))
