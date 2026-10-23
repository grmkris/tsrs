# Benchmark results

Measured 2026-10-23T10:09:15.263Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `736a0e189939278c9878e5a3454f8126f55081fc`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39936049194).

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
| Compiler-Unions | tsrs | 0.097 s | 0.080 GiB | 3,721,844,721 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261023.1 | 0.265 s | 0.148 GiB | 10,569,515,942 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.472 s | 0.236 GiB | 15,959,328,485 | 48 | 41 |
| Compiler | tsrs | 0.067 s | 0.077 GiB | 2,069,716,890 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261023.1 | 0.139 s | 0.132 GiB | 4,550,879,924 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.238 s | 0.213 GiB | 8,474,245,994 | 53 | 43 |
| cal-diy | tsrs | 1.216 s | 1.106 GiB | 46,033,834,785 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261023.1 | 3.840 s | 2.928 GiB | 145,125,953,924 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 7.656 s | 4.669 GiB | 218,878,277,823 | 10166 | 136 |
| date-fns | tsrs | 0.059 s | 0.127 GiB | 1,734,825,855 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261023.1 | 0.314 s | 0.245 GiB | 5,953,186,864 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.194 s | 0.286 GiB | 6,867,074,450 | 2270 | 1 |
| drizzle-orm | tsrs | 0.029 s | 0.087 GiB | 1,016,155,273 | 1554 | 2 |
| drizzle-orm | TypeScript 7.1.0-dev.20261023.1 | 0.104 s | 0.156 GiB | 3,490,770,841 | 1554 | 2 |
| drizzle-orm | TypeScript 7.0.2 | 0.385 s | 0.442 GiB | 16,169,314,700 | 1554 | 2 |
| formbricks-web | tsrs | 1.271 s | 1.382 GiB | 49,965,754,393 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261023.1 | 4.105 s | 4.228 GiB | 169,763,427,001 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 5.449 s | 5.735 GiB | 227,993,794,565 | 10451 | 0 |
| mikro-orm | tsrs | 2.407 s | 1.309 GiB | 64,024,984,310 | 2820 | 11 |
| mikro-orm | TypeScript 7.1.0-dev.20261023.1 | 5.899 s | 3.941 GiB | 238,759,970,400 | 2820 | 11 |
| mikro-orm | TypeScript 7.0.2 | 8.401 s | 6.830 GiB | 378,447,021,109 | 2820 | 2 |
| mui-docs | tsrs | 1.105 s | 1.012 GiB | 57,109,822,708 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261023.1 | 4.599 s | 3.301 GiB | 213,655,841,979 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 10.506 s | 7.969 GiB | 566,492,306,519 | 16766 | 0 |
| next-packages-next | tsrs | 0.565 s | 0.532 GiB | 19,642,176,425 | 2898 | 3 |
| next-packages-next | TypeScript 7.1.0-dev.20261023.1 | 1.865 s | 1.328 GiB | 61,278,380,317 | 2898 | 3 |
| next-packages-next | TypeScript 7.0.2 | 2.798 s | 1.875 GiB | 85,227,955,246 | 2891 | 3 |
| next-root | tsrs | 0.360 s | 0.408 GiB | 15,428,693,655 | 4001 | 4 |
| next-root | TypeScript 7.1.0-dev.20261023.1 | 1.088 s | 1.026 GiB | 42,539,860,186 | 4001 | 4 |
| next-root | TypeScript 7.0.2 | 1.418 s | 1.424 GiB | 58,662,189,904 | 3994 | 7 |
| nuxt | tsrs | 0.277 s | 0.351 GiB | 11,000,766,734 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261023.1 | 0.849 s | 0.902 GiB | 35,524,026,265 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 1.116 s | 1.146 GiB | 46,344,644,459 | 3705 | 3 |
| playwright | tsrs | 0.265 s | 0.309 GiB | 9,434,209,774 | 1527 | 0 |
| playwright | TypeScript 7.1.0-dev.20261023.1 | 0.766 s | 0.753 GiB | 29,859,465,287 | 1527 | 0 |
| playwright | TypeScript 7.0.2 | 0.994 s | 1.080 GiB | 45,730,478,982 | 1520 | 0 |
| storybook | tsrs | 0.311 s | 0.364 GiB | 12,517,085,983 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261023.1 | 0.990 s | 0.956 GiB | 40,222,536,419 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 1.510 s | 1.373 GiB | 63,356,616,757 | 3541 | 93 |
| supabase-studio | tsrs | 1.299 s | 1.138 GiB | 56,116,262,424 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261023.1 | 4.672 s | 3.106 GiB | 186,052,906,943 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 5.806 s | 4.738 GiB | 268,214,710,929 | 12546 | 0 |
| t3code-server | tsrs | 1.808 s | 1.128 GiB | 69,858,802,568 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261023.1 | 8.654 s | 4.281 GiB | 351,203,970,328 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 12.454 s | 6.374 GiB | 475,490,533,008 | 2941 | 5 |
| typeorm | tsrs | 0.200 s | 0.214 GiB | 6,869,265,142 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261023.1 | 0.506 s | 0.446 GiB | 18,366,594,747 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 0.706 s | 0.770 GiB | 28,398,808,102 | 3575 | 31 |
| vscode | tsrs | 2.309 s | 1.738 GiB | 87,432,758,654 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261023.1 | 8.942 s | 5.291 GiB | 243,290,410,661 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 9.336 s | 7.430 GiB | 317,037,979,779 | 10427 | 359 |
| webpack | tsrs | 0.529 s | 0.367 GiB | 12,480,822,589 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261023.1 | 0.891 s | 0.842 GiB | 34,685,904,811 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 2.008 s | 1.177 GiB | 47,090,150,068 | 1607 | 848 |
| xstate-main | tsrs | 0.174 s | 0.224 GiB | 6,733,072,518 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261023.1 | 0.528 s | 0.547 GiB | 20,969,369,077 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 0.685 s | 0.723 GiB | 27,834,445,644 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
