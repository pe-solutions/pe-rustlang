#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$REPO_ROOT"

if [ $# -ne 1 ]; then
    echo "Usage: $0 <problem-number>" >&2
    echo "  Example: $0 42" >&2
    exit 1
fi

if ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: '$1' is not a valid problem number" >&2
    exit 1
fi

NUM=$(printf "%04d" "$1")
CRATE="pe-$NUM"
DIR="solutions/$CRATE"
PROBLEM_NUM=$((10#$NUM))
PROBLEM_URL="https://projecteuler.net/problem=$PROBLEM_NUM"

if [ -d "$DIR" ]; then
    echo "Error: $DIR already exists" >&2
    exit 1
fi

# Look up title from problems.toml (run ./fetch-problems.sh to populate)
TITLE=$(NUM="$PROBLEM_NUM" python3 << 'PYEOF'
import os, re
num = int(os.environ["NUM"])
if not os.path.exists("problems.toml"):
    exit(0)
current = None
with open("problems.toml") as f:
    for line in f:
        line = line.rstrip()
        if re.match(rf"^\[{num}\]$", line):
            current = num
        elif current == num:
            m = re.match(r'^title\s*=\s*"(.+)"$', line)
            if m:
                print(m.group(1))
                break
PYEOF
)

mkdir -p "$DIR/src"

cat > "$DIR/Cargo.toml" << TOML
[package]
name = "$CRATE"
version = "0.1.0"
edition = "2021"
authors = ["menjaraz"]


[dependencies]
pe-utils = { workspace = true }
TOML

# Build header: title line if available, then URL
HEADER="// $PROBLEM_URL"
if [ -n "$TITLE" ]; then
    HEADER="// $TITLE
// $PROBLEM_URL"
fi

cat > "$DIR/src/main.rs" << RUST
$HEADER

fn solve() -> u64 {
    todo!()
}

pe_utils::pe_main!();
RUST

# Insert into workspace Cargo.toml in sorted order; pe-utils stays last
CRATE="$CRATE" python3 << 'PYEOF'
import os, re

crate = os.environ['CRATE']
new_member = f'"solutions/{crate}"'

with open("Cargo.toml") as f:
    content = f.read()

match = re.search(r'(members\s*=\s*\[)(.*?)(\])', content, re.DOTALL)
block = match.group(2)
members = [m.strip().rstrip(',') for m in block.strip().splitlines() if m.strip()]

solution_members = sorted([m for m in members if '"solutions/' in m] + [new_member])
other_members = [m for m in members if '"solutions/' not in m]
new_block = '\n' + ''.join(f'    {m},\n' for m in solution_members + other_members)

new_content = content[:match.start(2)] + new_block + content[match.end(2):]

with open("Cargo.toml", "w") as f:
    f.write(new_content)
PYEOF

echo "Created  $DIR/"
echo "Added    $CRATE to workspace Cargo.toml"
