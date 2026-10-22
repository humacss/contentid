#!/usr/bin/env bash
set -euo pipefail

# CI-only override for MinBin add-types. Remove after MinBin 1.0 is published.
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[patch.crates-io]
minbin = { git = "https://github.com/humacss/minbin.git", rev = "d0cce27aecef4b1eb4af293de113dbd0783e1217" }
EOF
