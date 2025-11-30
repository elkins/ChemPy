#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
ChemPy - A chemistry toolkit for Python

A free, open-source Python toolkit for chemistry, chemical engineering,
and materials science applications.
"""

__version__ = "0.1.0"
__author__ = "Joshua W. Allen"
__author_email__ = "jwallen@mit.edu"
__license__ = "MIT"

# Version info for different purposes
version_info = tuple(map(int, __version__.split(".")))

__all__ = [
    "constants",
    "element",
    "molecule",
    "reaction",
    "kinetics",
    "thermo",
    "species",
    "geometry",
    "graph",
    "pattern",
    "states",
]

# Lazy imports for better startup time
def __getattr__(name: str):
    """Lazy import of submodules."""
    if name in __all__:
        import importlib
        return importlib.import_module(f".{name}", __name__)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")

