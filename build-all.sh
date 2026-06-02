#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$REPO_ROOT"

PASS=0
FAIL=0
FAILED=()

# Collect pe-XXXX members from workspace (excludes pe-utils)
MEMBERS=$(cargo metadata --no-deps --format-version 1 \
    | python3 -c "
import json, sys
data = json.load(sys.stdin)
names = sorted(p['name'] for p in data['packages'] if p['name'].startswith('pe-') and p['name'] != 'pe-utils')
print('\n'.join(names))
")

TOTAL=$(echo "$MEMBERS" | wc -l)
echo "Building $TOTAL solutions..."
echo

i=0
while IFS= read -r pkg; do
    i=$((i + 1))
    printf "[%3d/%d] %-12s  " "$i" "$TOTAL" "$pkg"
    if cargo build -p "$pkg" --message-format short 2>&1 | grep -q "^error"; then
        echo "FAIL"
        FAILED+=("$pkg")
        FAIL=$((FAIL + 1))
    else
        echo "ok"
        PASS=$((PASS + 1))
    fi
done <<< "$MEMBERS"

echo
echo "Results: $PASS passed, $FAIL failed"

if [ ${#FAILED[@]} -gt 0 ]; then
    echo
    echo "Failed:"
    for pkg in "${FAILED[@]}"; do
        echo "  $pkg"
    done
    exit 1
fi
