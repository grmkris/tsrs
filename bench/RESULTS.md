# Benchmark results

Measured 2026-10-23T10:53:59.530Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `63dd4666386e4fe07e56b7177b83557c3cbd6148`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39941002316).

[Methodology](README.md).

![All measured compiler versions](benchmark.svg)

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="large-projects-dark.svg">
  <img alt="Type-check speedup against peak-memory ratio" src="large-projects.svg">
</picture>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="per-project-time-dark.svg">
  <img alt="Check time by project" src="per-project-time.svg">
</picture>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="per-project-memory-dark.svg">
  <img alt="Peak memory by project" src="per-project-memory.svg">
</picture>

| Project | Compiler | Wall time | Peak memory | Instructions | Files | Diagnostics |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Compiler-Unions | tsrs | 0.138 s | 0.080 GiB | 3,719,342,322 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261023.1 | 0.455 s | 0.149 GiB | 10,418,457,369 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.672 s | 0.237 GiB | 15,849,904,564 | 48 | 41 |
| Compiler | tsrs | 0.100 s | 0.076 GiB | 2,068,737,530 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261023.1 | 0.205 s | 0.132 GiB | 4,562,556,764 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.358 s | 0.213 GiB | 8,561,675,804 | 53 | 43 |
| cal-diy | tsrs | 1.950 s | 1.112 GiB | 46,445,744,355 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261023.1 | 5.609 s | 2.984 GiB | 148,195,719,690 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 7.235 s | 4.620 GiB | 220,477,936,935 | 10166 | 136 |
| date-fns | tsrs | 0.090 s | 0.126 GiB | 1,733,702,171 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261023.1 | 0.257 s | 0.240 GiB | 5,892,911,308 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.275 s | 0.283 GiB | 6,728,034,050 | 2270 | 1 |
| drizzle-orm | tsrs | 0.048 s | 0.087 GiB | 1,015,337,991 | 1554 | 2 |
| drizzle-orm | TypeScript 7.1.0-dev.20261023.1 | 0.131 s | 0.156 GiB | 3,446,281,903 | 1554 | 2 |
| drizzle-orm | TypeScript 7.0.2 | 0.487 s | 0.447 GiB | 16,368,517,188 | 1554 | 2 |
| formbricks-web | tsrs | 1.914 s | 1.384 GiB | 49,961,137,077 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261023.1 | 7.193 s | 4.201 GiB | 168,755,419,364 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 9.083 s | 5.707 GiB | 231,895,724,288 | 10451 | 0 |
| mikro-orm | tsrs | 2.163 s | 1.305 GiB | 64,074,707,041 | 2820 | 11 |
| mikro-orm | TypeScript 7.1.0-dev.20261023.1 | 9.681 s | 3.965 GiB | 238,141,354,653 | 2820 | 11 |
| mikro-orm | TypeScript 7.0.2 | 11.647 s | 6.759 GiB | 368,635,637,944 | 2820 | 2 |
| mui-docs | tsrs | 1.407 s | 1.011 GiB | 57,099,275,968 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261023.1 | 5.480 s | 3.212 GiB | 212,053,755,698 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 13.116 s | 7.866 GiB | 564,483,874,763 | 16766 | 0 |
| next-packages-next | tsrs | 0.744 s | 0.533 GiB | 19,646,938,463 | 2898 | 3 |
| next-packages-next | TypeScript 7.1.0-dev.20261023.1 | 2.677 s | 1.342 GiB | 62,502,681,639 | 2898 | 3 |
| next-packages-next | TypeScript 7.0.2 | 4.065 s | 1.871 GiB | 86,118,539,628 | 2891 | 3 |
| next-root | tsrs | 0.631 s | 0.407 GiB | 15,413,514,661 | 4001 | 4 |
| next-root | TypeScript 7.1.0-dev.20261023.1 | 2.285 s | 1.021 GiB | 42,753,521,176 | 4001 | 4 |
| next-root | TypeScript 7.0.2 | 2.525 s | 1.427 GiB | 57,661,146,137 | 3994 | 7 |
| nuxt | tsrs | 0.564 s | 0.351 GiB | 10,966,805,280 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261023.1 | 1.300 s | 0.902 GiB | 35,437,281,850 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 2.371 s | 1.152 GiB | 46,808,730,805 | 3705 | 3 |
| playwright | tsrs | 0.590 s | 0.309 GiB | 9,427,364,139 | 1527 | 0 |
| playwright | TypeScript 7.1.0-dev.20261023.1 | 0.944 s | 0.751 GiB | 29,730,275,728 | 1527 | 0 |
| playwright | TypeScript 7.0.2 | 1.400 s | 1.057 GiB | 44,056,497,491 | 1520 | 0 |
| storybook | tsrs | 0.416 s | 0.365 GiB | 12,537,799,000 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261023.1 | 1.451 s | 0.954 GiB | 39,926,015,252 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 2.089 s | 1.372 GiB | 62,548,972,433 | 3541 | 93 |
| supabase-studio | tsrs | 2.120 s | 1.137 GiB | 55,984,697,711 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261023.1 | 6.353 s | 3.159 GiB | 182,426,084,340 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 8.144 s | 4.837 GiB | 274,809,234,227 | 12546 | 0 |
| t3code-server | tsrs | 2.612 s | 1.125 GiB | 69,805,814,859 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261023.1 | 11.510 s | 4.381 GiB | 344,244,882,829 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 18.112 s | 6.322 GiB | 473,707,277,648 | 2941 | 5 |
| typeorm | tsrs | 0.293 s | 0.214 GiB | 6,869,198,444 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261023.1 | 0.862 s | 0.444 GiB | 18,287,693,124 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 1.131 s | 0.791 GiB | 29,967,133,122 | 3575 | 31 |
| vscode | tsrs | 2.901 s | 1.738 GiB | 87,446,850,099 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261023.1 | 9.193 s | 5.387 GiB | 233,466,855,994 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 12.926 s | 7.453 GiB | 315,234,739,162 | 10427 | 359 |
| webpack | tsrs | 0.374 s | 0.367 GiB | 12,471,044,815 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261023.1 | 1.107 s | 0.839 GiB | 34,801,159,971 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 1.465 s | 1.159 GiB | 47,586,979,970 | 1607 | 848 |
| xstate-main | tsrs | 0.317 s | 0.223 GiB | 6,728,635,097 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261023.1 | 1.448 s | 0.555 GiB | 21,197,305,693 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 1.789 s | 0.723 GiB | 27,525,383,886 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
