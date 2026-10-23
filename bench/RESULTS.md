# Benchmark results

Measured 2026-10-23T09:57:13.477Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `128d8d778565c1252fcfb95322c5dd1bc27e4a6d`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39934278411).

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
| Compiler-Unions | tsrs | 0.109 s | 0.080 GiB | 3,718,193,747 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261023.1 | 0.294 s | 0.147 GiB | 10,462,357,584 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.529 s | 0.238 GiB | 15,932,052,912 | 48 | 41 |
| Compiler | tsrs | 0.078 s | 0.077 GiB | 2,066,798,421 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261023.1 | 0.179 s | 0.131 GiB | 4,599,974,918 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.272 s | 0.213 GiB | 8,423,928,662 | 53 | 43 |
| cal-diy | tsrs | 1.200 s | 1.112 GiB | 46,758,255,062 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261023.1 | 4.092 s | 2.960 GiB | 146,585,116,903 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 5.290 s | 4.642 GiB | 217,569,628,500 | 10166 | 136 |
| date-fns | tsrs | 0.072 s | 0.126 GiB | 1,735,636,001 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261023.1 | 0.193 s | 0.242 GiB | 6,151,346,593 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.231 s | 0.283 GiB | 6,569,289,796 | 2270 | 1 |
| drizzle-orm | tsrs | 0.027 s | 0.087 GiB | 1,016,370,563 | 1554 | 2 |
| drizzle-orm | TypeScript 7.1.0-dev.20261023.1 | 0.104 s | 0.157 GiB | 3,512,902,416 | 1554 | 2 |
| drizzle-orm | TypeScript 7.0.2 | 0.368 s | 0.448 GiB | 15,941,468,530 | 1554 | 2 |
| formbricks-web | tsrs | 1.254 s | 1.382 GiB | 49,995,775,196 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261023.1 | 4.582 s | 4.187 GiB | 170,979,666,103 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 5.859 s | 5.736 GiB | 229,363,903,936 | 10451 | 0 |
| mikro-orm | tsrs | 1.504 s | 1.306 GiB | 64,044,442,858 | 2820 | 11 |
| mikro-orm | TypeScript 7.1.0-dev.20261023.1 | 5.221 s | 4.037 GiB | 234,359,475,617 | 2820 | 11 |
| mikro-orm | TypeScript 7.0.2 | 7.227 s | 6.837 GiB | 368,190,433,996 | 2820 | 2 |
| mui-docs | tsrs | 1.066 s | 1.012 GiB | 57,134,425,098 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261023.1 | 4.247 s | 3.368 GiB | 217,665,732,676 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 10.657 s | 7.919 GiB | 564,492,988,131 | 16766 | 0 |
| next-packages-next | tsrs | 0.520 s | 0.532 GiB | 19,605,310,801 | 2898 | 3 |
| next-packages-next | TypeScript 7.1.0-dev.20261023.1 | 1.954 s | 1.331 GiB | 61,658,598,361 | 2898 | 3 |
| next-packages-next | TypeScript 7.0.2 | 2.863 s | 1.894 GiB | 84,048,920,815 | 2891 | 3 |
| next-root | tsrs | 0.520 s | 0.407 GiB | 15,424,095,636 | 4001 | 4 |
| next-root | TypeScript 7.1.0-dev.20261023.1 | 2.150 s | 1.031 GiB | 42,825,188,264 | 4001 | 4 |
| next-root | TypeScript 7.0.2 | 1.993 s | 1.439 GiB | 59,493,912,396 | 3994 | 7 |
| nuxt | tsrs | 0.308 s | 0.353 GiB | 10,999,940,870 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261023.1 | 0.931 s | 0.907 GiB | 36,258,963,462 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 1.218 s | 1.146 GiB | 46,452,507,022 | 3705 | 3 |
| playwright | tsrs | 0.262 s | 0.310 GiB | 9,427,054,677 | 1527 | 0 |
| playwright | TypeScript 7.1.0-dev.20261023.1 | 0.785 s | 0.743 GiB | 30,009,011,805 | 1527 | 0 |
| playwright | TypeScript 7.0.2 | 1.213 s | 1.061 GiB | 44,533,556,809 | 1520 | 0 |
| storybook | tsrs | 0.301 s | 0.363 GiB | 12,529,576,675 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261023.1 | 1.105 s | 0.955 GiB | 39,890,601,832 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 1.515 s | 1.391 GiB | 62,421,314,572 | 3541 | 93 |
| supabase-studio | tsrs | 1.346 s | 1.139 GiB | 56,071,648,572 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261023.1 | 6.075 s | 3.125 GiB | 185,511,255,051 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 6.276 s | 4.802 GiB | 270,215,411,393 | 12546 | 0 |
| t3code-server | tsrs | 1.829 s | 1.128 GiB | 69,897,885,090 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261023.1 | 8.349 s | 4.358 GiB | 348,787,845,012 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 12.940 s | 6.357 GiB | 476,967,639,111 | 2941 | 5 |
| typeorm | tsrs | 0.254 s | 0.213 GiB | 6,874,524,482 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261023.1 | 0.704 s | 0.442 GiB | 18,765,958,565 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 0.992 s | 0.770 GiB | 28,018,665,054 | 3575 | 31 |
| vscode | tsrs | 2.358 s | 1.738 GiB | 87,513,342,349 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261023.1 | 6.887 s | 5.376 GiB | 234,579,802,940 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 10.005 s | 7.497 GiB | 308,851,157,685 | 10427 | 359 |
| webpack | tsrs | 0.310 s | 0.366 GiB | 12,494,509,847 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261023.1 | 1.561 s | 0.843 GiB | 34,809,725,438 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 1.882 s | 1.172 GiB | 49,860,985,193 | 1607 | 848 |
| xstate-main | tsrs | 0.323 s | 0.224 GiB | 6,728,459,990 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261023.1 | 1.010 s | 0.540 GiB | 20,712,421,914 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 1.214 s | 0.724 GiB | 28,395,835,112 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
