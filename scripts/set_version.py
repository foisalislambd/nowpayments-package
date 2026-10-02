#!/usr/bin/env python3
"""Write the release version into each package manifest. VERSION.md is unchanged."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace_first(path: Path, pattern: str, replacement: str) -> None:
    text = path.read_text(encoding="utf-8")
    updated, count = re.subn(pattern, replacement, text, count=1, flags=re.MULTILINE)
    if count != 1:
        sys.exit(f"Could not update version in {path}")
    path.write_text(updated, encoding="utf-8")


def main() -> None:
    if len(sys.argv) != 2 or not re.fullmatch(r"\d+\.\d+\.\d+", sys.argv[1]):
        sys.exit("usage: set_version.py X.Y.Z")
    version = sys.argv[1]

    replace_first(
        ROOT / "packages/node/package.json",
        r'^  "version": "\d+\.\d+\.\d+",\r?$',
        f'  "version": "{version}",',
    )
    replace_first(
        ROOT / "packages/python/pyproject.toml",
        r'^version = "\d+\.\d+\.\d+"\r?$',
        f'version = "{version}"',
    )
    replace_first(
        ROOT / "packages/rust/Cargo.toml",
        r'^version = "\d+\.\d+\.\d+"\r?$',
        f'version = "{version}"',
    )
    replace_first(
        ROOT / "packages/ruby/lib/nowpayments/version.rb",
        r'^  VERSION = "\d+\.\d+\.\d+"\r?$',
        f'  VERSION = "{version}"',
    )
    print(f"Set package versions to {version}")


if __name__ == "__main__":
    main()
