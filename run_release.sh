#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_BIN="$SCRIPT_DIR/target/release/shield_launcher"
FFMPEG_BIN="$SCRIPT_DIR/target/release/bin/ffmpeg"

if [[ ! -f "$RELEASE_BIN" ]]; then
    echo "Error: Release binary not found at $RELEASE_BIN."
    echo "Run 'cargo build --release -p shield_launcher' first."
    exit 1
fi

export PATH="$SCRIPT_DIR/target/release/bin:${PATH:-}"
export SHIELD_FFMPEG_PATH="$FFMPEG_BIN"

echo "Launching S.H.I.E.L.D Launcher (Pure Rust + Native GPU)..."
exec "$RELEASE_BIN" "$@"
