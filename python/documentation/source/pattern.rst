*****************************************************************
:mod:`chempy.pattern` --- Molecular Substructure Pattern Matching
*****************************************************************

.. automodule:: chempy.pattern

AtomPattern Objects
===================

.. autoclass:: chempy.pattern.AtomPattern
    :members:

BondPattern Objects
===================

.. autoclass:: chempy.pattern.BondPattern
    :members:

MoleculePattern Objects
=======================

.. autoclass:: chempy.pattern.MoleculePattern
    :members:

Working with Atom Types
=======================

.. note::
    The previous references to ``atomTypesEquivalent`` and
    ``atomTypesSpecificCaseOf`` have been removed as these
    functions are not part of the public API.

.. autofunction:: chempy.pattern.getAtomType

Adjacency Lists
===============

.. autofunction:: chempy.pattern.fromAdjacencyList

.. autofunction:: chempy.pattern.toAdjacencyList
