#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$repo_root"

wasm-pack build \
  --target web \
  --release \
  --out-dir playground/pkg
