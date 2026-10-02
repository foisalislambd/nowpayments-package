<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-22c55e?style=for-the-badge" alt="MIT license" />
  <img src="https://img.shields.io/badge/SDKs-6-6366f1?style=for-the-badge" alt="Six SDKs" />
  <img src="https://img.shields.io/badge/version-1.0.4-0ea5e9?style=for-the-badge" alt="version 1.0.4" />
</p>

<h1 align="center">NOWPayments SDKs</h1>

<p align="center">
  Unofficial client libraries for the <a href="https://nowpayments.io">NOWPayments</a> cryptocurrency payment API.<br/>
  Payments, invoices, payouts, subscriptions, custody, and IPN webhooks — one monorepo, six languages.
</p>

<p align="center">
  <a href="#packages">Packages</a> ·
  <a href="#features">Features</a> ·
  <a href="#documentation">Documentation</a> ·
  <a href="#development">Development</a> ·
  <a href="#contributing">Contributing</a>
</p>

---

## Packages

| Language | Package | Path | Install |
|----------|---------|------|---------|
| Node.js / TypeScript | [`nowpayments-node`](https://www.npmjs.com/package/nowpayments-node) | [`packages/node`](./packages/node) | `npm install nowpayments-node` |
| Python | [`nowpayments-py`](https://pypi.org/project/nowpayments-py/) | [`packages/python`](./packages/python) | `pip install nowpayments-py` |
| PHP | [`foisalislambd/nowpayments-php`](https://packagist.org/packages/foisalislambd/nowpayments-php) | [`packages/php`](./packages/php) | `composer require foisalislambd/nowpayments-php` |
| Go | `github.com/foisalislambd/nowpayments-go` | [`packages/go`](./packages/go) | `go get github.com/foisalislambd/nowpayments-go` |
| Ruby | [`nowpayments-ruby`](https://rubygems.org/gems/nowpayments-ruby) | [`packages/ruby`](./packages/ruby) | `gem install nowpayments-ruby` |
| Rust | [`nowpayments-rust`](https://crates.io/crates/nowpayments-rust) | [`packages/rust`](./packages/rust) | `cargo add nowpayments-rust` |

Each package has its own README, examples, and publish metadata. Shared API docs live once, at the repository root.

## Features

| Area | What you can do |
|------|-----------------|
| Payments | Create a payment, check status, list payments, update the estimate |
| Invoices | Create an invoice and send the customer to the hosted page |
| Payouts | Mass payout, 2FA verify, cancel a scheduled payout, fiat payouts |
| Subscriptions | Plans, recurring payments, cancellation |
| Custody | Sub-partners, transfers, deposits, write-offs |
| Conversions | Convert balances held in custody |
| IPN | Verify webhook signatures with HMAC-SHA512 |
| Helpers | Status labels and “is this payment finished?” checks |

API keys come from the [NOWPayments dashboard](https://account.nowpayments.io). Use the sandbox base URL while you are testing.

## Documentation

| Document | What it covers |
|----------|----------------|
| [docs/README.md](./docs/README.md) | Where to start |
| [docs/FULL_API_REFERENCE.md](./docs/FULL_API_REFERENCE.md) | Endpoints collected from the Postman exports |
| [docs/API_DOCUMENTATION.md](./docs/API_DOCUMENTATION.md) | Payments, auth, and IPN notes |
| [docs/METHODS_CHECKLIST.md](./docs/METHODS_CHECKLIST.md) | API route mapped to SDK methods |

Official NOWPayments references:

- [Postman — production](https://documenter.getpostman.com/view/7907941/2s93JusNJt)
- [Postman — sandbox](https://documenter.getpostman.com/view/7907941/T1LSCRHC)
- [Help center](https://nowpayments.io/help/payments/api)

## Development

```text
.
├── docs/            Shared API reference
├── packages/
│   ├── node/
│   ├── python/
│   ├── php/
│   ├── go/
│   ├── ruby/
│   └── rust/
├── scripts/         Doc generation and endpoint parity check
├── CONTRIBUTING.md
├── LICENSE
└── README.md
```

```bash
cd packages/node && npm install && npm run build
cd packages/python && pip install -e .
cd packages/php && composer install
cd packages/go && go test ./...
cd packages/ruby && bundle install
cd packages/rust && cargo test
```

Rebuild the full API reference from the raw Postman exports, then compare endpoint coverage across SDKs:

```bash
python scripts/format_api_docs.py
python scripts/verify_sdk_parity.py
```

## Releases

The version for every SDK is in [VERSION.md](./VERSION.md). Push a higher version to `main`. After CI passes, GitHub creates tag `vX.Y.Z` and publishes npm, PyPI, crates.io, and RubyGems with Trusted Publishing. Packagist and the Go module are published with a git tag, because those registries do not accept a Trusted Publisher upload from this monorepo.

## Contributing

Issues and pull requests are welcome. Read [CONTRIBUTING.md](./CONTRIBUTING.md) before you open one.

- Security reports: [SECURITY.md](./SECURITY.md)
- Community standards: [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
- Release notes: [CHANGELOG.md](./CHANGELOG.md)

## License

[MIT](./LICENSE) © [Foisalislambd](https://github.com/Foisalislambd)

NOWPayments is a trademark of its owner. This repository is an independent client SDK and is not affiliated with NOWPayments.
