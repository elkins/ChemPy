# Benchmarking Pure Python vs Cython Performance

This directory contains benchmarking infrastructure to compare the performance of pure Python implementations versus Cython-compiled extensions.

## Overview

ChemPy uses a hybrid approach where:
- All modules are written as `.py` files that work with pure Python
- The same `.py` files can be compiled with Cython for performance improvements
- A compatibility layer (`_cython_compat.py`) allows graceful fallback when Cython is unavailable

This benchmarking suite measures the actual performance difference between these modes.

## Structure

- `benchmark_graph.py` - Graph operations (isomorphism, cycles, copying)
- `benchmark_kinetics.py` - Reaction kinetics calculations
- `compare_benchmarks.py` - Script to compare and analyze benchmark results
- `conftest.py` - pytest configuration for benchmarks

## Running Benchmarks Locally

### Pure Python Mode

```bash
# Without Cython compiled
pytest benchmarks/ --benchmark-only
```

### Cython Mode

```bash
# First, compile Cython extensions
pip install cython
python setup.py build_ext --inplace

# Then run benchmarks
pytest benchmarks/ --benchmark-only
```

### Compare Results

```bash
# Run both modes and save results
pytest benchmarks/ --benchmark-only --benchmark-json=benchmark-python.json  # Pure Python
python setup.py build_ext --inplace
pytest benchmarks/ --benchmark-only --benchmark-json=benchmark-cython.json  # Cython

# Compare
python benchmarks/compare_benchmarks.py benchmark-python.json benchmark-cython.json
```

## CI Integration

The GitHub Actions workflow (`.github/workflows/benchmarks.yml`) automatically:
1. Runs benchmarks in both pure Python and Cython modes
2. Compares the results
3. Posts a summary to the workflow output

Trigger manually via: **Actions → Benchmarks → Run workflow**

## Adding New Benchmarks

Create test functions using pytest-benchmark:

```python
def test_my_operation(benchmark):
    """Benchmark description."""
    result = benchmark(my_function, arg1, arg2)
    assert result  # Optional validation
```

Follow these patterns:
- Group related benchmarks in classes
- Use descriptive test names
- Include fixtures for test data setup
- Add assertions to validate correctness
- Test various problem sizes (small, medium, large)

## Expected Performance Gains

Cython typically provides speedups in:
- **Graph algorithms** (isomorphism, cycle detection) - 2-5x
- **Numerical calculations** (kinetics, thermodynamics) - 1.5-3x
- **Data structure operations** (copying, merging) - 1.5-2.5x

Areas with less improvement:
- I/O operations
- Python object creation/manipulation
- Code dominated by library calls (NumPy, SciPy)

## Troubleshooting

**Problem:** "No module named 'chempy'"
- Ensure you're running from the project root
- Install in development mode: `pip install -e .`

**Problem:** Cython extensions not being used
- Check for `.so` or `.pyd` files in `chempy/` directory
- Verify build succeeded: `python setup.py build_ext --inplace`
- Import and check: `from chempy._cython_compat import HAS_CYTHON`

**Problem:** Benchmark results are unstable
- Increase rounds: `--benchmark-min-rounds=10`
- Use `--benchmark-warmup=on`
- Close other applications to reduce system noise
