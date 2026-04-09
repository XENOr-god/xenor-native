# xenor-native Architecture

## Repository Purpose

`xenor-native` is the experimental native lab for XENOr's deterministic
simulation model. The repository exists to keep low-level runtime work close to
the metal while it is still being proven, without turning unfinished native
work into the canonical substrate:

- deterministic simulation
- explicit tick and phase execution
- replay validation
- seed and input handling
- snapshot and checksum verification

This repository complements XENOr's existing codebases. It does not redefine
the broader project, and it is not the canonical deterministic substrate.

It is also a narrow native boundary that other XENOr repositories can depend on
without inheriting the full implementation directly. `xenor-site` consumes it
as a pinned Git submodule so CI and documentation can reference a specific
experimental native revision. Mature substrate work should graduate into
`xenor-engine`.

## Primary Boundaries

### Rust Host Layer

Rust is the repository's host and control layer.

Responsibilities:

- expose a safe API over the native kernel
- own replay playback and orchestration
- provide the CLI entrypoint
- hold deterministic integration tests
- emit replay and benchmark artifacts for offline analysis

Rust does not own the simulation math itself. It owns safety, coordination, and
tooling.

### C++ Simulation Kernel

C++ owns the deterministic kernel because this layer is about explicit control
over layout, arithmetic, and low-level stepping.

Responsibilities:

- maintain the authoritative simulation state
- execute phase transitions in a fixed order
- extract snapshots from native state
- compute state checksums inside the native boundary

The C++ layer does not expose classes across FFI. The native boundary is a C
ABI with opaque handles and explicit structs.

### Thin C ABI

The ABI exists to keep Rust and C++ aligned without leaking C++ semantics:

- `XenorSeed`
- `XenorInputFrame`
- `XenorStateSnapshot`
- `XenorChecksum`
- `XenorSimulation*`

This makes the boundary auditable, portable, and stable enough for future
bindings.

## Module Boundaries

- `crates/xenor-native`: safe Rust wrapper over the native C ABI, replay
  orchestration, runtime control, snapshot and checksum types
- `crates/xenor-cli`: deterministic CLI entrypoint for running seed/input
  sequences and emitting replay or benchmark artifacts
- `cpp/include` + `cpp/src`: native simulation kernel, explicit phase logic,
  snapshot assembly, checksum implementation
- `tests`: deterministic integration coverage that asserts same-input and
  changed-input behavior
- `python/tools`: offline replay inspection and benchmark summarization
- `zig`, `mlang`, `asm`: secondary experiments that remain outside the
  authoritative runtime path

## Optional Languages

The optional languages are intentionally isolated from the default build and
runtime path.

### Python

Python is tooling only. It inspects replay output and summarizes benchmark
results. It does not participate in simulation execution.

### Zig

Zig holds a fixed arena allocator sketch for deterministic scratch memory. This
is relevant to simulation infrastructure, but it is not central enough to own
the runtime.

### Haskell

Haskell provides an offline invariant checker for snapshot streams. It is useful
for expressing validation rules cleanly without pulling a second paradigm into
the main runtime.

### Assembly

Assembly is limited to isolated checksum or hash micro-kernels. It is
documented, optional, and intentionally disconnected from the default build so
the repository does not become architecture-fragile.

## Data Flow

The native data flow is intentionally narrow:

`seed -> input frames -> tick/phase execution -> snapshot -> checksum`

Expanded:

1. Rust constructs a `SimulationSeed`.
2. Rust feeds ordered `InputFrame` values into the native kernel.
3. C++ executes one deterministic tick with explicit phases:
   - input phase
   - simulation phase
   - finalize phase
4. Rust requests a `StateSnapshot` at a deterministic boundary.
5. C++ computes a `Checksum` from the native state.
6. Rust stores replay output, exposes the terminal snapshot/checksum, and
   validates determinism in tests.

This flow matters more than the individual implementation language. The
repository should continue to optimize around this path.

## Why Rust + C++ Is Primary

Rust and C++ are primary because they solve the central problem cleanly:

- C++ handles the simulation kernel with exact control over layout and stepping
- Rust provides memory safety, test ergonomics, CLI ergonomics, and replay
  orchestration
- the C ABI keeps the seam explicit instead of magical

Any future additions should strengthen this pairing rather than dilute it.

## Integration Points

The repository is designed to plug into the rest of Xenor without becoming
entangled with unrelated application logic:

- `xenor-web` can pin a reviewed native revision as a submodule and run native
  smoke checks in CI
- other Xenor repositories can call the Rust library for replay execution,
  snapshot capture, and checksum verification
- tooling layers can consume emitted replay JSON or benchmark CSV for analysis
- future bindings should stay behind the C ABI or the safe Rust API rather than
  reaching into C++ internals directly

## Extension Guidance

Future work should preserve Xenor identity:

- add phases only when they strengthen deterministic stepping or replay clarity
- extend snapshots with fields that materially improve validation or debugging
- keep replay formats inspectable and stable
- avoid introducing floating-point behavior into authoritative state evolution
- keep optional languages secondary unless they become indispensable to
  determinism or verification

If a feature does not help deterministic execution, replay, or validation, it
likely does not belong here.
