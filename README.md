# ChemPy - A Chemistry Toolkit for Python

[![Python 3.8+](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/downloads/)
[![Python 3.13](https://img.shields.io/badge/python-3.13-brightgreen.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
[![Checked with mypy](http://www.mypy-lang.org/static/mypy_badge.svg)](http://mypy-lang.org/)
[![GitHub Tests](https://github.com/elkins/ChemPy/workflows/Tests/badge.svg)](https://github.com/elkins/ChemPy/actions)
[![codecov](https://codecov.io/gh/elkins/ChemPy/branch/master/graph/badge.svg)](https://codecov.io/gh/elkins/ChemPy)

**ChemPy** is a free, open-source Python toolkit for chemistry, chemical engineering, and materials science applications.

## Quick Links

- 📖 **[Documentation](https://chempy.readthedocs.io)** - Full documentation and API reference
- 🐛 **[Issue Tracker](https://github.com/elkins/ChemPy/issues)** - Report bugs and request features
- 📝 **[Contributing](CONTRIBUTING.md)** - How to contribute
- 📋 **[Changelog](CHANGELOG.md)** - Version history
- 🔐 **[Security](SECURITY.md)** - Security policy

## Features

- Molecular structure representation and manipulation
- Chemical reactions and kinetics modeling
- Thermodynamic calculations
- Graph-based molecular analysis
- Pattern matching for molecular structures
- Optimized performance with Cython extensions
- Full type hint support with PEP 561 compliance
- Comprehensive test coverage
- Modern Python packaging (PEP 517/518)

## Installation

### Requirements

- **Python** 3.8 or later
- **NumPy** 1.20.0 or later
- **SciPy** 1.7.0 or later (recommended)
- **Cython** (for building from source)

### Optional Dependencies

- **OpenBabel** 2.2.0 or later (additional molecular formats)
- **Cairo** 1.8.0 or later (graphics support)

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

