"""
ChemPy test suite configuration for pytest
"""

import sys
from pathlib import Path

import pytest  # noqa: F401

# Add the project root to path
sys.path.insert(0, str(Path(__file__).parent.parent))
