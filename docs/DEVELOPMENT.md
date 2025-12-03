# ChemPy Development Guide

## Project Overview

ChemPy is a chemistry toolkit for Python with optimized performance through Cython extensions. This guide covers modern development practices and tooling.

## Quick Reference

| Task | Command |
|------|---------|
| Install for development | `make install-dev` |
| Build Cython extensions | `make build` |
| Run tests | `make test` |
| Check code quality | `make all` |
| Format code | `make format` |
| Build docs | `make docs` |

## Architecture

### Core Modules

- **constants.py**: Physical constants in SI units
- **element.py**: Element and atomic properties
- **molecule.py**: Molecular structure representation
- **reaction.py**: Chemical reactions
- **kinetics.py**: Reaction kinetics and rate laws
- **thermo.py**: Thermodynamic calculations
- **species.py**: Species definitions and properties
- **geometry.py**: Geometric calculations
- **graph.py**: Graph-based algorithms
- **pattern.py**: Molecular pattern matching
- **states.py**: State variables and properties

### Performance Optimization

All modules can be compiled as Cython extensions for significant performance improvements:

```bash
make build
```

This compiles `.py` files to C extensions automatically.

## Development Setup

### Environment Setup

```bash
# Create virtual environment
python -m venv venv
source venv/bin/activate

# Install with development dependencies
make install-dev

# Build Cython extensions
make build
```

### Pre-commit Hooks

Set up automatic code quality checks:

```bash
pip install pre-commit
pre-commit install
```

This runs formatters, linters, and type checks before each commit.

## Testing

### Test Structure

Tests are in `unittest/` directory organized by module:

- `moleculeTest.py` - Molecule tests
- `reactionTest.py` - Reaction tests
- `geometryTest.py` - Geometry tests
- `thermoTest.py` - Thermodynamic tests
- etc.

### Running Tests

```bash
# Run all tests
make test

# Run with coverage report
make test-cov

# Run specific test file
pytest unittest/moleculeTest.py

# Run specific test
pytest unittest/moleculeTest.py::TestClassName::test_method
```

## Code Quality

### Formatting

Code is formatted with Black (100-char lines) and isort (for imports):

```bash
make format
```

### Linting

Check code style:

```bash
make lint
```

### Type Checking

Validate type hints:

```bash
make type-check
```

### Pre-commit

Run all checks locally before pushing:

```bash
make all
```

## Documentation

### Building Docs

```bash
make docs
cd documentation
open build/html/index.html
```

### Writing Documentation

- Update RST files in `documentation/source/`
- Use Sphinx markup for proper formatting
- Link to API documentation when relevant

## Continuous Integration

GitHub Actions runs tests on:
- Multiple Python versions (3.8-3.12)
- Multiple OS (Ubuntu, macOS, Windows)
- Code quality checks (lint, type hints, format)

View workflows in `.github/workflows/`

## Release Process

1. Update version in `pyproject.toml`
2. Update `__version__` in `chempy/__init__.py`
3. Update CHANGELOG
4. Create git tag: `git tag v0.x.x`
5. Push: `git push && git push --tags`
6. Build: `python -m build`
7. Upload: `twine upload dist/*`

## Troubleshooting

### Cython build fails

```bash
# Clean and rebuild
make clean
make build
```

### Import errors

```bash
# Verify installation
pip install -e ".[dev]"

# Check imports
python -c "import chempy; print(chempy.__version__)"
```

### Tests fail

```bash
# Ensure Cython extensions are built
make build

# Run with verbose output
pytest -vv unittest/
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Resources

- **Cython**: http://cython.org/
- **pytest**: https://pytest.org/
- **Black**: https://github.com/psf/black
- **Sphinx**: https://www.sphinx-doc.org/
