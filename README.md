# ChemPy - A Chemistry Toolkit for Python

[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**ChemPy** is a free, open-source Python toolkit for chemistry, chemical engineering, and materials science applications.

## Features

- Molecular structure representation and manipulation
- Chemical reactions and kinetics modeling
- Thermodynamic calculations
- Graph-based molecular analysis
- Pattern matching for molecular structures
- Optimized performance with Cython extensions

## Installation

### Requirements

- **Python** 3.8 or later
- **NumPy** 1.20.0 or later
- **SciPy** 1.7.0 or later (recommended for full functionality)
- **Cython** (for compilation)

### Optional Dependencies

- **OpenBabel** 2.2.0 or later (for additional molecular formats)
- **Cairo** 1.8.0 or later (for graphics)

### Quick Start

Install with pip:

```bash
pip install -e .
```

### Building Cython Extensions

Cython extensions provide significant performance improvements. To compile them:

```bash
pip install -e ".[dev]"
python setup.py build_ext --inplace
```

Or using the Makefile:

```bash
make
```

## Development

### Setup Development Environment

```bash
pip install -e ".[dev]"
```

### Running Tests

```bash
pytest
```

With coverage:

```bash
pytest --cov=chempy
```

### Code Quality

Format code with Black:

```bash
black chempy unittest
```

Check imports with isort:

```bash
isort chempy unittest
```

Lint with flake8:

```bash
flake8 chempy unittest
```

Type checking with mypy:

```bash
mypy chempy
```

### Building Documentation

```bash
pip install -e ".[docs]"
cd documentation
make html
```

## Project Structure

```
chempy/
├── constants.py          # Physical constants
├── element.py            # Element properties
├── molecule.py           # Molecular structures
├── reaction.py           # Chemical reactions
├── kinetics.py           # Reaction kinetics
├── thermo.py             # Thermodynamic calculations
├── species.py            # Species definitions
├── geometry.py           # Geometric calculations
├── graph.py              # Graph algorithms
├── pattern.py            # Pattern matching
├── states.py             # State variables
└── ext/                  # Additional extensions
    └── thermo_converter.py

unittest/                 # Test suite

documentation/            # Sphinx documentation
```

## License

ChemPy is licensed under the MIT License - see [COPYING.txt](COPYING.txt) for details.

## Citation

If you use ChemPy in your research, please cite:

```
ChemPy - A chemistry toolkit for Python
Joshua W. Allen et al.
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Changelog

### Version 0.1.0

Initial release with basic chemistry toolkit functionality.
