# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Modern Python packaging with `pyproject.toml`
- GitHub Actions CI/CD workflow with Python 3.12 and 3.13 matrix testing
- Pre-commit hooks configuration
- pytest test runner configuration with coverage reporting
- Type hints support with mypy (compatible with mypy 1.10.1+)
- EditorConfig for consistent formatting
- Comprehensive development documentation
- CONTRIBUTING guide
- Modern Makefile with development tasks
- Smoke tests for species, thermo, kinetics, reaction, states, and TST modules
- Codecov integration with badges in documentation
- Sphinx documentation with mathjax support

### Changed
- Migrated from distutils to setuptools
- Updated README to Markdown format with CI/CD and coverage badges
- Improved .gitignore with modern Python patterns and coverage files
- Enhanced Makefile with quality checks
- Expanded Python support to 3.8 through 3.13
- Aligned code formatting tools (black line-length 120 in CI, flake8 max-line-length 120)
- Updated Sphinx documentation configuration and templates

### Fixed
- Type annotations in `pattern.py` (added overloads for `fromAdjacencyList`)
- Type annotations in `molecule.py` and `graph.py` for mypy compatibility
- Sphinx documentation warnings (removed broken autodoc references)
- README formatting issues (License section, badge formatting)
- Cython compatibility with mypy using targeted casts and TYPE_CHECKING imports

### Known Issues
- Test `testSubgraphIsomorphismManyLabels` in `moleculeTest.py` skipped due to infinite loop with pattern R atoms (documented for future fix)

## [0.1.0] - 2010-XX-XX

### Added
- Initial ChemPy release
- Core modules: constants, element, molecule, reaction, kinetics, thermo
- Cython extensions for performance optimization
- Graph-based molecular algorithms
- Pattern matching capabilities
- Thermodynamic calculations
- Reaction kinetics modeling

[Unreleased]: https://github.com/elkins/ChemPy/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/elkins/ChemPy/releases/tag/v0.1.0
