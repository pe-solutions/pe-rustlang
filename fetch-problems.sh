#!/usr/bin/env bash
# Usage:
#   ./fetch-problems.sh --all   full scrape of /archives + all statements (first-time setup, ~10 min)
#   ./fetch-problems.sh         update via /recent + fetch new statements only

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
import time
import html as html_mod
from datetime import date

MODE = os.environ["MODE"]

HEADERS = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Accept": "text/html,application/xhtml+xml",
    "Accept-Language": "en-US,en;q=0.9",
}
MINIMAL_HEADERS = {**HEADERS, "Referer": "https://projecteuler.net/"}

ROW_RE = re.compile(
    r'<td class="id_column">(\d+)</td>'
    r'<td><a href="problem=\d+"[^>]*>([^<]+)</a>'
)
DATA_URL_RE = re.compile(r'href="(resources/documents/[^"]+)"')
PE_BASE = "https://projecteuler.net/"


def likely_has_data_file(statement):
    return bool(re.search(r'\b\w+\.(txt|csv)\b', statement, re.I))


def fetch(url, headers=HEADERS):
    req = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(req, timeout=15) as r:
        return r.read().decode("utf-8")


def parse_rows(html):
    return {int(n): title for n, title in ROW_RE.findall(html)}


def html_to_text(raw):
    h = raw
    h = re.sub(r'<p[^>]*>',    '',     h)
    h = re.sub(r'</p>',        '\n\n', h)
    h = re.sub(r'<br\s*/?>',   '\n',   h)
    h = re.sub(r'<li[^>]*>',   '\n- ', h)
    h = re.sub(r'</li>',       '',     h)
    h = re.sub(r'</?[uoU][lL][^>]*>', '\n', h)
    h = re.sub(r'<tr[^>]*>',   '',     h)
    h = re.sub(r'</tr>',       '\n',   h)
    h = re.sub(r'<t[dh][^>]*>', ' ',   h)
    h = re.sub(r'</t[dh]>',    '',     h)
    h = re.sub(r'<[^>]+>',     '',     h)
    h = html_mod.unescape(h)
    lines = [re.sub(r'[ \t]+', ' ', ln).strip() for ln in h.split('\n')]
    result, prev_blank = [], False
    for ln in lines:
        blank = not ln
        if not (blank and prev_blank):
            result.append(ln)
        prev_blank = blank
    return '\n'.join(result).strip()


def fetch_minimal(num):
    """Return (statement, data_url) from the minimal endpoint."""
    try:
        raw = fetch(f"https://projecteuler.net/minimal={num}", MINIMAL_HEADERS)
        stmt = html_to_text(raw)
        m = DATA_URL_RE.search(raw)
        data_url = PE_BASE + m.group(1) if m else ""
        return stmt, data_url
    except Exception:
        return None, ""


def is_solved(num):
    path = f"solutions/pe-{num:04d}/src/main.rs"
    if not os.path.exists(path):
        return False
    with open(path) as f:
        return "todo!()" not in f.read()


def load_existing():
    if not os.path.exists("problems.toml"):
        return {}
    data, current = {}, None
    all_lines = open("problems.toml").read().splitlines()
    i = 0
    while i < len(all_lines):
        line = all_lines[i].rstrip()
        m = re.match(r"^\[(\d+)\]$", line)
        if m:
            current = int(m.group(1))
            data[current] = {}
            i += 1
            continue
        if current is None:
            i += 1
            continue
        # Multiline literal string: key = '''
        m = re.match(r"^(\w+)\s*=\s*'''$", line)
        if m:
            key = m.group(1)
            i += 1
            value_lines = []
            while i < len(all_lines):
                cl = all_lines[i]
                if cl.rstrip().endswith("'''"):
                    value_lines.append(cl.rstrip()[:-3])
                    i += 1
                    break
                value_lines.append(cl)
                i += 1
            data[current][key] = '\n'.join(value_lines).strip()
            continue
        # Single-line key = "value" or key = bool
        m = re.match(r'^(\w+)\s*=\s*(.+)$', line)
        if m:
            key, raw = m.group(1), m.group(2).strip()
            if raw == "true":
                data[current][key] = True
            elif raw == "false":
                data[current][key] = False
            else:
                data[current][key] = raw.strip('"')
        i += 1
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
        entry = merged[num]
        lines.append(f"[{num}]")
        lines.append(f'title = "{entry["title"]}"')
        stmt = entry.get("statement", "")
        if stmt:
            safe = stmt.replace("'''", "' ''")
            lines.append(f"statement = '''\n{safe}'''")
        data_url = entry.get("data_url", "")
        if data_url:
            lines.append(f'data_url = "{data_url}"')
        if entry.get("data_embedded"):
            lines.append("data_embedded = true")
        lines.append(f"solved = {str(entry['solved']).lower()}")
        lines.append("")
    with open("problems.toml", "w") as f:
        f.write("\n".join(lines))


