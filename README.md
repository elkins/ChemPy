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

## Glossary & Acronyms
- **SMILES**: Simplified Molecular Input Line Entry System. A notation for representing chemical structures as strings.
- **InChI**: International Chemical Identifier. A standardized string representation for chemical substances.
- **NASA Polynomials**: A standard format (initially developed by NASA) for representing thermodynamic data ($C_p, H, S$) as a function of temperature.
- **Wilhoit Model**: A robust thermodynamic model for heat capacity that ensures physical behavior at extremely low ($T \to 0$) and high ($T \to \infty$) temperatures.
- **VF2 Algorithm**: A high-performance algorithm for graph and subgraph isomorphism matching, used here for molecular pattern recognition.
- **GA (Group Additivity)**: A method for estimating thermodynamic properties of molecules based on their constituent functional groups.
- **SCF Energy**: Self-Consistent Field energy; the electronic energy of a molecule calculated via quantum chemical methods (e.g., Gaussian).
- **Partition Function**: A function that describes the statistical properties of a system in thermodynamic equilibrium.

## References
- **Pitzer, K. S.** (1946). "The Energy Levels of Restricted Rotors." *J. Chem. Phys.* **14**, p. 239-243. (Used for hindered rotor calculations).
- **East, A. L. L. and Radom, L.** (1997). "Ab initio statistical thermodynamical models for the computation of high-temperature thermodynamic functions." *J. Chem. Phys.* **106**, p. 6655-6674. (Foundational for `StatesModel`).
- **Cordella, L. P., Foggia, P., Sansone, C., and Vento, M.** (2004). "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs." *IEEE Trans. Pattern Anal. Mach. Intell.* **26**, p. 1367-1372. (The VF2 algorithm used in `graph.rs`).
- **Wilhoit, R. C.** (1975). "Thermodynamic properties of normal and branched alkanes." *J. Phys. Chem. Ref. Data* **2**, p. 427-437. (The Wilhoit heat capacity model).
- **Burcat, A. and Ruscic, B.** (2005). "Third Millennium Ideal Gas Thermodynamic Data for Combustion and Air-Pollution Use." *TAE 960 Report*. (Source for NASA polynomial standards).

## License
ChemPy is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments
ChemPy was originally developed by Joshua W. Allen in Python and has been ported to Rust to ensure its long-term performance and maintainability.
