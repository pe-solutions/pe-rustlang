#!/usr/bin/env bash
# Usage:
#   ./fetch-problems.sh --all   full scrape of /archives (first-time setup)
#   ./fetch-problems.sh         update via /recent only

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$REPO_ROOT"

MODE="${1:---recent}"

if [ "$MODE" != "--all" ] && [ "$MODE" != "--recent" ]; then
    echo "Usage: $0 [--all | --recent]" >&2
    exit 1
fi

if [ "$MODE" = "--recent" ] && [ ! -f "problems.toml" ]; then
    echo "Error: problems.toml not found. Run '$0 --all' first." >&2
    exit 1
fi

MODE="$MODE" python3 << 'PYEOF'
import urllib.request
import re
import os
import sys
import time
from datetime import date

MODE = os.environ["MODE"]

HEADERS = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Accept": "text/html,application/xhtml+xml",
    "Accept-Language": "en-US,en;q=0.9",
}
ROW_RE = re.compile(
    r'<td class="id_column">(\d+)</td>'
    r'<td><a href="problem=\d+"[^>]*>([^<]+)</a>'
)


def fetch(url):
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=15) as r:
        return r.read().decode("utf-8")


def parse_rows(html):
    return {int(n): title for n, title in ROW_RE.findall(html)}


def is_solved(num):
    path = f"solutions/pe-{num:04d}/src/main.rs"
    if not os.path.exists(path):
        return False
    with open(path) as f:
        return "todo!()" not in f.read()


def load_existing():
    if not os.path.exists("problems.toml"):
        return {}
    data = {}
    current = None
    with open("problems.toml") as f:
        for line in f:
            line = line.rstrip()
            m = re.match(r"^\[(\d+)\]$", line)
            if m:
                current = int(m.group(1))
                data[current] = {}
                continue
            if current is None:
                continue
            m = re.match(r'^(\w+)\s*=\s*(.+)$', line)
            if m:
                key, raw = m.group(1), m.group(2).strip()
                if raw == "true":
                    data[current][key] = True
                elif raw == "false":
                    data[current][key] = False
                else:
                    data[current][key] = raw.strip('"')
    return data


def write_toml(merged):
    today = date.today().isoformat()
    lines = [
        f"# Updated: {today}",
        "# ./fetch-problems.sh --all   full scrape (first-time setup)",
        "# ./fetch-problems.sh         update via /recent",
        "",
    ]
    for num in sorted(merged):
        title = merged[num]["title"]
        solved = merged[num]["solved"]
        lines.append(f"[{num}]")
        lines.append(f'title = "{title}"')
        lines.append(f"solved = {str(solved).lower()}")
        lines.append("")
    with open("problems.toml", "w") as f:
        f.write("\n".join(lines))


existing = load_existing()

if MODE == "--all":
    fetched = {}
    page = 1
    while True:
        url = ("https://projecteuler.net/archives"
               if page == 1
               else f"https://projecteuler.net/archives;page={page}")
        print(f"  page {page:2d} ... ", end="", flush=True)
        rows = parse_rows(fetch(url))
        new = {n: t for n, t in rows.items() if n not in fetched}
        if not new:
            print("done.")
            break
        fetched.update(new)
        print(f"{len(new)} problems  (total: {len(fetched)})")
        page += 1
        time.sleep(0.5)
else:
    print("  fetching /recent ...", end=" ", flush=True)
    fetched = parse_rows(fetch("https://projecteuler.net/recent"))
    print(f"{len(fetched)} problems")

# Merge: fetched titles win; solved flag preserved if already set, else auto-detect
merged = dict(existing)
for num, title in fetched.items():
    entry = existing.get(num, {})
    solved = entry.get("solved", is_solved(num))
    merged[num] = {"title": title, "solved": solved}

write_toml(merged)
new_count = sum(1 for n in fetched if n not in existing)
print(f"\nproblems.toml written  ({len(merged)} total, {new_count} new)")
PYEOF
