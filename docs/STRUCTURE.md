# Project Structure

ChemPy follows modern Python project organization with clear separation of concerns.

## Directory Structure

```
ChemPy/
├── README.md                       # Project overview and quick start
├── CHANGELOG.md                    # Version history and release notes
├── TODO.md                         # Future improvements and known issues
├── CONTRIBUTING.md                 # Contribution guidelines
├── SECURITY.md                     # Security policy
├── LICENSE                         # MIT license
├── pyproject.toml                  # Modern Python packaging configuration
├── setup.py                        # Build script (mainly for Cython)
├── setup.cfg                       # Setup configuration
├── pytest.ini                      # pytest configuration
├── Makefile                        # Common development tasks
├── .pre-commit-config.yaml         # Pre-commit hooks configuration
├── .editorconfig                   # Editor configuration
├── .gitignore                      # Git ignore patterns
├── docs/                           # Developer documentation
│   ├── README.md                   # Documentation index
│   ├── DEVELOPMENT.md              # Development setup guide
│   ├── STRUCTURE.md                # Project structure (this file)
│   └── TYPE_HINTS.md               # Type annotation guidelines
├── documentation/                  # Sphinx API documentation
│   ├── source/                     # Documentation source files
│   ├── build/                      # Generated HTML documentation
│   └── Makefile                    # Sphinx build commands
├── benchmarks/                     # Performance benchmarking
│   ├── README.md                   # Benchmarking guide
│   ├── benchmark_graph.py          # Graph algorithm benchmarks
│   ├── benchmark_kinetics.py       # Kinetics calculation benchmarks
│   └── compare_benchmarks.py       # Benchmark comparison script
├── chempy/                         # Main package
│   ├── __init__.py                 # Package initialization
│   ├── constants.py                # Physical/chemical constants
│   ├── element.py                  # Element data and properties
│   ├── molecule.py                 # Molecular structures
│   ├── reaction.py                 # Chemical reactions
│   ├── kinetics.py                 # Kinetics calculations
│   ├── thermo.py                   # Thermodynamic calculations
│   ├── species.py                  # Species representation
│   ├── geometry.py                 # Geometry utilities
│   ├── graph.py                    # Graph-based algorithms
│   ├── pattern.py                  # Pattern matching
│   ├── states.py                   # Physical/chemical states
│   ├── exception.py                # Custom exceptions
│   ├── *.pxd                       # Cython declaration files
│   ├── py.typed                    # PEP 561 type marker
│   ├── io/                         # Input/output modules
│   │   ├── gaussian.py             # Gaussian format support
│   │   └── ...
│   └── ext/                        # Extensions
│       ├── molecule_draw.py        # Molecular visualization
│       └── thermo_converter.py     # Thermodynamic conversions
├── tests/                          # Modern test suite
│   ├── test_*.py                   # Modern pytest tests
│   └── conftest.py                 # Test configuration
├── unittest/                       # Legacy test suite
│   ├── *Test.py                    # Legacy unit tests
│   └── conftest.py                 # Test configuration
├── scripts/                        # Utility scripts
└── .github/                        # GitHub-specific files
    ├── workflows/                  # CI/CD workflows
    │   ├── lint-and-test.yml       # Main CI pipeline
    │   ├── benchmarks.yml          # Performance benchmarks
    │   └── *.yml                   # Other workflows
    ├── ISSUE_TEMPLATE/             # Issue templates
    ├── pull_request_template.md    # PR template
    └── CODE_OF_CONDUCT.md          # Community guidelines
```

## Key Design Principles

### 1. Modern Python Packaging (PEP 517/518)
- `pyproject.toml` as the single source of truth for project metadata
- Declarative configuration with setuptools build backend
- Optional Cython compilation for performance

### 2. Type Safety (PEP 561)
- `py.typed` marker for type checking support
- Type stubs (`.pyi`) for optional dependencies
- mypy configuration in `pyproject.toml`

### 3. Code Quality
- Pre-commit hooks for automatic formatting and linting
- Black for code formatting (line length 120)
- isort for import sorting
- flake8 for linting
- mypy for type checking

### 4. Testing Strategy
- `tests/` - Modern pytest-based tests with descriptive names
- `unittest/` - Legacy tests maintained for compatibility
- `benchmarks/` - Performance benchmarking suite
- pytest configuration in `pytest.ini`
- Coverage reporting with pytest-cov

### 5. Documentation
- `docs/` - Developer/technical documentation (Markdown)
- `documentation/` - User-facing API docs (Sphinx/reST)
- Inline docstrings following NumPy/Google style
- README for quick start and overview

### 6. CI/CD
- GitHub Actions workflows for all checks
- Matrix testing across Python 3.8-3.13
- Automated coverage reporting to Codecov
- Pre-commit hooks match CI checks

## Module Organization

### Core Modules
- **constants** - Physical and chemical constants
- **element** - Periodic table data and element properties
- **molecule** - Molecular structure representation
- **graph** - Graph data structures and algorithms
- **pattern** - Pattern matching for molecular structures

### Specialized Modules
- **reaction** - Chemical reaction representation
- **kinetics** - Reaction rate calculations
- **thermo** - Thermodynamic property calculations
- **species** - Chemical species with associated data
- **states** - Statistical mechanical states
- **geometry** - Molecular geometry utilities

### Extension Modules (`chempy/ext/`)
- **molecule_draw** - Molecular visualization (requires optional deps)
- **thermo_converter** - Thermodynamic data format conversions

### I/O Modules (`chempy/io/`)
- Format-specific readers and writers
- Gaussian, SMILES, InChI support (some require Open Babel)

## Build Artifacts

Generated files (not tracked in git):
- `*.c`, `*.html` - Cython-generated C code and annotated HTML
- `*.so`, `*.pyd` - Compiled extension modules
- `build/`, `dist/` - Build directories
- `*.egg-info/` - Package metadata
- `.coverage`, `coverage.xml` - Coverage reports
- `.mypy_cache/`, `.pytest_cache/` - Tool caches

## Development Workflow

1. Make changes to source code
2. Run tests: `make test`
3. Check formatting: `make format`
4. Run type checking: `make mypy`
5. Pre-commit hooks verify changes
6. CI runs on push/PR

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development instructions.
