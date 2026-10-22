#!/usr/bin/env bash
# The root entry the README documents — the canonical runner lives in
# scripts/ (the golden tooling's own home, next to golden_matrix.sh).
exec "$(dirname "$0")/scripts/verify_goldens.sh" "$@"
