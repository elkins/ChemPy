#!/usr/bin/env python3
"""
Compare the latest pytest-benchmark results against the previous run.
Reads JSON files under `.benchmarks` and prints a concise delta report.
"""
from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from pathlib import Path
from typing import Any, Dict, List

BENCH_ROOT = Path(".benchmarks")


def _find_runs() -> List[Path]:
    if not BENCH_ROOT.exists():
        return []
    # Plugin stores files like 0001_latest.json under implementation folder
    return sorted(BENCH_ROOT.rglob("*.json"))


def _load(path: Path) -> Dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as f:
            return json.load(f)
    except Exception as exc:
        print(f"Failed to load benchmark file {path}: {exc}")
        return {"benchmarks": []}


def _extract(entries: List[Dict[str, Any]]) -> Dict[str, Dict[str, float]]:
    out: Dict[str, Dict[str, float]] = {}
    for e in entries or []:
        name = e.get("name") or e.get("fullname")
        if not name:
            # skip malformed entries
            continue
        stats = e.get("stats") or {}
        # Focus on stable metrics
        out[str(name)] = {
            "min": float(stats.get("min", 0.0)),
            "max": float(stats.get("max", 0.0)),
            "mean": float(stats.get("mean", 0.0)),
            "stddev": float(stats.get("stddev", 0.0)),
            "median": float(stats.get("median", 0.0)),
            "iqr": float(stats.get("iqr", 0.0)),
            "ops": float(stats.get("ops", 0.0)),
            "rounds": float(stats.get("rounds", 0.0)),
            "iterations": float(stats.get("iterations", 0.0)),
        }
    return out


def _fmt_delta(curr: float, prev: float) -> str:
    if prev == 0.0:
        return "n/a"
    delta = (curr - prev) / prev * 100.0
    sign = "+" if delta >= 0 else ""
    return f"{sign}{delta:.2f}%"


