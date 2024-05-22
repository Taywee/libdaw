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

