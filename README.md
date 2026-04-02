# xenor-native

`xenor-native` is a native simulation backend and laboratory for the Xenor
ecosystem. It does not replace the existing Xenor repositories. It provides a
focused environment for deterministic stepping, replay validation, seed/input
handling, snapshot extraction, and checksum verification close to the machine.

## Relationship To Xenor

Xenor already centers deterministic execution and inspectable simulation. This
repository extends that identity with a native-first stack:

- Rust hosts orchestration, replay control, tests, and the CLI surface.
- C++ owns the deterministic simulation kernel and core state transitions.
- A thin C ABI keeps the boundary stable and auditable.
- Python, Zig, Haskell, and assembly remain isolated sidecar modules for
  analysis, allocators, invariant checks, and micro-kernel experiments.

The point is not language variety. The point is disciplined boundaries around
deterministic simulation work.

## Core Capabilities

- seed-driven runtime initialization
- explicit tick and phase execution
- replay playback from recorded input frames
- state snapshot extraction at deterministic boundaries
- checksum generation for replay validation
- deterministic regression tests for same-input/same-output guarantees

## Repository Layout

```text
xenor-native/
  README.md
  ARCHITECTURE.md
  Cargo.toml
  build.rs
  crates/
    xenor-native/
      src/
        lib.rs
        ffi.rs
        runtime.rs
        replay.rs
        snapshot.rs
        checksum.rs
    xenor-cli/
      src/
        main.rs
  cpp/
    include/
      xenor_sim.h
      xenor_types.h
    src/
      xenor_internal.hpp
      xenor_sim.cpp
      xenor_state.cpp
      xenor_math.cpp
      xenor_checksum.cpp
    CMakeLists.txt
  python/
    tools/
      analyze_replay.py
      benchmark_report.py
  zig/
    alloc/
      arena_alloc.zig
  mlang/
    haskell/
      InvariantCheck.hs
  asm/
    kernels/
      tick_hash.S
  tests/
    determinism.rs
    replay.rs
```

## Build, Run, Test

Rust is the primary entrypoint and builds the C++ kernel through Cargo.

```bash
cargo build
cargo run --bin xenor-cli -- --seed 17 --snapshot
cargo test
```

The native library can also be built directly with CMake when isolating the C++
layer:

```bash
cmake -S cpp -B cpp/build
cmake --build cpp/build
```

## CLI Workflow

The CLI runs a deterministic replay from either an embedded sample or a tiny
comma-separated input file:

```text
throttle,steer,action_mask
14,1,0
22,-2,1
30,0,0
```

Example commands:

```bash
cargo run --bin xenor-cli -- --seed 17 --snapshot
cargo run --bin xenor-cli -- --seed 17 --input-file replay.txt --emit-replay target/replay.json
cargo run --bin xenor-cli -- --seed 17 --repeat 100 --benchmark-out target/bench.csv
```

## Determinism Philosophy

The kernel uses integer-only state transitions and explicit phase ordering:

`seed -> input frame -> input phase -> simulation phase -> finalize phase -> snapshot -> checksum`

Replay validation depends on two invariants:

- the same seed and the same input sequence must yield the same final snapshot
  and checksum
- changing the input sequence must change the terminal state or checksum

The test suite enforces both cases.

## Replay And Checksum Tooling

- `xenor-cli --emit-replay` writes a JSON replay report containing frames,
  snapshots, and checksums
- `python/tools/analyze_replay.py` inspects a replay file and highlights
  monotonic tick flow and checksum evolution
- `python/tools/benchmark_report.py` summarizes benchmark CSV output and checks
  checksum stability across repeated runs

## Optional Sidecar Modules

The optional modules are present because they support Xenor-native concerns, not
because they are fashionable:

- Zig: fixed arena allocator sketch for deterministic scratch storage
- Haskell: offline invariant checker for snapshot streams
- Assembly: isolated hash-mix experiment for checksum micro-kernels

They are intentionally outside the default build so the Rust/C++ core stays
primary and coherent.
