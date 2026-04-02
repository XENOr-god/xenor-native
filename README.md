# xenor-native

`xenor-native` is a native simulation backend and laboratory for the Xenor
ecosystem. It does not replace the existing Xenor repositories. It provides a
focused environment for deterministic stepping, replay validation, seed/input
handling, snapshot extraction, and checksum verification close to the machine.

## Role In The Xenor Ecosystem

Xenor already centers deterministic execution and inspectable simulation. This
repository extends that identity with a native-first stack:

- Rust hosts orchestration, replay control, tests, and the CLI/runtime surface.
- C++ owns the deterministic simulation kernel and core state transitions.
- A thin C ABI keeps the boundary stable and auditable.
- Python, Zig, Haskell, and assembly remain isolated sidecar modules for
  analysis, allocators, invariant checks, and micro-kernel experiments.

The point is not language variety. The point is disciplined boundaries around
deterministic simulation work.

`xenor-native` is intended to be consumed by other Xenor repositories rather
than folded into them:

- `xenor-web` vendors this repository as a Git submodule so the public surface
  can pin an explicit native revision and validate that the native workspace is
  present and healthy in CI.
- future Xenor execution or simulation repositories can bind to the Rust API,
  invoke the CLI for replay checks, or treat the C ABI as a stable native seam.

## Language Responsibilities

- Rust: safe host layer, orchestration, replay/runtime control, CLI entrypoint,
  and deterministic test harness
- C++: deterministic simulation kernel, phase execution, snapshot extraction,
  and checksum generation behind a C ABI
- Python: replay inspection and benchmark reporting only
- Zig, Haskell, Assembly: isolated secondary modules for allocator experiments,
  invariant validation, and checksum micro-kernel research

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

If you want one command surface for the standard developer loop:

```bash
make build
make test
make native-build
make smoke
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

## Replay, Snapshot, And Checksum Flow

The kernel uses integer-only state transitions and explicit phase ordering:

`seed -> input frame -> input phase -> simulation phase -> finalize phase -> snapshot -> checksum`

Replay validation depends on two invariants:

- the same seed and the same input sequence must yield the same final snapshot
  and checksum
- changing the input sequence must change the terminal state or checksum

The test suite enforces both cases.

## Tooling

- `xenor-cli --emit-replay` writes a JSON replay report containing frames,
  snapshots, and checksums
- `python/tools/analyze_replay.py` inspects a replay file and highlights
  monotonic tick flow and checksum evolution
- `python/tools/benchmark_report.py` summarizes benchmark CSV output and checks
  checksum stability across repeated runs

## CI Expectations

GitHub Actions validates the repository with an explicit native-oriented path:

- `cargo test`
- direct CMake build of the C++ kernel
- CLI smoke execution that emits replay and benchmark artifacts
- Python tooling smoke checks over those generated artifacts

## Downstream Sync Automation

After the validation workflow succeeds for a push to `main`, `xenor-native`
dispatches a downstream sync request to `xenor-web`. The downstream repository
then attempts to update its `xenor-native` submodule pointer, reruns the
existing lightweight validation, and opens or refreshes an automation pull
request instead of pushing directly to `main`.

This dispatch requires the `XENOR_SYNC_TOKEN` repository secret in
`xenor-native`.

- target repository: `XENOr-god/xenor-site`
- trigger mechanism: `repository_dispatch`
- required secret name: `XENOR_SYNC_TOKEN`
- classic PAT minimum scope: `repo`
- fine-grained PAT minimum access: repository access to `XENOr-god/xenor-site`
  with `Contents: write`

If the organization requires approval for fine-grained tokens, approve the
token for the target repository before relying on automatic sync.

## Optional Sidecar Modules

The optional modules are present because they support Xenor-native concerns, not
because they are fashionable:

- Zig: fixed arena allocator sketch for deterministic scratch storage
- Haskell: offline invariant checker for snapshot streams
- Assembly: isolated hash-mix experiment for checksum micro-kernels

They are intentionally outside the default build so the Rust/C++ core stays
primary and coherent.

## Consumption By Other Xenor Repositories

Expected integration points are deliberately narrow:

- pin the repository as a submodule when another Xenor repository needs a
  reviewed native revision
- call the Rust library for replay execution and snapshot/checksum retrieval
- invoke `xenor-cli` in CI for deterministic smoke checks
- consume generated replay artifacts from Python tooling for offline inspection

This repository is the native validation surface. Other repositories should use
it to strengthen determinism and integration discipline, not to duplicate the
kernel internally.
