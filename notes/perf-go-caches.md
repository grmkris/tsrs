# Go-mode checker cache defaults (2026-10-25)

Enable the existing inference-walk memo, union-construction cache and indexed-access memo by default in Go-compatible
history. Previously that history disabled all three unless explicitly enabled. The change is confined to their
mode selection; keys, store guards, replay logic, table sizes and shadow checks are unchanged. Each retains its
`0`/`off`, `1`/`on` and `shadow` environment overrides.

Checker counts, assignment, file order, work stealing, leaf cleanup and recycling are unchanged. This uses the
already implemented caches described in [perf-heavy-files-infer-memo.md](perf-heavy-files-infer-memo.md) and
[perf-union-inference.md](perf-union-inference.md), not the broader generic-call cache rejected in
[perf-round2-followups.md](perf-round2-followups.md). The new evidence is the Go-mode measurement and shadow
validation below. Caching reduces repeated checker work; it does not eliminate the fixed-queue bottleneck on
every project or make Go mode universally as fast as the default scheduler.

## Safety boundary

These caches already run in default history. This change enables the same implementations and guards in Go history.
They are checker-local, compare exact keys, and retain type handles for the owning checker's lifetime.

- Union construction keys the ordered inputs, reduction mode and alias. It excludes origins, error results,
  instantiations, extra type creation, and the reductions that depend on changing resolution state.
- Indexed access keys the object, union index and access flags, for calls without an access node or alias.
  Its store rule also rejects diagnostic/complexity reports, instantiations, extra type creation and impure unions.
- Inference keys the source, target, priority, variance and initial inference candidates/state. It stores only
  eligible walks without new types/symbols/signatures, instantiations or diagnostics, rejecting transient/circular
  resolution dependencies and active-instantiation contexts. Hits replay the inference outcome and flow-memo flags.

Shadow mode recomputes hits and checks the answer; inference additionally checks its outcome and effects, allowing
the documented lazy-member symbol materialization. The validation below supports the guards, but finite coverage
does not prove correctness for every program. An untracked mutable-state dependency is the remaining correctness
risk. Lazy-member materialization can change internal Symbols counters even when diagnostics and baselines match.
All three caches keep independent opt-outs for diagnosing any future mismatch.

## Measurement setup

Base revision: `66119d706aada74470984b8c1c0595179060a2d9`. Apple M3 Max, macOS arm64, 14 available cores.
Release builds use fat LTO, full debug symbols, no stripping and no PGO. Catalog checkouts match their pinned
revisions. Every comparison uses the same hardware, workload, build configuration and checker count, with
`--noEmit --incremental false --pretty false` and `TSRS_CHECKER_ASSIGNMENT=go`. Single-threaded controls add
`--singleThreaded` and `RAYON_NUM_THREADS=1`. No builds or validation run alongside measurements.

Hyperfine 2 records wall time, retired instructions and each process's maximum resident set. Tables use run
medians; wall and RSS are measured at the normal automatic checker count, and single-threaded instruction
counts are shown separately. Each variant has a preflight warmup, and paired repetitions alternate binary order.
The two tables are separate experiments with different baselines; do not combine their absolute times.

Two of the 19 catalog entries cannot reach checking with the catalog flags: drizzle-orm reports TS5090/TS5102
for paths/baseUrl, and date-fns reports TS6379 for composite with incremental disabled. Their configuration-error
runs match across variants and are excluded from checker-performance aggregates.

## Inference memo

Baseline: unmodified base revision, all three memos default off in Go history. Candidate: inference memo enabled.
Three pairs per project/count; five normal-count projects (VS Code, MUI, Supabase, T3 and Next root) received
seven-pair confirmations, whose medians are used below. Single-threaded webpack, Storybook and TypeORM also
received seven-pair confirmations.

