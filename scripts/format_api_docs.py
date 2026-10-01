#!/usr/bin/env python3
"""Format Postman-export raw docs into structured Markdown."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]  # repository root
DOCS = ROOT / "docs"
RAW_DIR = DOCS / "raw"
RAW_FILES = ["copied-docs.md", "copied-docs-2.md", "copied-docs-3.md"]

METHODS = {"GET", "POST", "PUT", "PATCH", "DELETE"}
API_URL_RE = re.compile(
    r"^https://api(?:-sandbox)?\.nowpayments\.io(/v1/[^\s?]+)",
    re.IGNORECASE,
)
SKIP_LINE_RE = re.compile(
    r"^(Public|ENVIRONMENT|LAYOUT|LANGUAGE|Example Request|Example Response|Body|Headers|View More|curl|json|Text|raw \(json\)|\d{3}(\s|$)|--header|--location|--data)",
    re.IGNORECASE,
)


def normalize_path(url_path: str) -> str:
    path = url_path.split("?")[0].rstrip("/")
    path = re.sub(r":\w+", "{id}", path)
    path = re.sub(r"<[^>]+>", "{id}", path)
    return path


def parse_raw_file(path: Path) -> list[dict]:
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    endpoints: list[dict] = []
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        if line in METHODS:
            method = line
            title_parts: list[str] = []
            url = ""
            path_only = ""
            j = i + 1
            while j < len(lines) and j < i + 8:
                candidate = lines[j].strip()
                m = API_URL_RE.match(candidate)
                if m:
                    url = m.group(0)
                    path_only = m.group(1)
                    break
                if candidate and candidate not in METHODS and not SKIP_LINE_RE.match(candidate):
                    if not candidate.startswith("http") and len(candidate) < 120:
                        title_parts.append(candidate)
                j += 1
            if url and path_only:
                block: list[str] = []
                k = j + 1
                while k < len(lines) and k < j + 80:
                    bl = lines[k].rstrip()
                    bs = bl.strip()
                    if bs in METHODS and k > j + 3:
                        break
                    if API_URL_RE.match(bs) and k > j + 2:
                        break
                    if bs:
                        block.append(bl)
                    k += 1
                endpoints.append(
                    {
                        "method": method,
                        "title": " ".join(title_parts[:2]) or normalize_path(url),
                        "url": url,
                        "path": normalize_path(path_only),
                        "details": block,
                        "source": path.name,
                    }
                )
                i = k
                continue
        i += 1
    return endpoints


def dedupe_endpoints(items: list[dict]) -> list[dict]:
    seen: dict[tuple[str, str], dict] = {}
    for ep in items:
        key = (ep["method"], ep["path"])
        if key not in seen or len(ep["details"]) > len(seen[key]["details"]):
            seen[key] = ep
    return sorted(seen.values(), key=lambda e: (e["path"], e["method"]))


def format_endpoint(ep: dict) -> str:
    path = ep["path"]
    query = ""
    if "?" in ep["url"]:
        query = ep["url"].split("?", 1)[1]

    out = [f"### {ep['method']} `{path}`", ""]
    if ep["title"] and ep["title"] != path:
        out.append(f"**{ep['title']}**")
        out.append("")
    out.append(f"- **URL:** `{ep['url'].split('?')[0]}`")
    if query:
        out.append(f"- **Query:** `{query}`")
    out.append(f"- **Source:** `{ep['source']}`")
    out.append("")

    details = []
    for line in ep["details"]:
        s = line.strip()
        if SKIP_LINE_RE.match(s):
            continue
        if s in METHODS:
            break
        if API_URL_RE.match(s):
            break
        if s.startswith("{") and details and details[-1].strip().endswith("{"):
            details.append(line)
            continue
        if len(s) > 200 and not s.startswith("{"):
            continue
        details.append(line)

    if details:
        out.append("<details>")
        out.append("<summary>Postman export details</summary>")
        out.append("")
        out.append("```")
        out.extend(details[:60])
        if len(details) > 60:
            out.append("... (truncated)")
        out.append("```")
        out.append("")
        out.append("</details>")
        out.append("")

    return "\n".join(out)


def group_by_category(endpoints: list[dict]) -> dict[str, list[dict]]:
    categories: dict[str, list[dict]] = {}
    rules = [
        ("/v1/status", "Auth & status"),
        ("/v1/auth", "Auth & status"),
        ("/v1/currencies", "Currencies"),
        ("/v1/full-currencies", "Currencies"),
        ("/v1/merchant/coins", "Currencies"),
        ("/v1/estimate", "Payments"),
        ("/v1/min-amount", "Payments"),
        ("/v1/payment", "Payments"),
        ("/v1/invoice", "Invoices"),
        ("/v1/payout", "Payouts"),
        ("/v1/balance", "Balance"),
        ("/v1/fiat-payouts", "Fiat payouts"),
        ("/v1/sub-partner", "Custody & sub-partners"),
        ("/v1/subscriptions", "Subscriptions"),
        ("/v1/conversion", "Conversions"),
    ]
    for ep in endpoints:
        cat = "Other"
        for prefix, name in rules:
            if ep["path"].startswith(prefix):
                cat = name
                break
        categories.setdefault(cat, []).append(ep)
    return categories


def build_full_reference(endpoints: list[dict]) -> str:
    api_doc = (DOCS / "API_DOCUMENTATION.md").read_text(encoding="utf-8")
    # Use overview only (stop before first endpoint section)
    overview = api_doc
    marker = "## Payment Endpoints"
    if marker in overview:
        overview = overview.split(marker, 1)[0].strip()
    overview = re.sub(r"^# .+\n+", "", overview, count=1)
    methods = (DOCS / "METHODS_CHECKLIST.md").read_text(encoding="utf-8")
    methods = re.sub(
        r"^# NOWPayments API – Methods Checklist\s*\n+",
        "",
        methods,
        count=1,
    )
    methods = re.sub(r"^## ", "### ", methods, flags=re.MULTILINE)

    parts = [
        "# NOWPayments API — Full Reference",
        "",
        "> Consolidated from official Postman exports (`docs/raw/`) and package docs.",
        "> Production: `https://api.nowpayments.io` · Sandbox: `https://api-sandbox.nowpayments.io`",
        "",
        "## Table of contents",
        "",
        "1. [Overview & authentication](#overview--authentication)",
        "2. [SDK method checklist](#sdk-method-checklist)",
        "3. [Endpoint reference (Postman)](#endpoint-reference-postman)",
        "4. [IPN (webhooks)](#ipn-webhooks)",
        "5. [Links](#links)",
        "",
        "---",
        "",
        "## Overview & authentication",
        "",
        overview,
        "",
        "**Headers (typical):**",
        "",
        "| Header | When |",
        "|--------|------|",
        "| `x-api-key` | Almost all requests |",
        "| `Content-Type: application/json` | POST/PATCH bodies |",
        "| `Authorization: Bearer <jwt>` | Payouts, custody, some lists (JWT from `POST /v1/auth`, expires in 5 min) |",
        "",
        "---",
        "",
        "## SDK method checklist",
        "",
        "> Maps each API route to `nowpayments-node` methods (same coverage in other SDKs).",
        "",
        methods,
        "",
        "---",
        "",
        "## Endpoint reference (Postman)",
        "",
        f"Parsed **{len(endpoints)}** unique endpoints from raw exports.",
        "",
    ]

    for cat, eps in sorted(group_by_category(endpoints).items()):
        parts.append(f"## {cat}")
        parts.append("")
        for ep in eps:
            parts.append(format_endpoint(ep))

    parts.extend(
        [
            "---",
            "",
            "## IPN (webhooks)",
            "",
            "1. Set `ipn_callback_url` when creating a payment or invoice.",
            "2. Save **IPN Secret** from Dashboard → Store Settings (shown once at creation).",
            "3. On callback: recursively **sort JSON keys**, `JSON.stringify`, HMAC-SHA512 with IPN secret.",
            "4. Compare with header `x-nowpayments-sig` (timing-safe).",
            "5. **Use the raw HTTP body** for verification when possible (parsed frameworks may change types).",
            "",
            "**Payment statuses:** `waiting`, `confirming`, `confirmed`, `spending`, `sending`, `partially_paid`, `finished`, `failed`, `refunded`, `expired`",
            "",
            "---",
            "",
            "## Links",
            "",
            "- [Postman — Production](https://documenter.getpostman.com/view/7907941/2s93JusNJt)",
            "- [Postman — Sandbox](https://documenter.getpostman.com/view/7907941/T1LSCRHC)",
            "- [Help Center — API](https://nowpayments.io/help/payments/api)",
            "- [Zendesk — Endpoints](https://nowpayments.zendesk.com/hc/en-us/articles/21345824322717-API-and-endpoint-description)",
            "",
        ]
    )
    return "\n".join(parts)


def main() -> None:
    all_eps: list[dict] = []
    for name in RAW_FILES:
        path = RAW_DIR / name
        if path.exists():
            all_eps.extend(parse_raw_file(path))

    endpoints = dedupe_endpoints(all_eps)
    # Documented in Zendesk / METHODS_CHECKLIST but absent from Postman raw exports
    fee_key = ("GET", "/v1/payout/fee")
    if fee_key not in {(e["method"], e["path"]) for e in endpoints}:
        endpoints.append(
            {
                "method": "GET",
                "title": "Get payout fee",
                "url": "https://api.nowpayments.io/v1/payout/fee",
                "path": "/v1/payout/fee",
                "details": [
                    "Query: currency (required), amount (required)",
                    "Estimates network fee for a payout.",
                ],
                "source": "METHODS_CHECKLIST.md (Zendesk API docs)",
            }
        )
        endpoints = sorted(endpoints, key=lambda e: (e["path"], e["method"]))

    (DOCS / "FULL_API_REFERENCE.md").write_text(
        build_full_reference(endpoints), encoding="utf-8"
    )
    print(f"Wrote FULL_API_REFERENCE.md ({len(endpoints)} endpoints)")


if __name__ == "__main__":
    main()
