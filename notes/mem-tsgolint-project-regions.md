# Per-project regions for the lint helper (2026-10-30)

The helper lints a request's tsconfig projects one after another. On `main` it keeps every program's arena memory
until the request ends:

- the loader's `ProgramRegions` set, shared by all programs;
- every lint worker's region;
- the main thread's request region.

`ProgramOwner::drop` frees each `Program`, but the files parsed and bound for it and its checkers' types and links
stay allocated. Peak memory is the sum over the projects, and with compressed pointers all of it counts against the
32 GiB arena reservation. oxlint sends one request for a whole workspace, so a monorepo with many tsconfig projects
pays for all of them at once.

Each project now gets `ProjectRegions`, freed after its program:

- its lint workers' regions;
- a main-thread region entered for the project;
- a fresh `ProgramRegions` set.

The request region keeps only project discovery. Drop order: the program, then the workers, the main region, and the
loaders last, as in the watcher's `install_program`. Nothing that outlives a project refers into its regions:

- the payload, timings, writer, config host and `cachedvfs` caches are on the heap;
- what escapes the scratch regions (`arena::escape_scratch`: program caches, diagnostics, lazily filled shared data)
  goes to the thread arena, because the request region is the outermost scratch region;
- every project builds an unshared program.

This revisits the request-wide loader set from [parallel program construction](perf-tsgolint-parallel-load.md). Its
reason, avoiding an arena tail per worker per tsconfig, only holds while the tails are retained; freed with their
project, they do not add up.

## Measurements

Linux x86-64, 16 vCPUs (KVM), a shared machine under load. Both builds use the release profile with
`CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16`, the helper alone
(`tsrs-lint headless < payload.json`). Generated workload: N independent projects of 24 files each, `lib: ["es2022",
"dom", "dom.iterable"]` and 12 type-aware rules, one `no-floating-promises` finding per file (the generator is in
#783). Peak RSS is the median of three runs, `main` and this change interleaved.

| Projects | `main` (`ce0b047`) | #848 (`765f97d`) | This change | Frames |
| ---: | ---: | ---: | ---: | --- |
| 40 | 634 MiB | 458 MiB | 157 MiB | byte-identical |
| 120 | 1,812 MiB | 1,304 MiB | 218 MiB | byte-identical |

Instructions retired, single-threaded (`RAYON_NUM_THREADS=1`, `perf stat -e instructions:u`), 120 projects, three
runs each: 21.157 G on `main`, 21.069 G with this change (-0.4%), spread under 0.01%.

On a large private monorepo, through oxlint with its full configuration, this change and the same patch on the
PR 624 head of 2026-10-25 report the same type-aware findings, except the tuple reports added by `e720cf07e`
(`require-array-sort-compare`), with the same peak RSS. Measured on the 2026-10-25 head, the helper's peak on that
monorepo fell from 18.5 GiB to 1.18 GiB with byte-identical output.

## Validation

- `cargo test --locked --profile ci -p tsrs_lint`, including the new lifecycle test: one request, six projects,
  serial and parallel paths.
- The same tests with `TSRS_ARENA_POISON=1`.
- The 40- and 120-project requests with `TSRS_ARENA_POISON=1`: byte-identical frames.
- `cargo clippy --locked -p tsrs_lint --all-targets --config 'build.warnings="deny"'` and `tools/lint/source.mjs`.
