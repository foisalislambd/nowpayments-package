#!/usr/bin/env python3
"""Decide whether VERSION.md is newer than the latest vX.Y.Z git tag."""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION_LINE = re.compile(r"\d+\.\d+\.\d+")
TAG = re.compile(r"^v(\d+)\.(\d+)\.(\d+)$")


def read_version() -> tuple[int, int, int]:
    text = (ROOT / "VERSION.md").read_text(encoding="utf-8")
    for line in text.splitlines():
        stripped = line.strip()
        if VERSION_LINE.fullmatch(stripped):
            return tuple(int(part) for part in stripped.split("."))  # type: ignore[return-value]
    sys.exit("VERSION.md has no line that is only a major.minor.patch version")


def latest_tag() -> tuple[int, int, int] | None:
    output = subprocess.check_output(
        ["git", "tag", "--list", "v*"],
        cwd=ROOT,
        text=True,
        encoding="utf-8",
    )
    found: list[tuple[int, int, int]] = []
    for line in output.splitlines():
        match = TAG.fullmatch(line.strip())
        if match:
            found.append(tuple(int(part) for part in match.groups()))  # type: ignore[arg-type]
    return max(found) if found else None


def emit(name: str, value: str) -> None:
    print(f"{name}={value}")
    path = os.environ.get("GITHUB_OUTPUT")
    if path:
        with open(path, "a", encoding="utf-8") as handle:
            handle.write(f"{name}={value}\n")


def main() -> None:
    version = read_version()
    current = latest_tag()
    version_text = ".".join(str(part) for part in version)
    if current is not None and version <= current:
        current_text = ".".join(str(part) for part in current)
        print(f"No release: {version_text} is not newer than v{current_text}")
        emit("release", "false")
        emit("version", version_text)
        return
    print(f"Release {version_text}")
    emit("release", "true")
    emit("version", version_text)


if __name__ == "__main__":
    main()
