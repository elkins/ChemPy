#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Benchmarks for graph operations (isomorphism, cycle finding).
"""

import pytest

from chempy.molecule import Atom, Bond, Molecule


class TestGraphIsomorphism:
    """Benchmark graph isomorphism operations."""

    @pytest.fixture(autouse=True)
    def setup(self):
        """Create test molecules for benchmarking."""
        # Create a simple ethane molecule
        self.ethane = Molecule()
        c1 = Atom(element="C")
        c2 = Atom(element="C")
        self.ethane.addAtom(c1)
        self.ethane.addAtom(c2)
        self.ethane.addBond(c1, c2, Bond(order=1))

        # Create a propane molecule
        self.propane = Molecule()
        c1 = Atom(element="C")
        c2 = Atom(element="C")
        c3 = Atom(element="C")
        self.propane.addAtom(c1)
        self.propane.addAtom(c2)
        self.propane.addAtom(c3)
        self.propane.addBond(c1, c2, Bond(order=1))
        self.propane.addBond(c2, c3, Bond(order=1))

        # Create a benzene molecule (cyclic)
        self.benzene = Molecule()
        carbons = [Atom(element="C") for _ in range(6)]
        for c in carbons:
            self.benzene.addAtom(c)
        for i in range(6):
            bond_order = 2 if i % 2 == 0 else 1
            self.benzene.addBond(carbons[i], carbons[(i + 1) % 6], Bond(order=bond_order))

    def test_isomorphism_simple(self, benchmark):
        """Benchmark simple isomorphism check between identical molecules."""
        result = benchmark(self.ethane.isIsomorphic, self.ethane)
        assert result

    def test_isomorphism_different_sizes(self, benchmark):
        """Benchmark isomorphism check between different sized molecules."""
        result = benchmark(self.ethane.isIsomorphic, self.propane)
        assert not result

    def test_isomorphism_cyclic(self, benchmark):
        """Benchmark isomorphism check with cyclic molecules."""
        result = benchmark(self.benzene.isIsomorphic, self.benzene)
        assert result


class TestGraphCycles:
    """Benchmark cycle finding algorithms."""

    @pytest.fixture(autouse=True)
    def setup(self):
        """Create cyclic test molecules."""
        # Create cyclopropane (3-membered ring)
        self.cyclopropane = Molecule()
        c1, c2, c3 = Atom(element="C"), Atom(element="C"), Atom(element="C")
        self.cyclopropane.addAtom(c1)
        self.cyclopropane.addAtom(c2)
        self.cyclopropane.addAtom(c3)
        self.cyclopropane.addBond(c1, c2, Bond(order=1))
        self.cyclopropane.addBond(c2, c3, Bond(order=1))
        self.cyclopropane.addBond(c3, c1, Bond(order=1))

        # Create cyclohexane (6-membered ring)
        self.cyclohexane = Molecule()
        carbons = [Atom(element="C") for _ in range(6)]
        for c in carbons:
            self.cyclohexane.addAtom(c)
        for i in range(6):
            self.cyclohexane.addBond(carbons[i], carbons[(i + 1) % 6], Bond(order=1))

    def test_get_smallest_set_of_smallest_rings_small(self, benchmark):
        """Benchmark SSSR algorithm on small ring."""
        result = benchmark(self.cyclopropane.getSmallestSetOfSmallestRings)
        assert len(result) == 1
        assert len(result[0]) == 3

    def test_get_smallest_set_of_smallest_rings_medium(self, benchmark):
        """Benchmark SSSR algorithm on medium ring."""
        result = benchmark(self.cyclohexane.getSmallestSetOfSmallestRings)
        assert len(result) == 1
        assert len(result[0]) == 6


class TestGraphCopy:
    """Benchmark graph copy operations."""

    @pytest.fixture(autouse=True)
    def setup(self):
        """Create test molecules of various sizes."""
        # Small molecule
        self.small = Molecule()
        c1, c2 = Atom(element="C"), Atom(element="C")
        self.small.addAtom(c1)
        self.small.addAtom(c2)
        self.small.addBond(c1, c2, Bond(order=1))

        # Medium molecule (decane - 10 carbons)
        self.medium = Molecule()
        carbons = [Atom(element="C") for _ in range(10)]
        for c in carbons:
            self.medium.addAtom(c)
        for i in range(9):
            self.medium.addBond(carbons[i], carbons[i + 1], Bond(order=1))

    def test_copy_small(self, benchmark):
        """Benchmark copying small molecule."""
        result = benchmark(self.small.copy, deep=True)
        assert result is not self.small
        assert result.isIsomorphic(self.small)

    def test_copy_medium(self, benchmark):
        """Benchmark copying medium molecule."""
        result = benchmark(self.medium.copy, deep=True)
        assert result is not self.medium
        assert result.isIsomorphic(self.medium)
