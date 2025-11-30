#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Build script for ChemPy - A chemistry toolkit for Python

This script handles compilation of Cython extensions.
Most configuration is in pyproject.toml.
"""

from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext
import Cython.Compiler.Options
import numpy

# Create annotated HTML files for each of the Cython modules for debugging
Cython.Compiler.Options.annotate = True

# Define Cython extensions
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

setup(
    ext_modules=ext_modules,
    include_dirs=[numpy.get_include()],
)

