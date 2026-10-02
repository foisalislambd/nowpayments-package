#!/usr/bin/env python3
"""Publish a package directory by committing it onto another GitHub repo and tagging it.

Packagist and the Go module proxy publish from a repository tag. They do not
accept a Trusted Publisher upload from this monorepo.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {".git", "vendor", "target", "node_modules", "__pycache__"}


def run(args: list[str], cwd: Path, env: dict[str, str] | None = None) -> str:
    result = subprocess.run(
        args,
        cwd=cwd,
        text=True,
        encoding="utf-8",
        capture_output=True,
        env=env,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stdout)
        sys.stderr.write(result.stderr)
        sys.exit(result.returncode)
    return result.stdout


def copy_package(source: Path, destination: Path) -> None:
    files = [
        path
        for path in destination.rglob("*")
        if path.is_file() and ".git" not in path.parts
    ]
    for path in files:
        path.unlink()
    for dirpath, dirnames, filenames in os.walk(source):
        dirnames[:] = [name for name in dirnames if name not in SKIP_DIRS]
        relative = Path(dirpath).relative_to(source)
        target_dir = destination if relative == Path(".") else destination / relative
        target_dir.mkdir(parents=True, exist_ok=True)
        for filename in filenames:
            shutil.copy2(Path(dirpath) / filename, target_dir / filename)
    license_file = ROOT / "LICENSE"
    if license_file.is_file() and not (destination / "LICENSE").is_file():
        shutil.copy2(license_file, destination / "LICENSE")


def main() -> None:
    if len(sys.argv) != 4:
        sys.exit("usage: mirror_release.py SOURCE_DIR OWNER/REPO X.Y.Z")
    source = (ROOT / sys.argv[1]).resolve()
    repo = sys.argv[2]
    version = sys.argv[3]
    token = os.environ.get("MIRROR_GITHUB_TOKEN", "").strip()
    if not token:
        sys.exit(
            "MIRROR_GITHUB_TOKEN is not set. Add a fine-grained token that can "
            "write nowpayments-php and nowpayments-go."
        )
    if not source.is_dir():
        sys.exit(f"Missing package directory: {source}")

    remote = f"https://x-access-token:{token}@github.com/{repo}.git"
    with tempfile.TemporaryDirectory(prefix="nowpayments-mirror-") as tmp:
        dest = Path(tmp) / "repo"
        cloned = subprocess.run(
            ["git", "clone", "--depth", "1", "--branch", "main", remote, str(dest)],
            text=True,
        )
        if cloned.returncode != 0:
            if dest.exists():
                shutil.rmtree(dest)
            run(["git", "clone", "--depth", "1", "--branch", "master", remote, str(dest)])

        wanted = f"refs/tags/v{version}"
        existing = run(["git", "ls-remote", "--tags", "origin"], dest)
        for line in existing.splitlines():
            ref = line.split()[-1] if line.split() else ""
            if ref in {wanted, f"{wanted}^{{}}"}:
                print(f"v{version} already exists on {repo}")
                return

        copy_package(source, dest)
        run(["git", "add", "-A"], dest)
        dirty = subprocess.run(["git", "diff", "--cached", "--quiet"], cwd=dest)
        if dirty.returncode != 0:
            run(
                [
                    "git",
                    "-c",
                    "user.name=github-actions[bot]",
                    "-c",
                    "user.email=41898282+github-actions[bot]@users.noreply.github.com",
                    "commit",
                    "-m",
                    f"Release v{version}",
                ],
                dest,
            )
            run(["git", "push", "origin", "HEAD"], dest)
        run(["git", "tag", f"v{version}"], dest)
        run(["git", "push", "origin", f"refs/tags/v{version}"], dest)
        print(f"Pushed v{version} to {repo}")


if __name__ == "__main__":
    main()
