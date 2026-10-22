# Benchmark results

Measured 2026-10-22T09:10:06.912Z on Namespace `nscloud-ubuntu-24.04-amd64-8x16` (8 vCPU, 16 GB RAM, Linux x64, AMD EPYC).

Arithmetic means of 10 runs after 3 warmups per compiler and project. Projects are never averaged together.

4 checkers and 8 worker CPUs per compiler.

tsrs commit: `c74ed23a6fba583521f198bdecb105831b6c4460`.

[Benchmark run and raw-data artifacts](https://github.com/oxc-project/tsrs/actions/runs/39749803894).

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
| Compiler-Unions | tsrs | 0.087 s | 0.079 GiB | 3,532,627,086 | 48 | 41 |
| Compiler-Unions | TypeScript 7.1.0-dev.20261022.1 | 0.271 s | 0.147 GiB | 10,547,573,752 | 48 | 41 |
| Compiler-Unions | TypeScript 7.0.2 | 0.535 s | 0.239 GiB | 15,935,410,963 | 48 | 41 |
| Compiler | tsrs | 0.056 s | 0.077 GiB | 1,780,645,776 | 53 | 43 |
| Compiler | TypeScript 7.1.0-dev.20261022.1 | 0.140 s | 0.132 GiB | 4,567,795,231 | 53 | 43 |
| Compiler | TypeScript 7.0.2 | 0.240 s | 0.211 GiB | 8,399,678,306 | 53 | 43 |
| cal-diy | tsrs | 1.239 s | 1.123 GiB | 42,656,950,660 | 10173 | 136 |
| cal-diy | TypeScript 7.1.0-dev.20261022.1 | 4.001 s | 2.967 GiB | 146,043,676,011 | 10173 | 136 |
| cal-diy | TypeScript 7.0.2 | 5.592 s | 4.650 GiB | 219,530,105,358 | 10166 | 136 |
| date-fns | tsrs | 0.072 s | 0.143 GiB | 1,537,096,982 | 2277 | 1 |
| date-fns | TypeScript 7.1.0-dev.20261022.1 | 0.215 s | 0.241 GiB | 6,097,916,231 | 2277 | 1 |
| date-fns | TypeScript 7.0.2 | 0.232 s | 0.286 GiB | 6,871,864,149 | 2270 | 1 |
| drizzle-orm | tsrs | 0.331 s | 0.470 GiB | 11,688,533,524 | 3476 | 10846 |
| drizzle-orm | TypeScript 7.1.0-dev.20261022.1 | 1.387 s | 1.258 GiB | 48,623,124,239 | 3476 | 10845 |
| drizzle-orm | TypeScript 7.0.2 | 1.892 s | 1.731 GiB | 70,334,886,700 | 3469 | 10845 |
| formbricks-web | tsrs | 1.211 s | 1.394 GiB | 44,068,269,437 | 10458 | 0 |
| formbricks-web | TypeScript 7.1.0-dev.20261022.1 | 4.287 s | 4.209 GiB | 166,006,109,177 | 10458 | 0 |
| formbricks-web | TypeScript 7.0.2 | 5.680 s | 5.742 GiB | 228,816,604,188 | 10451 | 0 |
| mikro-orm | tsrs | 1.451 s | 1.363 GiB | 62,539,191,402 | 2911 | 43651 |
| mikro-orm | TypeScript 7.1.0-dev.20261022.1 | 5.885 s | 3.883 GiB | 261,701,008,616 | 2911 | 43651 |
| mikro-orm | TypeScript 7.0.2 | 8.790 s | 6.715 GiB | 403,614,375,127 | 2911 | 43651 |
| mui-docs | tsrs | 1.150 s | 1.014 GiB | 52,620,037,418 | 16766 | 0 |
| mui-docs | TypeScript 7.1.0-dev.20261022.1 | 5.500 s | 3.261 GiB | 214,299,242,800 | 16766 | 0 |
| mui-docs | TypeScript 7.0.2 | 12.140 s | 8.074 GiB | 572,690,259,389 | 16766 | 0 |
| next-packages-next | tsrs | 0.407 s | 0.511 GiB | 16,425,183,540 | 2889 | 6 |
| next-packages-next | TypeScript 7.1.0-dev.20261022.1 | 2.870 s | 1.273 GiB | 62,075,287,500 | 2889 | 6 |
| next-packages-next | TypeScript 7.0.2 | 2.535 s | 1.795 GiB | 82,087,718,108 | 2882 | 6 |
| next-root | tsrs | 0.350 s | 0.365 GiB | 11,919,011,908 | 3555 | 438 |
| next-root | TypeScript 7.1.0-dev.20261022.1 | 1.118 s | 0.957 GiB | 40,526,583,713 | 3555 | 438 |
| next-root | TypeScript 7.0.2 | 1.572 s | 1.328 GiB | 54,196,597,508 | 3548 | 441 |
| nuxt | tsrs | 0.238 s | 0.354 GiB | 9,644,637,116 | 3705 | 1 |
| nuxt | TypeScript 7.1.0-dev.20261022.1 | 0.831 s | 0.899 GiB | 35,635,035,169 | 3705 | 1 |
| nuxt | TypeScript 7.0.2 | 1.050 s | 1.142 GiB | 46,850,787,448 | 3705 | 3 |
| playwright | tsrs | 0.269 s | 0.309 GiB | 8,157,520,252 | 1514 | 13 |
| playwright | TypeScript 7.1.0-dev.20261022.1 | 0.991 s | 0.769 GiB | 31,675,986,954 | 1514 | 13 |
| playwright | TypeScript 7.0.2 | 1.222 s | 1.075 GiB | 44,301,132,708 | 1507 | 13 |
| storybook | tsrs | 0.403 s | 0.365 GiB | 11,002,378,338 | 3548 | 93 |
| storybook | TypeScript 7.1.0-dev.20261022.1 | 0.976 s | 0.949 GiB | 40,467,665,356 | 3548 | 93 |
| storybook | TypeScript 7.0.2 | 1.545 s | 1.398 GiB | 62,714,871,818 | 3541 | 93 |
| supabase-studio | tsrs | 1.161 s | 1.145 GiB | 50,637,330,424 | 12553 | 9 |
| supabase-studio | TypeScript 7.1.0-dev.20261022.1 | 4.638 s | 3.125 GiB | 185,100,126,834 | 12553 | 9 |
| supabase-studio | TypeScript 7.0.2 | 5.390 s | 4.758 GiB | 265,969,729,157 | 12546 | 0 |
| t3code-server | tsrs | 1.543 s | 1.133 GiB | 60,301,347,493 | 2948 | 6 |
| t3code-server | TypeScript 7.1.0-dev.20261022.1 | 12.686 s | 4.348 GiB | 348,623,400,542 | 2948 | 6 |
| t3code-server | TypeScript 7.0.2 | 11.198 s | 6.350 GiB | 470,613,110,681 | 2941 | 5 |
| typeorm | tsrs | 0.164 s | 0.213 GiB | 6,020,173,711 | 3575 | 31 |
| typeorm | TypeScript 7.1.0-dev.20261022.1 | 0.551 s | 0.448 GiB | 18,578,493,408 | 3575 | 31 |
| typeorm | TypeScript 7.0.2 | 0.684 s | 0.774 GiB | 28,582,601,529 | 3575 | 31 |
| vscode | tsrs | 2.671 s | 1.739 GiB | 75,370,711,054 | 10427 | 371 |
| vscode | TypeScript 7.1.0-dev.20261022.1 | 6.710 s | 5.443 GiB | 228,399,859,013 | 10427 | 371 |
| vscode | TypeScript 7.0.2 | 13.862 s | 7.471 GiB | 311,001,612,922 | 10427 | 359 |
| webpack | tsrs | 0.252 s | 0.365 GiB | 10,675,774,336 | 1607 | 840 |
| webpack | TypeScript 7.1.0-dev.20261022.1 | 1.000 s | 0.844 GiB | 34,804,778,887 | 1607 | 840 |
| webpack | TypeScript 7.0.2 | 1.223 s | 1.170 GiB | 48,810,904,259 | 1607 | 848 |
| xstate-main | tsrs | 0.381 s | 0.222 GiB | 5,903,730,396 | 1539 | 0 |
| xstate-main | TypeScript 7.1.0-dev.20261022.1 | 0.694 s | 0.548 GiB | 21,232,165,131 | 1539 | 0 |
| xstate-main | TypeScript 7.0.2 | 0.874 s | 0.724 GiB | 27,864,743,109 | 1532 | 0 |

File counts and diagnostics were collected in separate, untimed runs.