| Project | Wall s before / after | Wall change | Default instructions G before / after | Single instructions G before / after | Default peak RSS MiB before / after |
| --- | ---: | ---: | ---: | ---: | ---: |
| vscode | 1.433 / 1.180 | -17.6% | 106.39 / 99.54 | 88.21 / 81.71 | 2445 / 2446 |
| xstate-main | 0.162 / 0.164 | +0.8% | 8.13 / 8.12 | 6.20 / 6.19 | 247 / 247 |
| webpack | 0.241 / 0.236 | -2.1% | 15.86 / 15.84 | 10.95 / 10.93 | 488 / 488 |
| mui-docs | 1.219 / 1.221 | +0.1% | 96.70 / 96.68 | 38.87 / 38.84 | 1543 / 1538 |
| Compiler | 0.062 / 0.063 | +0.8% | 2.05 / 2.02 | 1.74 / 1.70 | 75 / 75 |
| Compiler-Unions | 0.097 / 0.097 | +0.1% | 3.65 / 3.60 | 3.00 / 2.94 | 79 / 79 |
| cal-diy | 1.094 / 1.092 | -0.2% | 72.06 / 71.25 | 32.73 / 32.43 | 1677 / 1675 |
| formbricks-web | 1.191 / 1.120 | -5.9% | 74.01 / 73.44 | 41.20 / 40.71 | 1958 / 1959 |
| supabase-studio | 1.107 / 1.079 | -2.5% | 92.58 / 90.51 | 46.97 / 46.35 | 1797 / 1798 |
| t3code-server | 1.468 / 1.340 | -8.7% | 118.71 / 99.29 | 45.31 / 40.22 | 1803 / 1804 |
| mikro-orm | 1.176 / 1.154 | -1.9% | 90.96 / 89.40 | 50.39 / 48.80 | 1935 / 1936 |
| next-packages-next | 0.489 / 0.471 | -3.7% | 22.90 / 22.74 | 17.56 / 17.42 | 632 / 633 |
| next-root | 0.416 / 0.405 | -2.6% | 20.53 / 20.53 | 16.74 / 16.72 | 500 / 500 |
| storybook | 0.270 / 0.268 | -0.6% | 18.56 / 18.45 | 10.19 / 10.14 | 496 / 496 |
| nuxt | 0.300 / 0.294 | -1.9% | 18.28 / 18.24 | 10.85 / 10.83 | 482 / 483 |
| playwright | 0.209 / 0.205 | -1.8% | 13.70 / 13.62 | 7.69 / 7.62 | 447 / 447 |
| typeorm | 0.202 / 0.199 | -1.3% | 9.20 / 9.18 | 7.85 / 7.83 | 254 / 253 |

VS Code and T3 are faster in all seven confirmation pairs: -17.6% and -8.7% wall, respectively. Their normal-count
instructions fall 6.4% and 16.4%; single-threaded instructions fall 7.4% and 11.2%. Instructions decrease on all
17 checked projects in both count configurations. Normal-count peak RSS changes range from -0.4% to +0.2%.

Wall time is not lower everywhere. MUI's initial apparent 10.1% improvement disappeared in confirmation (+0.1%).
Single-threaded webpack and TypeORM retain small wall regressions of 1.0% (0.899 to 0.908 s) and 1.3%
(0.658 to 0.667 s), despite fewer instructions. Small wall differences and three-pair results remain noise-sensitive.

## Union construction and indexed access

Baseline: the inference-only candidate above. Candidate: all three caches enabled. Seven pairs per project at
normal checker counts and three single-threaded pairs; XState single-threaded received a seven-pair confirmation,
used below. These measurements isolate the union and indexed-access caches together, not each independently.

| Project | Wall s before / after | Wall change | Default instructions G before / after | Single instructions G before / after | Default peak RSS MiB before / after | Peak RSS change |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| vscode | 1.357 / 1.335 | -1.6% | 99.43 / 98.40 | 81.72 / 80.86 | 2448 / 2453 | +0.2% |
| xstate-main | 0.152 / 0.151 | -0.3% | 8.11 / 8.02 | 6.19 / 6.11 | 247 / 248 | +0.3% |
| webpack | 0.232 / 0.227 | -2.1% | 15.85 / 15.70 | 10.93 / 10.80 | 489 / 491 | +0.4% |
| mui-docs | 1.349 / 1.331 | -1.3% | 96.67 / 95.51 | 38.83 / 38.10 | 1539 / 1549 | +0.6% |
| Compiler | 0.064 / 0.063 | -2.4% | 2.02 / 2.01 | 1.70 / 1.69 | 75 / 76 | +0.4% |
| Compiler-Unions | 0.095 / 0.095 | +0.3% | 3.60 / 3.55 | 2.94 / 2.89 | 79 / 81 | +1.6% |
| cal-diy | 0.976 / 0.955 | -2.2% | 71.25 / 70.19 | 32.43 / 31.92 | 1677 / 1682 | +0.3% |
| formbricks-web | 1.007 / 1.014 | +0.7% | 73.42 / 72.78 | 40.71 / 40.33 | 1954 / 1963 | +0.5% |
| supabase-studio | 1.174 / 1.128 | -4.0% | 90.50 / 87.83 | 46.35 / 45.43 | 1798 / 1804 | +0.4% |
| t3code-server | 1.390 / 1.384 | -0.4% | 99.23 / 98.72 | 40.20 / 39.89 | 1802 / 1813 | +0.6% |
| mikro-orm | 1.084 / 1.068 | -1.5% | 89.40 / 87.70 | 48.80 / 47.32 | 1936 / 1939 | +0.2% |
| next-packages-next | 0.473 / 0.472 | -0.2% | 22.74 / 22.34 | 17.42 / 17.06 | 633 / 634 | +0.0% |
| next-root | 0.411 / 0.407 | -0.9% | 20.55 / 20.48 | 16.72 / 16.66 | 499 / 501 | +0.4% |
| storybook | 0.254 / 0.251 | -1.2% | 18.45 / 18.15 | 10.15 / 9.99 | 496 / 497 | +0.3% |
| nuxt | 0.285 / 0.274 | -4.1% | 18.25 / 17.91 | 10.83 / 10.70 | 484 / 486 | +0.3% |
| playwright | 0.195 / 0.190 | -2.5% | 13.63 / 13.48 | 7.62 / 7.53 | 447 / 449 | +0.5% |
| typeorm | 0.188 / 0.187 | -0.5% | 9.17 / 9.10 | 7.83 / 7.76 | 254 / 254 | +0.0% |

