#!/usr/bin/env bash
# Smoke test of a built headless `tsrs-lint` (.github/workflows/lint-release.yml):
#
#   lint-smoke.sh <tsrs-lint binary>
#
# Writes a one-file project with one floating promise, sends it the request shape oxlint sends (the V2 payload) and
# requires exactly one `no-floating-promises` diagnostic frame back with exit 0. Frames are [u32 LE length][u8 kind][JSON].
set -euo pipefail

bin=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cd "$work"

cat > tsconfig.json <<'JSON'
{ "compilerOptions": { "strict": true, "target": "es2022", "lib": ["es2022"], "types": [], "noEmit": true }, "include": ["*.ts"] }
JSON
cat > floating.ts <<'TS'
export async function load(): Promise<number> {
  return 1;
}
export function run(): void {
  load();
}
TS
printf '{"version":2,"configs":[{"file_paths":["%s/floating.ts"],"rules":[{"name":"no-floating-promises"},{"name":"await-thenable"}]}],"report_syntactic":false,"report_semantic":false}\n' "$work" > payload.json
cat > frames.cjs <<'JS'
const { readFileSync } = require('node:fs');
const bytes = readFileSync(process.argv[2]);
let frames = 0;
let found = 0;
for (let at = 0; at < bytes.length; ) {
  const length = bytes.readUInt32LE(at);
  const kind = bytes[at + 4];
  const body = JSON.parse(bytes.subarray(at + 5, at + 5 + length).toString('utf8'));
  if (kind === 1 && body.rule === 'no-floating-promises') found++;
  frames++;
  at += 5 + length;
}
console.log(`${frames} frames, ${found} no-floating-promises`);
process.exit(found === 1 ? 0 : 1);
JS

"$bin" headless < payload.json > frames.bin
node frames.cjs frames.bin
