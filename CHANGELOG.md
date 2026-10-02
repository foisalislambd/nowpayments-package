# Changelog

All notable changes to this monorepo are documented here. Each language package is versioned together at the version below unless a package note says otherwise.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Node: axios 1.20, TypeScript 7.0, and `@types/node` 26. Type declarations are emitted with `tsc` because tsup cannot generate them on TypeScript 7.
- Rust: reqwest 0.13 (with the `query` feature), hmac 0.13, and sha2 0.11, upgraded together so the digest 0.11 stack stays compatible.
- CI: `actions/checkout` v7, `actions/setup-node` v7, `actions/setup-python` v7, and `actions/setup-go` v7.

## 1.0.4 - 2026-10-02

### Added

- Monorepo layout under `packages/` for Node.js, Python, PHP, Go, Ruby, and Rust.
- Shared API docs in `docs/`, including the Postman-derived full reference.
- Project docs for contributing, security reports, and the code of conduct.
- GitHub issue templates, pull request template, and CI for build and test checks.
