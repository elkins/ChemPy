#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
Compare benchmark results between pure Python and Cython implementations.

Usage:
    python compare_benchmarks.py <pure_python.json> <cython.json>
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Tuple


def load_benchmark_results(filepath: str) -> Dict:
    """Load benchmark results from JSON file."""
    with open(filepath, "r") as f:
        return json.load(f)


def calculate_speedup(pure_python_time: float, cython_time: float) -> float:
    """Calculate speedup factor (how many times faster)."""
    if cython_time == 0:
        return float("inf")
    return pure_python_time / cython_time


def format_time(seconds: float) -> str:
    """Format time in human-readable units."""
    if seconds < 1e-6:
        return f"{seconds * 1e9:.2f} ns"
    elif seconds < 1e-3:
        return f"{seconds * 1e6:.2f} μs"
    elif seconds < 1:
        return f"{seconds * 1e3:.2f} ms"
    else:
        return f"{seconds:.2f} s"


def compare_benchmarks(pure_python_results: Dict, cython_results: Dict) -> List[Tuple[str, float, float, float]]:
    """
    Compare benchmark results and calculate speedups.

    Returns list of tuples: (test_name, pure_python_time, cython_time, speedup)
    """
    comparisons = []

    # Extract benchmarks from results
    pure_benchmarks = {b["fullname"]: b for b in pure_python_results.get("benchmarks", [])}
    cython_benchmarks = {b["fullname"]: b for b in cython_results.get("benchmarks", [])}

    # Find common benchmarks
    common_tests = set(pure_benchmarks.keys()) & set(cython_benchmarks.keys())

    for test_name in sorted(common_tests):
        pure_result = pure_benchmarks[test_name]
        cython_result = cython_benchmarks[test_name]

        # Use mean time for comparison
        pure_time = pure_result["stats"]["mean"]
        cython_time = cython_result["stats"]["mean"]

        speedup = calculate_speedup(pure_time, cython_time)
        comparisons.append((test_name, pure_time, cython_time, speedup))

    return comparisons


def print_comparison_table(comparisons: List[Tuple[str, float, float, float]]) -> None:
    """Print formatted comparison table."""
    if not comparisons:
        print("No common benchmarks found to compare.")
        return

    print("| Test Name | Pure Python | Cython | Speedup |")
    print("|-----------|-------------|--------|---------|")

    for test_name, pure_time, cython_time, speedup in comparisons:
        # Shorten test name for readability
        short_name = test_name.split("::")[-1]
        speedup_str = f"{speedup:.2f}x" if speedup != float("inf") else "∞"

        print(f"| {short_name} | {format_time(pure_time)} | {format_time(cython_time)} | **{speedup_str}** |")

    # Calculate summary statistics
    speedups = [s for _, _, _, s in comparisons if s != float("inf")]
    if speedups:
        avg_speedup = sum(speedups) / len(speedups)
        max_speedup = max(speedups)
        min_speedup = min(speedups)

        print()
        print("### Summary")
        print(f"- **Average Speedup:** {avg_speedup:.2f}x")
        print(f"- **Maximum Speedup:** {max_speedup:.2f}x")
        print(f"- **Minimum Speedup:** {min_speedup:.2f}x")
        print(f"- **Tests Compared:** {len(comparisons)}")

        # Performance verdict
        if avg_speedup > 2.0:
            print("\n✅ **Cython provides significant performance improvement!**")
        elif avg_speedup > 1.2:
            print("\n✅ **Cython provides moderate performance improvement.**")
        elif avg_speedup > 1.0:
            print("\n⚠️ **Cython provides minor performance improvement.**")
        else:
            print(
                "\n⚠️ **No significant performance improvement from Cython.** "
                "Consider profiling to identify bottlenecks."
            )


def main():
    """Main entry point."""
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <pure_python.json> <cython.json>")
        sys.exit(1)

    pure_python_file = Path(sys.argv[1])
    cython_file = Path(sys.argv[2])

    if not pure_python_file.exists():
        print(f"Error: File not found: {pure_python_file}")
        sys.exit(1)

    if not cython_file.exists():
        print(f"Error: File not found: {cython_file}")
        sys.exit(1)

    # Load results
    pure_python_results = load_benchmark_results(str(pure_python_file))
    cython_results = load_benchmark_results(str(cython_file))

    # Compare and print
    comparisons = compare_benchmarks(pure_python_results, cython_results)
    print_comparison_table(comparisons)


if __name__ == "__main__":
    main()
