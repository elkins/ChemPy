"""Pytest configuration for ChemPy tests."""

import pytest


@pytest.fixture
def sample_molecule():
    """Provide a sample molecule for testing."""
    try:
        from chempy import molecule

        return molecule.Molecule()
    except ImportError:
        return None


@pytest.fixture
def sample_reaction():
    """Provide a sample reaction for testing."""
    try:
        from chempy import reaction

        return reaction.Reaction()
    except ImportError:
        return None