Median per-project normal-count changes: -1.30% wall, -1.08% instructions and +0.35% peak RSS. Instructions
fall on all 17 projects; wall medians fall on 15/17. Compiler-Unions (+0.28%) and Formbricks (+0.73%) have small
wall regressions. The largest wall improvements are Nuxt (-4.1%) and Supabase (-4.0%).

RSS grows 0.02–1.61% across the catalog: up to 1.28 MiB on Compiler-Unions in relative terms and 10.58 MiB on T3
in absolute terms. This is a CPU/memory tradeoff. Single-threaded median changes are -0.75% wall, -1.27%
instructions and +0.15% RSS; small wall regressions remain on cal-diy (+0.06%), T3 (+0.84%) and Next root
(+0.68%). XState's initial three-pair instruction regression did not repeat: every confirmed pair is lower,
with a -1.3% median. Confirmed instruction medians are lower on all 17 projects in both count configurations.

Catalog diagnostics and exit codes match. Types and Instantiations counters match in normal-count preflights;
Symbols counts differ slightly on several projects, so the change does not claim identical internal allocation
statistics. These numbers do not establish the same gains in shipping PGO/BOLT builds.

## Validation

Used the clean alternate reference checkout `target/release-prep-reference`, verified at Cargo's pin
`3e9236361fa97a9e6c685f4833ac457ae2d8c50a`. The root `ts-ref` checkout was at a different revision and was not
used. Dev-profile CLI and runner builds use the verified checkout's testdata through `TSRS_TESTDATA`.

The final all-cache candidate completed Go history, Go with `TSRS_LAZY_MEMBERS=0`, canonical multithreaded
history, and combined inference/union/indexed-access shadow mode (including `TSRS_INFER_MEMO_MIN_STEPS=1`).
Each selected 15,448 variants and matched all supported baselines:

| Baseline kind | Passing outcomes per mode |
| --- | ---: |
| Errors | 13,700 |
| Types | 12,998 |
| Symbols | 12,998 |
| JavaScript/declaration emit | 13,626 |
| JavaScript maps | 149 |
| Source-map records | 156 |

No differences, crashes or timeouts in those final runs. All 36 before/after outcome lists are byte-identical
for both Go configurations. In the earlier inference-only comparison, four runs initially timed out on
`intersectionConstructorReductionCrash` at 120 seconds; the case passed with a 600-second limit in every mode,
and those outcomes were merged into the full results. Validation timings are not performance measurements.

All 50 canonical-history regression fixtures and 21 checker unit checks pass. Under Go history,
`base-type-cycle-entry` and `union-too-complex-cross-product` retain their baseline history-dependent output;
the other 48 fixtures match their canonical expected output. Six focused memo fixtures have identical diagnostics
and exit codes across the inference-only baseline, all-cache defaults, explicit opt-outs, explicit opt-ins and
combined shadow mode. On `infer-memo-repeated-walks`, inference records 125 hits skipping 335,529 steps, and
union caching records 6,208 hits; corresponding opt-outs record zero. Clippy across all targets with warnings
denied, formatting and source lint pass.

After narrowing the PR to caches, rebuilt the dev CLI/runner and release CLI, reran all 50 regression fixtures,
21 checker unit checks and the 30 focused memo configurations, and reran lint. All pass. The final release's
Mach-O `__TEXT` sections are byte-identical to the measured all-cache candidate; the full binary SHA-256 is
`ec61a5cd9d9a16d47f419e93d18deffb75c7ea35233f7f0ebb34a9a937171f05`.

## Artifacts and reproduction

Binary SHA-256 values, all built with the same release settings:

| Variant | SHA-256 |
| --- | --- |
| Base, Go cache defaults off | `676ad4773d7d31515367a10b70aa55b84b3031cca021fd5fdf4aeb4f8d722027` |
| Inference only | `b64d19fbba308c35ec4c2f1042b8f4a65d18847f4d474499a979423874578898` |
| All three caches | `9f70a82fe19450b3d375dff0dae3071c4291d2f2e9bc91a2736e2a28921572e6` |

Raw samples, preflights, binary metadata, focused checks and full-suite summaries are local ignored artifacts in
`target/go-infer-default/` and `target/go-union-ia-default/`. Their reports include the repeated confirmations.
The commands below rebuild the final candidate and repeat the union/indexed-access comparison against the saved
inference-only baseline. The earlier inference-stage table is recorded in `target/go-infer-default/perf/`.

```sh
cargo build --profile dev -p tsrs_cli -p tsrs_testrunner
cargo build --release -p tsrs_cli --config profile.release.debug=true --config profile.release.strip=false
node target/go-union-ia-default/bench.mjs default 7
node target/go-union-ia-default/bench.mjs single 3
node target/go-union-ia-default/bench.mjs single 7 xstate-main
```