# ── fetch titles ─────────────────────────────────────────────────────────────
existing = load_existing()

if MODE == "--all":
    fetched_titles, page = {}, 1
    print("Fetching titles from archives...")
    while True:
        url = ("https://projecteuler.net/archives"
               if page == 1
               else f"https://projecteuler.net/archives;page={page}")
        print(f"  page {page:2d} ... ", end="", flush=True)
        rows = parse_rows(fetch(url))
        new = {n: t for n, t in rows.items() if n not in fetched_titles}
        if not new:
            print("done.")
            break
        fetched_titles.update(new)
        print(f"{len(new)} problems  (total: {len(fetched_titles)})")
        page += 1
        time.sleep(0.5)
else:
    print("Fetching titles from /recent ...", end=" ", flush=True)
    fetched_titles = parse_rows(fetch("https://projecteuler.net/recent"))
    print(f"{len(fetched_titles)} problems")

# ── merge titles into working dict ────────────────────────────────────────────
merged = dict(existing)
for num, title in fetched_titles.items():
    entry = existing.get(num, {})
    merged[num] = {
        "title": title,
        "statement": entry.get("statement", ""),
        "data_url": entry.get("data_url", ""),
        "data_embedded": entry.get("data_embedded", False),
        "solved": entry.get("solved", is_solved(num)),
    }

# ── fetch missing statements + data URLs ─────────────────────────────────────
# --all: fill in every problem missing a statement; check data_url for
#        problems whose statement suggests a downloadable file
# --recent: same but restricted to newly seen problems
pool = merged if MODE == "--all" else fetched_titles
need_stmt = {n for n in pool if not merged[n].get("statement")}
need_data = {n for n in merged
             if not merged[n].get("data_url")
             and "data_url" not in (existing.get(n) or {})
             and likely_has_data_file(merged[n].get("statement", ""))}
need = sorted(need_stmt | need_data)
CHECKPOINT = 50

if need:
    print(f"\nFetching {len(need)} minimal pages (statements: {len(need_stmt)}, data URLs: {len(need_data)})...")
    for i, num in enumerate(need, 1):
        tags = "+".join(filter(None, ["stmt" if num in need_stmt else "",
                                      "data_url" if num in need_data else ""]))
        print(f"  [{i:4d}/{len(need)}] problem {num:4d} [{tags}] ... ", end="", flush=True)
        stmt, data_url = fetch_minimal(num)
        if num in need_stmt and stmt:
            merged[num]["statement"] = stmt
        if num in need_data:
            merged[num]["data_url"] = data_url  # "" if none found
        print("ok" if (stmt or num not in need_stmt) else "failed")
        if i % CHECKPOINT == 0:
            write_toml(merged)
            print(f"  [checkpoint] problems.toml written ({i}/{len(need)})")
        time.sleep(0.5)
else:
    print("All statements and data URLs up to date.")

# ── write ─────────────────────────────────────────────────────────────────────
write_toml(merged)
new_count = sum(1 for n in fetched_titles if n not in existing)
print(f"\nproblems.toml written  ({len(merged)} total, {new_count} new)")
PYEOF
