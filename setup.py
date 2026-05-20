#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Build script for ChemPy - A chemistry toolkit for Python

This script handles compilation of Cython extensions.
Most configuration is in pyproject.toml (PEP 517/518).

Usage:
    python setup.py build_ext --inplace

Note:
    Cython extensions are optional but recommended for performance.
    The package can be used without compilation using pure Python modules.
"""

import os
import sys

import numpy
from setuptools import Extension, setup

# Check if Cython compilation should be skipped (e.g., on Windows CI)
skip_build = (
    os.environ.get("SKIP_CYTHON_BUILD", "").lower() in ("1", "true", "yes")
    or sys.platform == "win32"  # Skip on Windows due to compilation issues
)

try:
    import Cython.Compiler.Options

    # Create annotated HTML files for each of the Cython modules for debugging
    Cython.Compiler.Options.annotate = True
    cython_available = True and not skip_build
except ImportError:
    cython_available = False

if skip_build:
    if sys.platform == "win32":
        print("Info: Skipping Cython build on Windows. Pure Python modules will be used.")
    else:
        print("Info: Skipping Cython build (SKIP_CYTHON_BUILD set). Pure Python modules will be used.")
elif not cython_available:
    print("Warning: Cython not available. Pure Python modules will be used.")

# Define Cython extensions for performance-critical modules
ext_modules = [
    Extension("chempy.constants", ["chempy/constants.py"]),
    Extension("chempy.element", ["chempy/element.py"]),
    Extension("chempy.graph", ["chempy/graph.py"]),
    Extension("chempy.geometry", ["chempy/geometry.py"]),
    Extension("chempy.kinetics", ["chempy/kinetics.py"]),
    Extension("chempy.molecule", ["chempy/molecule.py"]),
    Extension("chempy.pattern", ["chempy/pattern.py"]),
    Extension("chempy.reaction", ["chempy/reaction.py"]),
    Extension("chempy.species", ["chempy/species.py"]),
    Extension("chempy.states", ["chempy/states.py"]),
    Extension("chempy.thermo", ["chempy/thermo.py"]),
    Extension("chempy.ext.thermo_converter", ["chempy/ext/thermo_converter.py"]),
]

# Only include extensions if Cython is available
if not cython_available:
    ext_modules = []

setup(
    ext_modules=ext_modules,
    include_dirs=[numpy.get_include()],
)
