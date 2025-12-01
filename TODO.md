# TODO - Future Improvements

This document tracks medium-to-high risk improvements and known issues identified during the modernization effort. These items require careful consideration and broader refactoring efforts.

## Type Annotations

### High Priority
- **Comprehensive type checking with `--check-untyped-defs`**
  - Status: Not enabled (would introduce ~50 errors)
  - Effort: High (requires systematic function signature updates)
  - Risk: Medium (could expose edge cases in type handling)
  - Impact: Would enforce type annotations for all function parameters and returns
  - Recommendation: Tackle incrementally module-by-module after gaining confidence with current type coverage

### Medium Priority
- **Expand type annotations across all modules**
  - Current state: Core typing issues resolved, but coverage is partial
  - Many functions lack parameter and return type hints
  - Would improve IDE support and catch more bugs statically

## Algorithm Bugs

### Critical
- **Infinite loop in subgraph isomorphism with pattern R atoms**
  - Location: `unittest/moleculeTest.py::testSubgraphIsomorphismManyLabels` (currently skipped)
  - Root cause: Pattern matching algorithm enters infinite loop when handling generic R (any) atoms
  - Reproduction: Test generates molecules with many labeled atoms and attempts pattern matching
  - Impact: Test hangs indefinitely, consuming memory until system resources exhausted
  - Workaround: Test skipped with documentation
  - Fix needed: Algorithm refactoring in pattern matching logic to handle generic atoms correctly
  - Risk: High (core functionality bug in graph isomorphism)

## Documentation

### Low Priority
- **API documentation completeness**
  - Some internal functions documented in Sphinx but not part of public API
  - Consider clarifying public vs. internal API boundaries
  - Add more usage examples and tutorials

## Testing

### Medium Priority
- **Expand test coverage**
  - Current: Smoke tests for core modules
  - Goal: Comprehensive unit tests with edge cases
  - Focus areas: Pattern matching edge cases, thermodynamic edge conditions, complex reaction networks

- **Performance benchmarking**
  - Establish baseline performance metrics
  - Add regression tests for performance-critical paths (graph algorithms, pattern matching)

## Build & CI

### Low Priority
- **Cython optimization verification**
  - Verify Cython extensions are being built and used in CI
  - Add performance comparisons between pure Python and Cython implementations
  - Consider optional Cython builds for easier development

- **Additional CI checks**
  - Consider adding security scanning (bandit, safety)
  - Consider adding complexity metrics (radon)
  - Consider adding documentation build verification on PRs

## Dependencies

### Medium Priority
- **Open Babel optional dependency handling**
  - Currently required for some functionality (pybel imports)
  - Consider making it truly optional with graceful degradation
  - Add clear documentation for optional vs. required dependencies

## Code Quality

### Low Priority
- **Reduce complexity in core algorithms**
  - Some functions in `graph.py` and `molecule.py` are quite complex
  - Consider refactoring for maintainability
  - Add complexity thresholds to CI (e.g., max cyclomatic complexity)

---

## Notes
- This list compiled during 2024 modernization effort
- Items ordered by priority within each category
- Before tackling high-risk items, ensure comprehensive test coverage
- Consider creating issues in GitHub for tracking and discussion
