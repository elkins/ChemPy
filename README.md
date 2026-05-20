# ChemPy (Rust)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> [!NOTE]
> **Project Evolution**
> This project has been converted from its original Python implementation to a **Pure Rust crate**. 
> While the Python version has been integrated into the [RMG-Py](https://github.com/ReactionMechanismGenerator/RMG-Py) ecosystem, this repository now serves as a high-performance Rust port of the foundational ChemPy toolkit.

**ChemPy (Rust)** is a high-performance toolkit for chemistry, chemical engineering, and materials science applications, with a focus on molecular structures, thermodynamics, and chemical kinetics.

## Features
- **Fast Graph Isomorphism:** Efficient VF2-based molecular graph comparison.
- **Thermodynamic Models:** Support for NASA polynomials and Wilhoit models.
- **Kinetics:** Arrhenius models and rate coefficient calculations.
- **States:** Partition function and density of states calculations.

## Getting Started
Add this to your `Cargo.toml`:
```toml
[dependencies]
chempy = { git = "https://github.com/elkins/ChemPy.git", branch = "rust-conversion" }
```

## Development
Run tests with:
```bash
cargo test
```

## Python Comparison (Legacy)
The original Python implementation is preserved in the `python/` directory for behavioral and performance comparison.

To run the original Python tests:
```bash
cd python
pip install -e .
pytest unittest/
```

To run original Python benchmarks:
```bash
cd python
pytest unittest/benchmarksTest.py --benchmark-only
```

## License
ChemPy is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments
ChemPy was originally developed by Joshua W. Allen in Python and has been ported to Rust to ensure its long-term performance and maintainability.
