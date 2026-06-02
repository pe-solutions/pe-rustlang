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

# Look up title and data_url from problems.toml
eval "$(NUM="$PROBLEM_NUM" python3 << 'PYEOF'
import os, re, shlex
num = int(os.environ["NUM"])
title, data_url = "", ""
if os.path.exists("problems.toml"):
    all_lines = open("problems.toml").read().splitlines()
    current, i = None, 0
    while i < len(all_lines):
        line = all_lines[i].rstrip()
        m = re.match(r"^\[(\d+)\]$", line)
        if m:
            current = int(m.group(1))
            if current > num:
                break
            i += 1
            continue
        if current != num:
            i += 1
            continue
        # Skip multiline blocks
        if re.match(r"^\w+\s*=\s*'''$", line):
            i += 1
            while i < len(all_lines) and not all_lines[i].rstrip().endswith("'''"):
                i += 1
            i += 1
            continue
        m = re.match(r'^title\s*=\s*"(.+)"$', line)
        if m:
            title = m.group(1)
        m = re.match(r'^data_url\s*=\s*"(.+)"$', line)
        if m:
            data_url = m.group(1)
        i += 1
print(f"TITLE={shlex.quote(title)}")
print(f"DATA_URL={shlex.quote(data_url)}")
PYEOF
)"

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

# Download data file if listed in problems.toml and not already present
if [ -n "$DATA_URL" ]; then
    FILENAME=$(basename "$DATA_URL")
    if [ -f "$DIR/data/$FILENAME" ]; then
        echo "Data     $DIR/data/$FILENAME (already present)"
    else
        mkdir -p "$DIR/data"
        echo "Downloading $FILENAME ..."
        DATA_URL="$DATA_URL" DIR="$DIR" FILENAME="$FILENAME" python3 << 'PYEOF'
import os, urllib.request
urllib.request.urlretrieve(
    os.environ["DATA_URL"],
    f"{os.environ['DIR']}/data/{os.environ['FILENAME']}"
)
PYEOF
        echo "Data     $DIR/data/$FILENAME"
    fi
fi

echo "Created  $DIR/"
echo "Added    $CRATE to workspace Cargo.toml"
