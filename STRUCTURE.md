"""
Project Structure Overview
==========================

Modern Python Project Layout
-----------------------------

The modernized ChemPy project follows current Python best practices:

.. code-block:: text

    ChemPy/
    ├── chempy/                      # Main package directory
    │   ├── __init__.py             # Package initialization
    │   ├── constants.py            # Physical/chemical constants
    │   ├── element.py              # Element data and properties
    │   ├── molecule.py             # Molecular structure
    │   ├── reaction.py             # Chemical reactions
    │   ├── kinetics.py             # Kinetics calculations
    │   ├── thermo.py               # Thermodynamic calculations
    │   ├── species.py              # Species representation
    │   ├── geometry.py             # Geometry utilities
    │   ├── graph.py                # Graph-based analysis
    │   ├── pattern.py              # Pattern matching
    │   ├── states.py               # Physical/chemical states
    │   ├── exception.py            # Custom exceptions
    │   ├── *.pxd                   # Cython type definitions
    │   └── ext/                    # Extensions
    │       ├── __init__.py
    │       ├── molecule_draw.py
    │       └── thermo_converter.py
    │
    ├── tests/                      # Test suite (recommended modern structure)
    │   ├── __init__.py
    │   ├── conftest.py             # Pytest configuration
    │   └── test_*.py               # Test modules
    │
    ├── unittest/                   # Legacy test directory (deprecated)
    │   ├── conftest.py             # Pytest configuration
    │   └── *Test.py               # Legacy test files
    │
    ├── docs/                       # Documentation
    │   ├── __init__.py
    │   ├── conf.py                 # Sphinx configuration
    │   └── source/                 # Documentation sources
    │       └── *.rst
    │
    ├── .github/                    # GitHub configuration
    │   └── workflows/
    │       └── tests.yml           # CI/CD workflows
    │
    ├── pyproject.toml              # Modern PEP 517/518 config
    ├── setup.py                    # Setup for Cython extensions
    ├── setup.cfg                   # Setup configuration
    ├── Makefile                    # Development tasks
    ├── .editorconfig               # Editor configuration
    ├── .pre-commit-config.yaml     # Pre-commit hooks
    ├── .gitignore                  # Git ignore rules
    ├── README.md                   # Project overview
    ├── CONTRIBUTING.md             # Contribution guidelines
    ├── DEVELOPMENT.md              # Development guide
    ├── CHANGELOG.md                # Version history
    ├── LICENSE                     # License file
    └── MODERNIZATION.md            # Modernization notes

Key Improvements
----------------

1. **pyproject.toml** - Single source of truth for project metadata
   - Replaces setup.cfg for configuration
   - PEP 517/518 compliant build backend
   - Tool configuration in one place (black, isort, mypy, pytest, etc.)

2. **Structured test directory** - Modern layout at ``tests/``
   - Separate from main package for clarity
   - Follows pytest conventions
   - Includes conftest.py for fixtures

3. **Documentation structure** - Dedicated docs folder
   - Sphinx configuration in docs/
   - Separate from source code
   - Easy to build with ``make docs``

4. **CI/CD workflows** - Automated testing and deployment
   - GitHub Actions workflows in .github/workflows/
   - Runs on multiple Python versions
   - Code quality checks

5. **Development tools** - Modern Python tooling
   - Pre-commit hooks for quality gates
   - EditorConfig for consistency
   - Comprehensive Makefile targets

Package Organization
--------------------

The main package ``chempy`` is organized by functionality:

- **Data & Constants**: constants, element
- **Molecular**: molecule, species, geometry, graph, pattern
- **Reactions**: reaction, kinetics
- **Thermodynamics**: thermo
- **Utilities**: states, exception
- **Extensions**: ext/ (optional advanced features)

Cython Support
--------------

Type definitions (.pxd files) enable performance optimizations:

- element.pxd
- molecule.pxd
- graph.pxd
- geometry.pxd
- kinetics.pxd
- pattern.pxd
- reaction.pxd
- species.pxd
- states.pxd
- thermo.pxd

These are compiled via setup.py during installation.

Dependencies
------------

Core dependencies:
- numpy: Numerical computing
- scipy: Scientific computing

Development dependencies:
- pytest: Testing framework
- black: Code formatting
- isort: Import sorting
- mypy: Type checking
- flake8: Style checking
- sphinx: Documentation generation

Migration Path
--------------

Existing code continues to work, but migration recommendations:

1. Move tests from ``unittest/`` to ``tests/`` (optional)
2. Use new ``pyproject.toml`` for all configuration
3. Adopt modern type hints
4. Follow PEP 8 styling
5. Add docstrings with type information

"""
