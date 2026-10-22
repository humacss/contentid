#!/usr/bin/env bash
set -euo pipefail

# CI-only override for MinBin add-types. Remove after MinBin 1.0 is published.
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[patch.crates-io]
minbin = { git = "https://github.com/humacss/minbin.git", rev = "bc87f2945f6ba0de0ac0c45c30ff2430473d94a4" }
EOF
