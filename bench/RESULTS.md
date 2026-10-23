# Benchmark results

Measured 2026-10-23T10:57:43.119Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `935dd26ef76948a58e2c1717b4cd994bd6793328`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39941307502).

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
| Compiler-Unions | tsrs | 0.210 s | 0.079 GiB | 3,715,830,967 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261023.1 | 0.403 s | 0.149 GiB | 10,470,352,610 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.720 s | 0.240 GiB | 15,799,394,685 | 48 | 41 |
| Compiler | tsrs | 0.191 s | 0.076 GiB | 2,070,245,351 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261023.1 | 0.373 s | 0.132 GiB | 4,551,103,741 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.680 s | 0.212 GiB | 8,302,146,694 | 53 | 43 |
| cal-diy | tsrs | 1.722 s | 1.082 GiB | 46,172,328,028 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261023.1 | 5.875 s | 2.970 GiB | 147,925,189,288 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 7.714 s | 4.583 GiB | 221,441,139,548 | 10166 | 136 |
| date-fns | tsrs | 0.118 s | 0.126 GiB | 1,733,780,007 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261023.1 | 0.297 s | 0.240 GiB | 6,010,678,589 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.336 s | 0.286 GiB | 6,899,043,587 | 2270 | 1 |
| drizzle-orm | tsrs | 0.045 s | 0.087 GiB | 1,014,892,292 | 1554 | 2 |
| drizzle-orm | TypeScript 7.1.0-dev.20261023.1 | 0.147 s | 0.156 GiB | 3,478,818,572 | 1554 | 2 |
| drizzle-orm | TypeScript 7.0.2 | 0.515 s | 0.443 GiB | 16,098,584,461 | 1554 | 2 |
| formbricks-web | tsrs | 1.870 s | 1.362 GiB | 50,090,023,323 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261023.1 | 6.695 s | 4.215 GiB | 167,482,464,731 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 8.497 s | 5.763 GiB | 225,773,401,839 | 10451 | 0 |
| mikro-orm | tsrs | 2.062 s | 1.272 GiB | 64,090,142,663 | 2820 | 11 |
| mikro-orm | TypeScript 7.1.0-dev.20261023.1 | 8.251 s | 3.985 GiB | 234,860,355,726 | 2820 | 11 |
| mikro-orm | TypeScript 7.0.2 | 10.913 s | 6.736 GiB | 372,379,001,256 | 2820 | 2 |
| mui-docs | tsrs | 1.367 s | 0.996 GiB | 57,215,804,632 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261023.1 | 5.978 s | 3.282 GiB | 204,755,099,365 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 13.057 s | 7.695 GiB | 551,641,551,794 | 16766 | 0 |
| next-packages-next | tsrs | 0.697 s | 0.524 GiB | 19,679,250,826 | 2898 | 3 |
| next-packages-next | TypeScript 7.1.0-dev.20261023.1 | 2.471 s | 1.350 GiB | 63,343,486,270 | 2898 | 3 |
| next-packages-next | TypeScript 7.0.2 | 3.553 s | 1.886 GiB | 84,873,798,836 | 2891 | 3 |
| next-root | tsrs | 0.546 s | 0.402 GiB | 15,427,733,117 | 4001 | 4 |
| next-root | TypeScript 7.1.0-dev.20261023.1 | 1.632 s | 1.024 GiB | 42,930,677,992 | 4001 | 4 |
| next-root | TypeScript 7.0.2 | 2.153 s | 1.430 GiB | 57,689,027,485 | 3994 | 7 |
| nuxt | tsrs | 0.307 s | 0.348 GiB | 10,981,312,934 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261023.1 | 1.143 s | 0.905 GiB | 35,726,816,541 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 1.277 s | 1.147 GiB | 47,363,760,385 | 3705 | 3 |
| playwright | tsrs | 0.544 s | 0.304 GiB | 9,462,037,710 | 1527 | 0 |
| playwright | TypeScript 7.1.0-dev.20261023.1 | 1.271 s | 0.752 GiB | 30,483,035,573 | 1527 | 0 |
| playwright | TypeScript 7.0.2 | 1.878 s | 1.048 GiB | 43,715,263,048 | 1520 | 0 |
| storybook | tsrs | 0.369 s | 0.360 GiB | 12,571,381,081 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261023.1 | 1.283 s | 0.948 GiB | 40,064,953,377 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 1.869 s | 1.375 GiB | 61,877,136,987 | 3541 | 93 |
| supabase-studio | tsrs | 2.353 s | 1.126 GiB | 55,854,367,128 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261023.1 | 9.437 s | 3.139 GiB | 184,050,719,880 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 12.987 s | 4.790 GiB | 270,792,742,067 | 12546 | 0 |
| t3code-server | tsrs | 2.923 s | 1.104 GiB | 69,853,529,317 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261023.1 | 12.058 s | 4.339 GiB | 351,270,335,391 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 20.458 s | 6.347 GiB | 476,468,979,085 | 2941 | 5 |
| typeorm | tsrs | 0.295 s | 0.212 GiB | 6,872,159,171 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261023.1 | 0.858 s | 0.443 GiB | 18,346,581,660 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 1.214 s | 0.794 GiB | 29,364,241,162 | 3575 | 31 |
| vscode | tsrs | 3.114 s | 1.718 GiB | 87,526,563,376 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261023.1 | 9.409 s | 5.386 GiB | 234,052,905,505 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 14.415 s | 7.474 GiB | 312,989,949,061 | 10427 | 359 |
| webpack | tsrs | 0.394 s | 0.361 GiB | 12,482,709,701 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261023.1 | 1.290 s | 0.825 GiB | 35,734,195,468 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 1.623 s | 1.175 GiB | 48,383,537,664 | 1607 | 848 |
| xstate-main | tsrs | 0.283 s | 0.220 GiB | 6,734,329,438 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261023.1 | 0.941 s | 0.548 GiB | 21,025,608,486 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 1.262 s | 0.722 GiB | 27,840,420,799 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
