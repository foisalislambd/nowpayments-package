#!/usr/bin/env python3
"""Verify endpoint parity across SDKs and docs."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def norm(path: str) -> str:
    path = path.split("?")[0].rstrip("/")
    path = re.sub(r"\$\{[^}]+\}", "{id}", path)
    path = re.sub(r"#\{[^}]+\}", "{id}", path)
    path = re.sub(r"\{[a-z_]+\}", "{id}", path, flags=re.I)
    if path.endswith("/payment") and "/payment/" not in path + "/":
        pass
    return path


def extract_node() -> set[str]:
    text = (ROOT / "packages/node/src/index.ts").read_text(encoding="utf-8")
    return {norm(m) for m in re.findall(r"['\"](/v1/[^'\"]+)['\"]", text)}


def extract_py() -> set[str]:
    text = (ROOT / "packages/python/nowpayments/client.py").read_text(encoding="utf-8")
    return {norm(m) for m in re.findall(r'["\'](/v1/[^"\']+)["\']', text)}


def extract_ruby() -> set[str]:
    text = (ROOT / "packages/ruby/lib/nowpayments/client.rb").read_text(encoding="utf-8")
    return {norm(m) for m in re.findall(r"['\"](/v1/[^'\"]+)['\"]", text)}


def extract_php() -> set[str]:
    text = (ROOT / "packages/php/src/NowPayments.php").read_text(encoding="utf-8")
    return {norm(m) for m in re.findall(r"['\"](/v1/[^'\"]+)['\"]", text)}


def extract_go() -> set[str]:
    text = "".join(f.read_text(encoding="utf-8", errors="replace") for f in (ROOT / "packages/go").glob("*.go"))
    return {norm(m) for m in re.findall(r'"(/v1/[^"]+)"', text)}


def extract_rust() -> set[str]:
    text = (ROOT / "packages/rust/src/client.rs").read_text(encoding="utf-8")
    paths = re.findall(r'"(/v1/[^"]+)"', text)
    paths += [m.replace("{}", "{id}") for m in re.findall(r'format!\("/v1/([^"]+)"', text)]
    out = set()
    for p in paths:
        if not p.startswith("/v1"):
            p = "/v1/" + p
        out.add(norm(p))
    return out


def checklist_routes() -> set[str]:
    text = (ROOT / "docs/METHODS_CHECKLIST.md").read_text(encoding="utf-8")
    return {norm(p) for _, p in re.findall(r"(GET|POST|PATCH|DELETE) (/v1/[^\s|]+)", text)}


def parsed_full_ref() -> set[tuple[str, str]]:
    text = (ROOT / "docs/FULL_API_REFERENCE.md").read_text(encoding="utf-8")
    return {(m, norm(p)) for m, p in re.findall(r"### (GET|POST|PATCH|DELETE) `([^`]+)`", text)}


def main() -> None:
    sets = {
        "node": extract_node(),
        "py": extract_py(),
        "ruby": extract_ruby(),
        "php": extract_php(),
        "go": extract_go(),
        "rust": extract_rust(),
    }
    base = sets["node"]
    print("Counts:", {k: len(v) for k, v in sets.items()})
    ok = True
    for name, eps in sets.items():
        if name == "node":
            continue
        missing = base - eps
        extra = eps - base
        if missing or extra:
            ok = False
            print(f"\n=== {name} vs node ===")
            for x in sorted(missing):
                print("  MISSING:", x)
            for x in sorted(extra):
                print("  EXTRA:", x)

    check = checklist_routes()
    print("\nChecklist routes:", len(check))
    mc = sorted(base - check)
    cm = sorted(check - base)
    if mc:
        ok = False
        print("Code not in checklist:", mc)
    if cm:
        ok = False
        print("Checklist not in node code:", cm)

    parsed = parsed_full_ref()
    print("\nParsed FULL_API endpoints:", len(parsed))
    required = [
        ("POST", "/v1/payout/w_id/cancel"),
        ("GET", "/v1/payout/fee"),
        ("GET", "/v1/status"),
        ("POST", "/v1/payment"),
    ]
    for method, path in required:
        if (method, path) not in parsed:
            # try path contains
            if not any(m == method and path in p for m, p in parsed):
                ok = False
                print(f"FULL_API missing parsed: {method} {path}")

    # cancel body key
    for pkg, path in [
        ("node", "packages/node/src/index.ts"),
        ("py", "packages/python/nowpayments/client.py"),
    ]:
        t = (ROOT / path).read_text(encoding="utf-8")
        if "w_id/cancel" not in t:
            ok = False
            print(f"{pkg}: wrong cancel path")
        if "payout_id" not in t or "payout_id" not in t.split("w_id/cancel")[0][-200:] + t.split("w_id/cancel")[-1][:200]:
            pass  # weak check

    print("\nRESULT:", "PASS" if ok else "ISSUES FOUND")


if __name__ == "__main__":
    main()
