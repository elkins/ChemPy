#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
ChemPy - A comprehensive chemistry toolkit for Python

A free, open-source Python toolkit for chemistry, chemical engineering,
and materials science applications.

Modules:
    constants: Physical and chemical constants
    element: Element properties and data
    molecule: Molecular structure representation
    reaction: Chemical reaction handling
    kinetics: Chemical kinetics tools
    thermo: Thermodynamic calculations
    species: Chemical species representation
    geometry: Molecular geometry utilities
    graph: Graph-based molecular analysis
    pattern: Pattern matching for molecules
    states: Physical and chemical states

Examples:
    >>> import chempy
    >>> from chempy import constants
    >>> print(constants.avogadro_constant)
"""

from __future__ import annotations

__version__ = "0.2.0"
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
    "exception",
]

# Lazy imports for better startup time
def __getattr__(name: str):
    """Lazy import of submodules."""
    if name in __all__:
        import importlib
        return importlib.import_module(f".{name}", __name__)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")

def __dir__():
    """Return list of public attributes."""
    return sorted(__all__ + ["__version__", "__author__", "__author_email__", "__license__"])


