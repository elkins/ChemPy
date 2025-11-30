# Project Structure Modernization

## Overview

The ChemPy project has been modernized to follow current Python best practices and industry standards.

## Key Structural Changes

### 1. **Package Layout**
- ✅ Main package remains at `chempy/`
- ✅ Added `tests/` directory for test suite (modern pytest convention)
- ✅ Added `docs/` directory for documentation
- ✅ Legacy `unittest/` directory maintained for backward compatibility

### 2. **Configuration Consolidation**
- ✅ **pyproject.toml** - Primary configuration file (PEP 517/518)
  - All tool configurations centralized
  - Build system definition
  - Dependency management
  - Development tool settings (black, isort, mypy, pytest, coverage, pylint)

- ✅ **setup.cfg** - Setuptools configuration for broader compatibility
  - Mirrors pyproject.toml settings
  - Compatible with older tooling

- ✅ **setup.py** - Build script for Cython extensions
  - Handles compilation of .pxd/.pyx files
  - Required while Cython modules are in use

### 3. **Development Infrastructure**
- ✅ **Makefile** - Enhanced with modern targets
  - Supports both old and new test locations
  - Parallel test execution
  - Coverage reporting
  - Type checking
  - Code formatting

- ✅ **.github/workflows/** - CI/CD automation
  - Automated testing on multiple Python versions
  - Code quality checks

- ✅ **.pre-commit-config.yaml** - Automated checks before commits
  - Code formatting
  - Linting
  - Type checking

- ✅ **.editorconfig** - IDE consistency
  - Consistent formatting across editors

### 4. **Documentation Structure**
- ✅ **docs/** - Documentation source directory
  - `docs/conf.py` - Sphinx configuration
  - `docs/source/` - Documentation content (from original documentation/)

- ✅ **docs/__init__.py** - Package marker for docs module

### 5. **Testing Infrastructure**
- ✅ **tests/** - Modern test directory
  - `tests/__init__.py` - Test package marker
  - `tests/conftest.py` - Pytest configuration and fixtures
  - Follows pytest naming conventions (test_*.py)

- ✅ **unittest/conftest.py** - Updated pytest configuration
  - Maintains backward compatibility
  - Supports legacy test files (*Test.py)

### 6. **Version & Metadata**
- ✅ Updated version to 0.2.0 (reflecting modernization)
- ✅ Enhanced package metadata in pyproject.toml
  - Added maintainers
  - Added documentation links
  - Added repository links
  - More comprehensive classifiers

## Benefits of This Structure

### 1. **Standards Compliance**
- Follows PEP 427, 517, 518 for modern Python packaging
- Complies with pytest conventions
- Follows Sphinx documentation standards

### 2. **Better Tooling**
- Single source of truth in pyproject.toml
- Automatic tooling discovery
- Reduced configuration duplication
- Modern CI/CD pipelines

### 3. **Improved Testing**
- Clear separation of tests from code
- Support for both legacy and modern test locations
- Parallel test execution capability
- Enhanced fixtures in conftest.py

### 4. **Documentation**
- Organized documentation structure
- Sphinx-compatible layout
- Better RTD integration

### 5. **Developer Experience**
- Comprehensive Makefile targets
- Pre-commit hooks for quality gates
- EditorConfig for consistency
- Clear development guidelines (DEVELOPMENT.md, CONTRIBUTING.md)

## Migration Path

### For Existing Code
✅ **No breaking changes** - All existing code continues to work

### For New Development
1. Use `tests/` directory for new tests
2. Follow modern test naming conventions (`test_*.py`)
3. Add type hints to new code
4. Use modern f-strings and syntax (Python 3.8+)

### For Contributors
1. Use `pre-commit install` for automated checks
2. Run `make check` before committing
3. Follow guidelines in CONTRIBUTING.md
4. Reference DEVELOPMENT.md for setup

## File Structure Summary

```
chempy/                    - Main package (unchanged)
├── *.py                  - Module files
├── *.pxd                 - Cython type definitions
└── ext/                  - Extension modules

tests/                     - Modern test directory (NEW)
├── __init__.py
├── conftest.py
└── test_*.py

unittest/                  - Legacy test directory (maintained)
├── conftest.py
└── *Test.py

docs/                      - Documentation directory (NEW)
├── __init__.py
├── conf.py               - Sphinx config
└── source/               - Documentation files

pyproject.toml            - Modern config (ENHANCED)
setup.cfg                 - Setuptools config (NEW)
setup.py                  - Build config (maintained for Cython)
Makefile                  - Build targets (ENHANCED)
```

## Configuration Files

### pyproject.toml (Primary)
Centralized configuration for:
- Project metadata
- Dependencies and optional features
- Build system
- Tool configurations:
  - black (formatting)
  - isort (imports)
  - mypy (type checking)
  - pylint (linting)
  - pytest (testing)
  - coverage (reporting)

### setup.py
Specialized configuration for:
- Cython extension compilation
- NumPy include paths
- Custom build options

### Makefile
Development workflow automation:
```bash
make help          # Show all available commands
make install-dev   # Install with dev dependencies
make check         # Run all quality checks
make test          # Run test suite
make format        # Auto-format code
make docs          # Build documentation
```

## Version Information

- **Previous version**: 0.1.0 (Alpha)
- **Current version**: 0.2.0 (Beta - modernization)
- Reflects maturity improvements with modern tooling

## Next Steps

1. ✅ Core modernization complete
2. Plan: Migrate more tests to `tests/` directory
3. Plan: Add type hints to codebase
4. Plan: Publish to PyPI
5. Plan: Setup ReadTheDocs
6. Plan: Establish CI/CD badge requirements

## References

- [PEP 427 - Wheel Binary Format](https://peps.python.org/pep-0427/)
- [PEP 517 - Build Backend Interface](https://peps.python.org/pep-0517/)
- [PEP 518 - pyproject.toml](https://peps.python.org/pep-0518/)
- [pytest Documentation](https://docs.pytest.org/)
- [Sphinx Documentation](https://www.sphinx-doc.org/)
- [setuptools Documentation](https://setuptools.pypa.io/)
