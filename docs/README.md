# ChemPy Developer Documentation

This directory contains technical documentation for ChemPy developers and contributors.

## Documentation Files

### Development Guides
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Development environment setup, build instructions, and testing
- **[TYPE_HINTS.md](TYPE_HINTS.md)** - Type annotation guidelines and mypy configuration
- **[STRUCTURE.md](STRUCTURE.md)** - Project structure and module organization

### Project Information
These files are in the root directory:
- **[../README.md](../README.md)** - Project overview, installation, and quick start
- **[../CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines and workflow
- **[../CHANGELOG.md](../CHANGELOG.md)** - Version history and release notes
- **[../TODO.md](../TODO.md)** - Future improvements and known issues
- **[../SECURITY.md](../SECURITY.md)** - Security policy and vulnerability reporting

### Specialized Documentation
- **[../benchmarks/README.md](../benchmarks/README.md)** - Performance benchmarking guide
- **[../documentation/](../documentation/)** - Sphinx API documentation source

## Building API Documentation

The Sphinx documentation is in the `documentation/` directory:

```bash
cd documentation
make html
# Output in documentation/build/html/
```

## Quick Links

- [GitHub Repository](https://github.com/elkins/ChemPy)
- [Issue Tracker](https://github.com/elkins/ChemPy/issues)
- [Contributing Guide](../CONTRIBUTING.md)
