# ChemPy - A Chemistry Toolkit for Python

[![Python 3.8+](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/downloads/)
[![Python 3.13](https://img.shields.io/badge/python-3.13-brightgreen.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
[![Checked with mypy](http://www.mypy-lang.org/static/mypy_badge.svg)](http://mypy-lang.org/)
[![Tests](https://github.com/elkins/ChemPy/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/elkins/ChemPy/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/elkins/ChemPy/branch/master/graph/badge.svg)](https://codecov.io/gh/elkins/ChemPy)
[![PEP 561 Compliant](https://img.shields.io/badge/pep-561-blue.svg)](https://www.python.org/dev/peps/pep-0561/)
[![Benchmarks](https://github.com/elkins/ChemPy/actions/workflows/benchmarks.yml/badge.svg?branch=master)](https://github.com/elkins/ChemPy/actions/workflows/benchmarks.yml?query=branch%3Amaster)

**ChemPy** is a free, open-source Python toolkit for chemistry, chemical engineering, and materials science applications.

## Quick Links

- 📖 **[Documentation](https://chempy.readthedocs.io)** - Full documentation and API reference
- 🐛 **[Issue Tracker](https://github.com/elkins/ChemPy/issues)** - Report bugs and request features
- 📝 **[Contributing](CONTRIBUTING.md)** - How to contribute
- 📋 **[Changelog](CHANGELOG.md)** - Version history and releases
- 🔐 **[Security](SECURITY.md)** - Security policy and reporting
- 👨‍💻 **[Development](DEVELOPMENT.md)** - Development setup and guidelines

## Features


## Platform Support

**Windows:** Experimental. Unit tests are not run on Windows in CI due to persistent failures and lack of a Windows development environment. Use at your own risk.

If you are able to help improve Windows compatibility, contributions and fixes are very welcome!

**macOS and Linux:** Fully supported and tested in CI.
## Installation

### Requirements

- **Python** 3.8 or later (3.12 or 3.13 recommended)
- **NumPy** 1.20.0 or later
- **SciPy** 1.7.0 or later (recommended)

### Optional Dependencies

- **Cython** - For building optimized extensions from source
- **OpenBabel** 2.2.0 or later - Additional molecular formats support
- **Cairo** 1.8.0 or later - Graphics and molecular drawing

### Quick Start

Install via pip:

```bash
pip install chempy
```

Or install from source with development dependencies:

```bash
git clone https://github.com/elkins/ChemPy.git
cd ChemPy
pip install -e ".[dev]"
make build
```

## Getting Started

```python
from chempy import constants, element, molecule

# Access physical constants
print(f"Avogadro constant: {constants.avogadro_constant}")

# Query element properties
h = element.Element.from_atomic_number(1)
print(f"Hydrogen mass: {h.mass} u")

# Create molecular structures
mol = molecule.Molecule()  # Create molecule
```

## Development

### Modernization Status

ChemPy has been fully modernized for Python 3.8-3.13:

- ✅ **Python 3.13 support** - All code updated and tested on latest Python
- ✅ **Open Babel 3.x integration** - Modern molecular format handling
- ✅ **Type hints (PEP 561)** - Full type annotation coverage with `py.typed` marker
- ✅ **Test suite (35 tests)** - 100% passing with zero warnings
- ✅ **Code quality** - Zero warnings, mypy strict checks, black formatted
- ✅ **GitHub Actions CI/CD** - Automated testing across Python 3.8-3.13
- ✅ **NumPy compatibility** - Fixed array-to-scalar deprecation warnings
- ✅ **Modern packaging** - PEP 517/518 compliant with pyproject.toml

See [MODERNIZATION_COMPLETE.md](MODERNIZATION_COMPLETE.md) for detailed migration notes.

### Setup Development Environment

```bash
# Install with all development tools
pip install -e ".[dev,docs]"

# Install pre-commit hooks for automatic quality checks
pre-commit install

# Build Cython extensions for performance
make build
```

### Running Tests

```bash
# Run all tests
make test

# Run with coverage report
make test-cov

# Run tests in parallel
make test-fast

# Run specific test file
pytest unittest/moleculeTest.py -v
```

### Benchmarking

ChemPy includes a small benchmark suite using `pytest-benchmark` to track performance of key hot-paths (SMILES parsing, rotor counting, density-of-states ILT, etc.).

Run locally:

```bash
pytest unittest/benchmarksTest.py --benchmark-only
```

Compare two runs (e.g., branch vs. main):

```bash
# On main
pytest unittest/benchmarksTest.py --benchmark-only --benchmark-save=main

# On your branch
pytest unittest/benchmarksTest.py --benchmark-only --benchmark-save=feature

# Compare
pytest unittest/benchmarksTest.py --benchmark-only --benchmark-compare
```

CI runs a quick benchmark job on Ubuntu/Python 3.12 and uploads JSON results as an artifact for trend tracking.

Latest CI benchmark artifacts (master):

- https://github.com/elkins/ChemPy/actions/workflows/benchmarks.yml?query=branch%3Amaster

Open the most recent run, then download the artifact named `benchmark-results-<OS>-py312`.

### Code Quality

```bash
# Format code automatically
make format

# Check code style
make lint

# Type checking
make type-check

# All quality checks
make check
```

### Building Documentation

```bash
make docs
cd documentation
open build/html/index.html
```

## Project Structure

```
chempy/
├── constants.py          # Physical constants (SI units)
├── element.py            # Element properties and data
├── molecule.py           # Molecular structure representation
├── reaction.py           # Chemical reactions
├── kinetics.py           # Reaction kinetics modeling
├── thermo.py             # Thermodynamic calculations
├── species.py            # Chemical species definitions
├── geometry.py           # Geometric calculations
├── graph.py              # Graph-based molecular analysis
├── pattern.py            # Pattern matching for molecules
├── states.py             # Physical/chemical state variables
├── py.typed              # PEP 561 type hint marker
└── ext/                  # Extensions
    ├── molecule_draw.py  # Molecular visualization
    └── thermo_converter.py  # Thermodynamics converters

tests/                     # Modern test directory
unittest/                  # Legacy test suite
docs/                      # Documentation
documentation/             # Sphinx documentation source
```

## Documentation

- [Development Guide](DEVELOPMENT.md) - Setup and development workflow
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Structure Overview](STRUCTURE.md) - Project organization
- [Modernization Notes](MODERNIZATION_STRUCTURE.md) - Recent updates

## Citation

If you use ChemPy in your research, please cite:

```bibtex
@software{chempy2024,
  title={ChemPy: A Chemistry Toolkit for Python},
  author={Allen, Joshua W.},
  year={2024},
  url={https://github.com/elkins/ChemPy}
}
```

## License

ChemPy is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- [RMG](https://rmgpy.github.io/) - Reaction Mechanism Generator
- [Cantera](https://cantera.org/) - Chemical kinetics and thermodynamics
- [OpenBabel](http://openbabel.org/) - Molecular structures and formats
- [GAMESS](https://www.msg.chem.iastate.edu/gamess/) - Quantum chemistry

## Support

For questions and discussions:
- Open an [issue](https://github.com/elkins/ChemPy/issues)
- Read the [documentation](https://chempy.readthedocs.io)
- Check [existing discussions](https://github.com/elkins/ChemPy/discussions)

## Acknowledgments

ChemPy was originally developed by Joshua W. Allen and is maintained by the open-source community.

---

Made with ❤️ for the chemistry community

