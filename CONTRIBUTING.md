# Contributing

Thanks for helping improve the NOWPayments SDKs. This repository holds six clients that should stay in step with the same API.

## Before you start

- Search [open issues](../../issues) so the work is not already underway.
- For a security problem, follow [SECURITY.md](./SECURITY.md). Do not file a public issue.
- Small docs and typo fixes can go straight to a pull request.

## Local setup

Clone the repository, then work inside the package you are changing:

```bash
cd packages/node && npm install && npm run build
cd packages/python && pip install -e .
cd packages/php && composer install
cd packages/go && go test ./...
cd packages/ruby && bundle install && gem build nowpayments.gemspec
cd packages/rust && cargo test
```

You only need the toolchain for the language you touch. Node.js 18+, Python 3.9+, PHP 8.1+, Go 1.23+, Ruby 3.1+, and a recent stable Rust toolchain match the published packages.

## Changing an API method

When you add or change an endpoint, update every SDK that already implements that area, plus the shared docs:

1. Implement the method in `packages/node`, `packages/python`, `packages/php`, `packages/go`, `packages/ruby`, and `packages/rust`.
2. Keep request paths, field names, and IPN signing behavior aligned with the [Postman docs](https://documenter.getpostman.com/view/7907941/2s93JusNJt).
3. Add or adjust an example under that package’s `examples/` folder when the call is user-facing.
4. Update `docs/METHODS_CHECKLIST.md` if the route list changed.
5. Run:

```bash
python scripts/verify_sdk_parity.py
```

Raw Postman exports live in `docs/raw/` and should stay unedited. To regenerate `docs/FULL_API_REFERENCE.md`:

```bash
python scripts/format_api_docs.py
```

## Pull requests

- One change per pull request when you can.
- Describe what changed and how you checked it.
- Do not commit secrets, `.env` files, `node_modules`, `vendor`, `target`, or built `dist` output.
- Match the style already used in the package you edit.

## Releases

Package versions are currently `1.0.4` and are published from each package directory to npm, PyPI, Packagist, RubyGems, crates.io, and the Go module proxy. Version bumps belong in that package’s manifest (`package.json`, `pyproject.toml`, `composer.json`, `nowpayments.gemspec` / `version.rb`, `Cargo.toml`) and in [CHANGELOG.md](./CHANGELOG.md).
