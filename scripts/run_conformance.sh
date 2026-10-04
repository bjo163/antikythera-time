#!/usr/bin/env bash
set -euo pipefail
cargo test --workspace
python reference-python/mtime_reference.py
python reference-python/mtime2_reference.py
python sdk/python/test_mtime_sdk.py
python scripts/check_layering.py
echo "M-Time public conformance runner: PASS"
