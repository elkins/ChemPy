# Contributing to ChemPy

Thank you for your interest in contributing to ChemPy! We welcome contributions of all kinds.

## Getting Started

For detailed development setup instructions, see the [Developer Documentation](docs/DEVELOPMENT.md).

### 1. Set Up Your Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/ChemPy.git
cd ChemPy

# Create a virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install in development mode with all dependencies
make install-dev
```

### 2. Build Cython Extensions

```bash
make build
```

## Development Workflow

### Running Tests

```bash
# Run all tests
make test

# Run tests with coverage
make test-cov

# Run specific test file
pytest unittest/moleculeTest.py
```

### Code Quality

Before submitting a PR, ensure your code passes all quality checks:

```bash
# Format your code
make format

# Run linting
make lint

# Check type hints
make type-check

# Run everything
make all
```

### Code Style

- **Python**: Follow PEP 8 with 100-character line length (enforced by Black)
- **Formatting**: Use Black for formatting, isort for import organization
- **Type Hints**: Add type hints where possible
- **Documentation**: Include docstrings for all public functions and classes

## Submitting Changes

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and write tests

3. **Run quality checks**
   ```bash
   make all
   ```

4. **Commit with clear messages**
   ```bash
   git commit -m "Add clear description of changes"
   ```

5. **Push and create a Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```

## Pull Request Guidelines

- Include a clear description of changes
- Link related issues
- Ensure all tests pass
- Update documentation if needed
- Ensure code passes quality checks

## Reporting Issues

- Use GitHub Issues for bug reports
- Include Python version, OS, and traceback
- Provide minimal reproducible example
- Check if issue already exists

## Code of Conduct

- Be respectful and constructive
- Help others learn and grow
- Report harassment to maintainers

## Questions?

Feel free to open an issue or discussion if you have questions!