def compare() -> int:
    parser = argparse.ArgumentParser(description="Compare pytest-benchmark runs.")
    parser.add_argument(
        "--impl",
        help="Implementation folder under .benchmarks (e.g., Darwin-CPython-3.12-64bit)",
        default=None,
    )
    parser.add_argument(
        "--n",
        type=int,
        default=2,
        help="Number of latest runs to include (2 to compare; 1 to show latest)",
    )
    parser.add_argument(
        "--latest",
        type=int,
        dest="n",
        help="Alias for --n (number of latest runs)",
    )
    parser.add_argument(
        "--metric",
        choices=["mean", "median", "ops"],
        default="mean",
        help="Primary metric to highlight in output",
    )
    parser.add_argument(
        "--group",
        type=str,
        help="Filter benchmarks by name substring (group)",
    )
    parser.add_argument(
        "--names",
        nargs="+",
        help="Filter by exact benchmark names (space-separated)",
    )
    parser.add_argument(
        "--output",
        choices=["text", "csv", "json"],
        default="text",
        help="Output format for the report",
    )
    parser.add_argument(
        "--regex",
        type=str,
        help="Regex to filter benchmark names",
    )
    parser.add_argument(
        "--save",
        type=str,
        help="Optional path to save CSV/JSON output to file",
    )
    args = parser.parse_args()

    runs = _find_runs()
    if args.impl:
        runs = [p for p in runs if args.impl in str(p)]
    else:
        # Auto-detect latest implementation folder by most recent JSON
        if runs:
            latest_run = runs[-1]
            # Implementation folder is the parent of the JSON
            impl_dir = latest_run.parent
            runs = [p for p in runs if impl_dir in p.parents or p.parent == impl_dir]
    if len(runs) == 0:
        print("No benchmark runs found. Run `pytest -q` first.")
        return 1
    if args.n <= 1 or len(runs) == 1:
        latest = runs[-1]
        latest_data = _load(latest)
        latest_entries = latest_data.get("benchmarks", [])
        latest_map = _extract(latest_entries)
        if args.group:
            latest_map = {k: v for k, v in latest_map.items() if args.group in k}
        if args.regex:
            pattern = re.compile(args.regex)
            latest_map = {k: v for k, v in latest_map.items() if pattern.search(k)}
        if args.names:
            latest_map = {k: v for k, v in latest_map.items() if k in args.names}
        if not latest_map:
            print("No benchmarks matched the provided filters.")
            return 0

        def emit_text():
            print(f"Showing latest benchmark run: {latest}")
            print("Name                                  mean        median      ops        rounds     iterations")
            print("-----------------------------------------------------------------------------------------------")
            for name in sorted(latest_map.keys()):
                bench = latest_map[name]
                print(
                    f"{name:35s} "
                    f"{bench['mean']:>10.4f} {'':>10s} "
                    f"{bench['median']:>10.4f} {'':>10s} "
                    f"{bench['ops']:>10.2f} {'':>10s} "
                    f"{int(bench['rounds']):>8d}        {int(bench['iterations']):>10d}"
                )

        if args.output == "csv":
            writer = csv.writer(sys.stdout)
            writer.writerow(["name", "mean", "median", "ops", "rounds", "iterations"])
            for name in sorted(latest_map.keys()):
                bench = latest_map[name]
                writer.writerow(
                    [
                        name,
                        bench["mean"],
                        bench["median"],
                        bench["ops"],
                        int(bench["rounds"]),
                        int(bench["iterations"]),
                    ]
                )
        elif args.output == "json":
            print(json.dumps({"run": str(latest), "benchmarks": latest_map}, indent=2))
        else:
            emit_text()
        # Optionally save output to file for csv/json
        if args.save and args.output in {"csv", "json"}:
            try:
                out_path = Path(args.save)
                if args.output == "csv":
                    with out_path.open("w", newline="") as f:
                        writer = csv.writer(f)
                        writer.writerow(["name", "mean", "median", "ops", "rounds", "iterations"])
                        for name in sorted(latest_map.keys()):
                            bench = latest_map[name]
                            writer.writerow(
                                [
                                    name,
                                    bench["mean"],
                                    bench["median"],
                                    bench["ops"],
                                    int(bench["rounds"]),
                                    int(bench["iterations"]),
                                ]
                            )
                else:
                    with out_path.open("w") as f:
                        json.dump({"run": str(latest), "benchmarks": latest_map}, f, indent=2)
                print(f"Saved {args.output} output to {out_path}")
            except Exception as exc:
                print(f"Failed to save output to {args.save}: {exc}")
        return 0

    latest = runs[-1]
    previous = runs[-2]

    latest_data = _load(latest)
    prev_data = _load(previous)

    latest_entries = latest_data.get("benchmarks", [])
    prev_entries = prev_data.get("benchmarks", [])

    latest_map = _extract(latest_entries)
    if args.names:
        latest_map = {k: v for k, v in latest_map.items() if k in args.names}
    prev_map = _extract(prev_entries)
    if args.names:
        prev_map = {k: v for k, v in prev_map.items() if k in args.names}

    names = sorted(set(latest_map.keys()) | set(prev_map.keys()))
    if args.group:
        names = [n for n in names if args.group in n]
    if args.regex:
        pattern = re.compile(args.regex)
        names = [n for n in names if pattern.search(n)]
    if args.names:
        names = [n for n in names if n in args.names]
    if not names:
        print("No benchmarks matched the provided filters.")
        return 0

    def emit_text():
        print(f"Comparing benchmarks:\n  latest:  {latest}\n  previous:{previous}\n")
        print("Name                                  mean        median      ops        rounds     iterations")
        print("-----------------------------------------------------------------------------------------------")
        for name in names:
            latest_bench = latest_map.get(name)
            prev_bench = prev_map.get(name)
            if not latest_bench or not prev_bench:
                state = "added" if latest_bench and not prev_bench else "removed"
                print(f"{name:35s} {state}")
                continue
            mean_delta = _fmt_delta(latest_bench["mean"], prev_bench["mean"])
            med_delta = _fmt_delta(latest_bench["median"], prev_bench["median"])
            ops_delta = _fmt_delta(latest_bench["ops"], prev_bench["ops"])

            def star(col: str) -> str:
                return "*" if args.metric == col else ""

            print(
                f"{name:35s} "
                f"{latest_bench['mean']:>10.4f}{star('mean')} ({mean_delta:>8s}) "
                f"{latest_bench['median']:>10.4f}{star('median')} ({med_delta:>8s}) "
                f"{latest_bench['ops']:>10.2f}{star('ops')} ({ops_delta:>8s}) "
                f"{int(latest_bench['rounds']):>8d}        {int(latest_bench['iterations']):>10d}"
            )

    if args.output == "csv":
        writer = csv.writer(sys.stdout)
        writer.writerow(
            [
                "name",
                "mean",
                "mean_delta",
                "median",
                "median_delta",
                "ops",
                "ops_delta",
                "rounds",
                "iterations",
            ]
        )
        for name in names:
            latest_bench = latest_map.get(name)
            prev_bench = prev_map.get(name)
            if not latest_bench or not prev_bench:
                continue
            writer.writerow(
                [
                    name,
                    latest_bench["mean"],
                    _fmt_delta(latest_bench["mean"], prev_bench["mean"]),
                    latest_bench["median"],
                    _fmt_delta(latest_bench["median"], prev_bench["median"]),
                    latest_bench["ops"],
                    _fmt_delta(latest_bench["ops"], prev_bench["ops"]),
                    int(latest_bench["rounds"]),
                    int(latest_bench["iterations"]),
                ]
            )
    elif args.output == "json":
        print(
            json.dumps(
                {
                    "latest": str(latest),
                    "previous": str(previous),
                    "benchmarks": {
                        name: {"latest": latest_map.get(name), "previous": prev_map.get(name)} for name in names
                    },
                },
                indent=2,
            )
        )
    else:
        emit_text()
    # Optionally save output to file for csv/json
    if args.save and args.output in {"csv", "json"}:
        try:
            out_path = Path(args.save)
            if args.output == "csv":
                with out_path.open("w", newline="") as f:
                    writer = csv.writer(f)
                    writer.writerow(
                        [
                            "name",
                            "mean",
                            "mean_delta",
                            "median",
                            "median_delta",
                            "ops",
                            "ops_delta",
                            "rounds",
                            "iterations",
                        ]
                    )
                    for name in names:
                        latest_bench = latest_map.get(name)
                        prev_bench = prev_map.get(name)
                        if not latest_bench or not prev_bench:
                            continue
                        writer.writerow(
                            [
                                name,
                                latest_bench["mean"],
                                _fmt_delta(latest_bench["mean"], prev_bench["mean"]),
                                latest_bench["median"],
                                _fmt_delta(latest_bench["median"], prev_bench["median"]),
                                latest_bench["ops"],
                                _fmt_delta(latest_bench["ops"], prev_bench["ops"]),
                                int(latest_bench["rounds"]),
                                int(latest_bench["iterations"]),
                            ]
                        )
            else:
                with out_path.open("w") as f:
                    json.dump(
                        {
                            "latest": str(latest),
                            "previous": str(previous),
                            "benchmarks": {
                                name: {
                                    "latest": latest_map.get(name),
                                    "previous": prev_map.get(name),
                                }
                                for name in names
                            },
                        },
                        f,
                        indent=2,
                    )
            print(f"Saved {args.output} output to {out_path}")
        except Exception as exc:
            print(f"Failed to save output to {args.save}: {exc}")

    return 0


if __name__ == "__main__":
    sys.exit(compare())
