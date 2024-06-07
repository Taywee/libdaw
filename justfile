version := ```
python -c '
import tomllib
with open("Cargo.toml", "rb") as cargo:
    data = tomllib.load(cargo)
print(data["workspace"]["package"]["version"])
'
```

_default:
    @just --list
venv:
    #!/bin/sh
    set -euf
    if [ ! -e .venv ]; then
        set -x
        python -mvenv .venv
        .venv/bin/pip install maturin ptpython
    fi

ptpython: venv
    .venv/bin/ptpython

develop: venv
    .venv/bin/maturin develop -m python-libdaw/Cargo.toml

clean:
    -rm -r .venv

# git tag with the current version and push
tag-and-push:
    git tag v{{version}}
    git push
    git push --tags
