# Per-project regions for the lint helper (2026-10-25)

The helper lints a request's tsconfig projects one after another. It used to keep every program's arena memory
until the request ended:

- the loader's `ProgramRegions` set, shared by all programs;
- every lint worker's region;
- the main thread's request region.

`ProgramOwner::drop` freed each `Program`, but the files parsed and bound for it and its checkers' types and links
stayed allocated. Peak memory was the sum over the projects, and with compressed pointers all of it counts against
the 32 GiB arena reservation.

Each project now gets `ProjectRegions`, freed after its program:

- its lint workers' regions;
- a main-thread region entered for the project;
- a fresh `ProgramRegions` set.

The request region keeps only project discovery. Drop order: the program, then the workers, the main region, and the
loaders last, as in the watcher's `install_program`. Nothing that outlives a project refers into its regions:

- the payload, timings, writer, config host and `cachedvfs` caches are on the heap;
- process-wide statics allocate in the thread arena;
- every project builds an unshared program.

This revisits the request-wide loader set from [parallel program construction](perf-tsgolint-parallel-load.md). Its
reason, avoiding an arena tail per worker per tsconfig, only holds while the tails are retained.

## Measurements

Linux x86-64, 16 cores, release build (fat LTO), the helper alone (`tsrs-lint headless < payload.json`). Three
alternating samples, medians. Generated workload: N projects of 24 files each, `lib: ["es2022", "dom",
"dom.iterable"]` and 12 type-aware rules. The generator is in the issue. One finding per file; frames are
byte-identical before and after.

| Projects | Peak RSS before | Peak RSS after | Wall before | Wall after |
| ---: | ---: | ---: | ---: | ---: |
| 40 | 740 MiB | 147 MiB | 2.26 s | 1.94 s |
| 120 | 2,121 MiB | 197 MiB | 6.01 s | 5.50 s |

On a large private monorepo, measured the same way on oxlint's captured request:

- helper peak RSS fell from 18.5 GiB to 1.18 GiB;
- wall time was unchanged within noise (77.6 s before, 76.2 s after);
- output was byte-identical.

Through oxlint with the repository's full configuration, the process-tree peak went from 3.0x stock
oxlint-tsgolint's to 0.57x, with identical findings. Worker counts, scheduling and checker reuse are unchanged.

## Validation

- `cargo test --locked -p tsrs_lint`, including the new lifecycle test: one request, six projects, serial and
  parallel paths.
- The same tests with `TSRS_ARENA_POISON=1`.
- The private-monorepo request run in poison mode: byte-identical output.
- `cargo clippy -p tsrs_lint --all-targets --config 'build.warnings="deny"'` and `tools/lint/source.py`.
