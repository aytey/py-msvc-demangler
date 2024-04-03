py-msvc-demangler: Demangles MSVC C++ linker symbols
============================================================

.. image:: https://github.com/aytey/py-msvc-demangler/workflows/Build/badge.svg?branch=master
    :target: https://github.com/aytey/py-msvc-demangler/actions?query=branch%3Amaster

A package for demangling C++ linker symbol strings

This package provides python bindings for the rust crate
`msvc_demangler <https://github.com/mstange/msvc-demangler-rust>`_ by building
a native Python extension using `PyO3 <https://github.com/pyO3/pyO3>`_.

This is mainly an experiment in creating python extensions in Rust.
`A blog post about this is here.
<https://www.benfrederickson.com/writing-python-extensions-in-rust-using-pyo3/>`_

Usage
-------------------

To install

.. code-block:: python

    pip install msvc-demangler


Building from source requires the nightly version of the rust compiler.

This module exposes a single function that transforms C++ linker symbols to a human readable
representation.

.. code-block:: python

    from msvc_demangler import demangle

    print(demangle('??_0klass@@QEAAHH@Z'))
    # prints 'public: int __cdecl klass::operator/=(int)'

Released under the MIT License
