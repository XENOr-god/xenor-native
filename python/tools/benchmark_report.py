#!/usr/bin/env python3

from __future__ import annotations

import csv
import statistics
import sys
from pathlib import Path


def load_rows(path: Path) -> list[dict[str, str]]:
    with path.open("r", encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle))


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: benchmark_report.py <benchmark.csv>", file=sys.stderr)
        return 1

    path = Path(sys.argv[1])
    rows = load_rows(path)
    if not rows:
        print("benchmark file is empty", file=sys.stderr)
        return 1

    durations = [int(row["duration_ns"]) for row in rows]
    checksums = {row["checksum"].strip() for row in rows}

    print(f"file: {path}")
    print(f"samples: {len(rows)}")
    print(f"checksum_stable: {str(len(checksums) == 1).lower()}")
    print(f"min_ns: {min(durations)}")
    print(f"max_ns: {max(durations)}")
    print(f"mean_ns: {int(statistics.fmean(durations))}")
    print(f"median_ns: {int(statistics.median(durations))}")
    if len(durations) >= 2:
        print(f"stdev_ns: {int(statistics.pstdev(durations))}")
    else:
        print("stdev_ns: 0")

    return 0 if len(checksums) == 1 else 2


if __name__ == "__main__":
    raise SystemExit(main())
