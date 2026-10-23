# Benchmark results

Measured 2026-10-23T09:59:41.474Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `1ebe78464e2b442ed1fe8204ccf204dbc243d3aa`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39934412526).

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
| Compiler-Unions | tsrs | 0.112 s | 0.080 GiB | 3,721,037,139 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261023.1 | 0.292 s | 0.146 GiB | 10,496,184,017 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.555 s | 0.238 GiB | 15,827,622,754 | 48 | 41 |
| Compiler | tsrs | 0.075 s | 0.077 GiB | 2,068,712,596 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261023.1 | 0.170 s | 0.131 GiB | 4,610,122,920 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.284 s | 0.213 GiB | 8,482,527,030 | 53 | 43 |
| cal-diy | tsrs | 1.088 s | 1.109 GiB | 46,378,693,205 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261023.1 | 3.371 s | 2.970 GiB | 146,177,892,744 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 4.897 s | 4.629 GiB | 217,092,700,432 | 10166 | 136 |
| date-fns | tsrs | 0.061 s | 0.127 GiB | 1,736,466,467 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261023.1 | 0.183 s | 0.244 GiB | 6,066,707,108 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.193 s | 0.284 GiB | 6,825,934,876 | 2270 | 1 |
| drizzle-orm | tsrs | 0.028 s | 0.087 GiB | 1,016,065,810 | 1554 | 2 |
| drizzle-orm | TypeScript 7.1.0-dev.20261023.1 | 0.094 s | 0.156 GiB | 3,453,272,713 | 1554 | 2 |
| drizzle-orm | TypeScript 7.0.2 | 0.346 s | 0.449 GiB | 16,327,216,577 | 1554 | 2 |
| formbricks-web | tsrs | 1.378 s | 1.384 GiB | 49,961,053,450 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261023.1 | 4.900 s | 4.178 GiB | 171,826,514,348 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 6.187 s | 5.752 GiB | 226,429,164,611 | 10451 | 0 |
| mikro-orm | tsrs | 2.133 s | 1.304 GiB | 64,028,611,992 | 2820 | 11 |
| mikro-orm | TypeScript 7.1.0-dev.20261023.1 | 7.781 s | 3.936 GiB | 240,822,438,178 | 2820 | 11 |
| mikro-orm | TypeScript 7.0.2 | 12.430 s | 6.790 GiB | 369,973,096,680 | 2820 | 2 |
| mui-docs | tsrs | 1.078 s | 1.012 GiB | 57,121,021,831 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261023.1 | 5.798 s | 3.285 GiB | 215,248,042,755 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 10.392 s | 7.931 GiB | 566,156,456,667 | 16766 | 0 |
| next-packages-next | tsrs | 0.525 s | 0.531 GiB | 19,658,892,630 | 2898 | 3 |
| next-packages-next | TypeScript 7.1.0-dev.20261023.1 | 1.892 s | 1.340 GiB | 62,471,130,118 | 2898 | 3 |
| next-packages-next | TypeScript 7.0.2 | 2.732 s | 1.873 GiB | 85,479,553,700 | 2891 | 3 |
| next-root | tsrs | 0.404 s | 0.408 GiB | 15,430,296,808 | 4001 | 4 |
| next-root | TypeScript 7.1.0-dev.20261023.1 | 1.223 s | 1.034 GiB | 42,491,615,705 | 4001 | 4 |
| next-root | TypeScript 7.0.2 | 1.602 s | 1.442 GiB | 60,150,074,908 | 3994 | 7 |
| nuxt | tsrs | 0.270 s | 0.352 GiB | 10,993,034,440 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261023.1 | 0.820 s | 0.901 GiB | 35,362,914,998 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 1.071 s | 1.152 GiB | 48,516,056,347 | 3705 | 3 |
| playwright | tsrs | 0.358 s | 0.311 GiB | 9,463,151,431 | 1527 | 0 |
| playwright | TypeScript 7.1.0-dev.20261023.1 | 1.081 s | 0.742 GiB | 29,555,742,078 | 1527 | 0 |
| playwright | TypeScript 7.0.2 | 1.205 s | 1.066 GiB | 45,149,879,603 | 1520 | 0 |
| storybook | tsrs | 0.377 s | 0.365 GiB | 12,577,540,176 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261023.1 | 1.037 s | 0.955 GiB | 39,051,406,171 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 1.722 s | 1.382 GiB | 62,781,255,992 | 3541 | 93 |
| supabase-studio | tsrs | 1.521 s | 1.138 GiB | 56,059,648,609 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261023.1 | 5.047 s | 3.125 GiB | 185,722,004,237 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 5.541 s | 4.772 GiB | 268,705,146,907 | 12546 | 0 |
| t3code-server | tsrs | 1.735 s | 1.128 GiB | 69,824,415,090 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261023.1 | 9.776 s | 4.374 GiB | 347,748,197,139 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 11.351 s | 6.404 GiB | 469,547,553,235 | 2941 | 5 |
| typeorm | tsrs | 0.253 s | 0.213 GiB | 6,871,814,142 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261023.1 | 0.731 s | 0.451 GiB | 18,131,063,884 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 0.977 s | 0.783 GiB | 28,794,535,945 | 3575 | 31 |
| vscode | tsrs | 2.376 s | 1.740 GiB | 87,498,145,826 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261023.1 | 6.877 s | 5.384 GiB | 233,517,547,768 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 9.276 s | 7.474 GiB | 309,526,691,653 | 10427 | 359 |
| webpack | tsrs | 0.358 s | 0.367 GiB | 12,514,285,754 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261023.1 | 1.048 s | 0.844 GiB | 34,573,940,636 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 1.396 s | 1.158 GiB | 46,896,541,624 | 1607 | 848 |
| xstate-main | tsrs | 0.285 s | 0.225 GiB | 6,733,886,767 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261023.1 | 0.953 s | 0.550 GiB | 21,287,640,197 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 1.197 s | 0.725 GiB | 28,133,206,069 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
