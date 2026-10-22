# Benchmark results

Measured 2026-10-22T10:21:55.578Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `d1ec3891a18c7a524c530242fc01a01f92ed1274`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39760526894).

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
| Compiler-Unions | tsrs | 0.079 s | 0.080 GiB | 3,328,348,917 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261022.1 | 0.246 s | 0.148 GiB | 10,562,036,790 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.445 s | 0.238 GiB | 15,862,174,309 | 48 | 41 |
| Compiler | tsrs | 0.064 s | 0.076 GiB | 1,756,837,482 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261022.1 | 0.163 s | 0.131 GiB | 4,623,727,690 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.275 s | 0.212 GiB | 8,430,746,099 | 53 | 43 |
| cal-diy | tsrs | 1.070 s | 1.115 GiB | 41,848,963,456 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261022.1 | 3.738 s | 2.937 GiB | 146,936,388,363 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 5.224 s | 4.655 GiB | 216,890,006,183 | 10166 | 136 |
| date-fns | tsrs | 0.064 s | 0.142 GiB | 1,488,860,058 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261022.1 | 0.201 s | 0.241 GiB | 6,126,054,485 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.225 s | 0.286 GiB | 6,968,853,452 | 2270 | 1 |
| drizzle-orm | tsrs | 0.379 s | 0.472 GiB | 11,610,983,170 | 3476 | 10846 |
| drizzle-orm | TypeScript 7.1.0-dev.20261022.1 | 1.535 s | 1.264 GiB | 49,298,638,170 | 3476 | 10845 |
| drizzle-orm | TypeScript 7.0.2 | 2.014 s | 1.741 GiB | 69,116,372,857 | 3469 | 10845 |
| formbricks-web | tsrs | 1.288 s | 1.398 GiB | 43,934,444,353 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261022.1 | 4.738 s | 4.230 GiB | 172,031,913,996 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 7.272 s | 5.723 GiB | 231,952,220,886 | 10451 | 0 |
| mikro-orm | tsrs | 1.996 s | 1.364 GiB | 62,170,082,701 | 2911 | 43651 |
| mikro-orm | TypeScript 7.1.0-dev.20261022.1 | 9.359 s | 3.886 GiB | 253,816,860,166 | 2911 | 43651 |
| mikro-orm | TypeScript 7.0.2 | 13.875 s | 6.786 GiB | 403,544,576,052 | 2911 | 43651 |
| mui-docs | tsrs | 1.214 s | 1.012 GiB | 51,687,405,067 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261022.1 | 4.103 s | 3.334 GiB | 215,656,099,072 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 12.058 s | 8.021 GiB | 570,589,973,545 | 16766 | 0 |
| next-packages-next | tsrs | 0.443 s | 0.512 GiB | 16,385,276,056 | 2889 | 6 |
| next-packages-next | TypeScript 7.1.0-dev.20261022.1 | 2.006 s | 1.263 GiB | 61,986,393,497 | 2889 | 6 |
| next-packages-next | TypeScript 7.0.2 | 2.448 s | 1.815 GiB | 80,896,807,859 | 2882 | 6 |
| next-root | tsrs | 0.332 s | 0.365 GiB | 11,839,235,415 | 3555 | 438 |
| next-root | TypeScript 7.1.0-dev.20261022.1 | 1.124 s | 0.966 GiB | 39,155,834,281 | 3555 | 438 |
| next-root | TypeScript 7.0.2 | 1.535 s | 1.299 GiB | 54,902,424,660 | 3548 | 441 |
| nuxt | tsrs | 0.328 s | 0.353 GiB | 9,572,301,521 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261022.1 | 0.945 s | 0.915 GiB | 36,390,319,712 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 1.237 s | 1.141 GiB | 46,842,611,838 | 3705 | 3 |
| playwright | tsrs | 0.221 s | 0.310 GiB | 8,133,014,025 | 1514 | 13 |
| playwright | TypeScript 7.1.0-dev.20261022.1 | 1.028 s | 0.765 GiB | 31,416,450,180 | 1514 | 13 |
| playwright | TypeScript 7.0.2 | 1.115 s | 1.075 GiB | 45,359,053,752 | 1507 | 13 |
| storybook | tsrs | 0.303 s | 0.366 GiB | 10,978,150,434 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261022.1 | 1.020 s | 0.955 GiB | 39,197,122,430 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 2.039 s | 1.391 GiB | 62,196,968,029 | 3541 | 93 |
| supabase-studio | tsrs | 1.248 s | 1.147 GiB | 50,428,075,565 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261022.1 | 5.341 s | 3.134 GiB | 185,373,633,937 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 6.223 s | 4.775 GiB | 272,622,426,043 | 12546 | 0 |
| t3code-server | tsrs | 1.711 s | 1.132 GiB | 60,217,529,053 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261022.1 | 8.899 s | 4.344 GiB | 347,191,415,742 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 11.952 s | 6.410 GiB | 470,848,903,643 | 2941 | 5 |
| typeorm | tsrs | 0.163 s | 0.212 GiB | 5,920,010,670 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261022.1 | 0.493 s | 0.447 GiB | 18,109,620,377 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 0.680 s | 0.771 GiB | 28,379,881,652 | 3575 | 31 |
| vscode | tsrs | 2.055 s | 1.739 GiB | 74,929,086,432 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261022.1 | 7.299 s | 5.304 GiB | 243,065,303,159 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 8.518 s | 7.560 GiB | 305,922,496,992 | 10427 | 359 |
| webpack | tsrs | 0.287 s | 0.364 GiB | 10,643,673,203 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261022.1 | 0.953 s | 0.839 GiB | 34,847,763,990 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 1.253 s | 1.144 GiB | 48,795,534,116 | 1607 | 848 |
| xstate-main | tsrs | 0.189 s | 0.222 GiB | 5,880,686,228 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261022.1 | 0.700 s | 0.548 GiB | 21,191,085,775 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 0.994 s | 0.719 GiB | 27,789,952,989 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
