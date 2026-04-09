# xenor-native

`xenor-native` is the experimental native lab for the XENOr stack. It is a
bounded incubation repository for low-level native execution work. It is not a
co-equal public core layer, and it is not the canonical deterministic
substrate.

## Status

Active experimental repository. Useful for native research, verification, and
benchmarking work, but not the canonical substrate that the public stack should
treat as stable.

## Role in XENOr

`xenor-native` exists to keep low-level native work contained while it is still
being proven.

This repository is where XENOr can explore:

- native execution kernels
- replay and checksum experiments
- ABI boundaries
- verification harnesses
- benchmarking paths
- parity and low-level instrumentation work

The point is not language variety. The point is to keep unfinished native work
out of the canonical substrate until it is ready.

## Relationship to xenor-engine

`xenor-engine` is the canonical deterministic substrate for the public XENOr
stack.

`xenor-native` sits upstream of that surface as an experimental native lab.
Work can graduate from `xenor-native` into `xenor-engine`, but `xenor-native`
itself must not be presented as the primary engine or canonical substrate.

The relationship is deliberately asymmetric:

- incubation can happen in `xenor-native`
- canonical substrate ownership stays in `xenor-engine`

## What belongs here

- experimental kernels and runtime paths
- replay, checksum, and snapshot verification experiments
- thin ABI seams and boundary validation
- parity probes and low-level determinism checks
- benchmark harnesses and measurement tooling
- isolated allocator, invariant, and micro-kernel research

## What does NOT belong here

- canonical substrate claims
- public stack ownership language
- token or deployment messaging
- primary newcomer documentation
- production-ready positioning for unfinished native work
- work that already belongs in `xenor-engine`

## Graduation criteria

Work should move from `xenor-native` into `xenor-engine` only when it meets all
of the following:

- deterministic behavior is repeatable and clearly specified
- the boundary is narrow enough to review and maintain
- replay, snapshot, checksum, or ABI semantics are explicit
- tests and benchmarks justify promotion
- the responsibility belongs in the canonical substrate rather than an open
  experiment loop

## Language responsibilities

- Rust: host layer, orchestration, replay/runtime control, CLI, and test
  harnesses
- C++: kernel experiments and low-level state transition paths behind the ABI
- Python: replay inspection and benchmark reporting
- Zig, Haskell, Assembly: optional sidecar research modules for allocator,
  invariant, and checksum experimentation

## Repository layout

```text
xenor-native/
  README.md
  ARCHITECTURE.md
  Cargo.toml
  build.rs
  crates/
    xenor-native/
    xenor-cli/
  cpp/
  python/
  zig/
  mlang/
  asm/
  tests/
```

## Build, run, test

Rust is the primary entrypoint and builds the C++ layer through Cargo.

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

## CLI workflow

The CLI runs deterministic native experiments from either an embedded sample or
a tiny comma-separated input file:

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

## CI expectations

GitHub Actions validates the repository as an experimental native lab:

- `cargo test`
- direct CMake build of the C++ layer
- CLI smoke execution that emits replay and benchmark artifacts
- Python tooling smoke checks over generated artifacts

These checks prove the lab is healthy. They do not turn the lab into the
canonical substrate.

## Consumption by other XENOr repositories

Expected integration points are narrow:

- `xenor-site` can pin a reviewed experimental revision as a Git submodule
- other repos can call the Rust library or CLI for bounded replay or checksum
  checks
- generated replay artifacts can be inspected offline through the Python tools

Use this repository to harden low-level ideas before promotion, not to blur repo
boundaries across the active stack.

## Optional sidecar modules

The optional modules remain isolated because they support native-lab research,
not because they are part of the public core stack:

- Zig: allocator experiments
- Haskell: invariant validation
- Assembly: checksum micro-kernel experiments

They stay outside the default build so the Rust/C++ path remains primary and
coherent.

## Downstream sync automation

After the validation workflow succeeds for a push to `main`, `xenor-native`
dispatches a downstream sync request to `xenor-site`. The downstream repository
can then review and update its `xenor-native` submodule pointer through a pull
request instead of pushing directly to `main`.

This keeps the experimental lab visible to the public surface without implying
that the lab is the canonical substrate.
