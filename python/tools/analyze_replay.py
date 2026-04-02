#!/usr/bin/env python3

from __future__ import annotations

import json
import sys
from pathlib import Path


def load_replay(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: analyze_replay.py <replay.json>", file=sys.stderr)
        return 1

    path = Path(sys.argv[1])
    replay = load_replay(path)
    steps = replay.get("steps", [])

    if not steps:
        print("replay contains no steps", file=sys.stderr)
        return 1

    ticks = [step["snapshot"]["tick"] for step in steps]
    checksums = [step["checksum"] for step in steps]
    monotonic = all(current > previous for previous, current in zip(ticks, ticks[1:]))
    unique_checksums = len(set(checksums))
    final_snapshot = replay["final_snapshot"]

    print(f"file: {path}")
    print(f"seed: {replay['seed']}")
    print(f"steps: {len(steps)}")
    print(f"tick_monotonic: {str(monotonic).lower()}")
    print(f"unique_step_checksums: {unique_checksums}")
    print(f"final_checksum: {replay['final_checksum']}")
    print(
        "final_snapshot:"
        f" tick={final_snapshot['tick']}"
        f" phase={final_snapshot['phase']}"
        f" position={final_snapshot['position']}"
        f" velocity={final_snapshot['velocity']}"
        f" energy={final_snapshot['energy']}"
    )

    if not monotonic:
        print("error: replay ticks are not strictly increasing", file=sys.stderr)
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
