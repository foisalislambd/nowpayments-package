# Version

1.0.5

This number is the release version for every SDK in this repository. Do not put another bare `major.minor.patch` line above it. The release workflow reads the first line that is only a version.

## When a release runs

A push to `main` runs CI. After CI succeeds, the release workflow reads this file and compares it with the newest `v` tag on this repository, using semantic version order.

- A higher version creates GitHub release `vX.Y.Z` and publishes the packages.
- The same version, or a lower one, does nothing.

Registries that already have that exact version are left as they are, so a retry does not fail on a package that was already uploaded.

## Where each package is published

| Package | Registry | How |
|---------|----------|-----|
| `packages/node` | [npm](https://www.npmjs.com/package/nowpayments-node) and [GitHub Packages](https://github.com/foisalislambd/nowpayments-package/pkgs/npm/nowpayments-node) | npmjs via Trusted Publishing. GitHub Packages as `@foisalislambd/nowpayments-node` |
| `packages/python` | [PyPI](https://pypi.org/project/nowpayments-py/) | Trusted Publishing (GitHub OIDC). GitHub Packages has no Python registry |
| `packages/rust` | [crates.io](https://crates.io/crates/nowpayments-rust) | Trusted Publishing (GitHub OIDC). GitHub Packages has no Cargo registry |
| `packages/ruby` | [RubyGems](https://rubygems.org/gems/nowpayments-ruby) and GitHub Packages | RubyGems via Trusted Publishing, then the same gem to `rubygems.pkg.github.com` |
| `packages/php` | [Packagist](https://packagist.org/packages/foisalislambd/nowpayments-php) | Git tag on `nowpayments-php` (Packagist has no public OIDC publish) |
| `packages/go` | [pkg.go.dev](https://pkg.go.dev/github.com/foisalislambd/nowpayments-go) | Git tag `vX.Y.Z` on `nowpayments-go` |

Trusted publishers must use workflow file `.github/workflows/release.yml` and must leave the GitHub environment empty.

One-time setup on each site, for repository `foisalislambd/nowpayments-package`:

- npm: package **nowpayments-node** → Settings → Trusted Publisher
- PyPI: project **nowpayments-py** → Publishing → Add a new pending publisher
- crates.io: crate **nowpayments-rust** → Settings → Trusted Publishing. The crate has to exist before the first OIDC publish (`cargo login` and `cargo publish` once from your machine if it does not).
- RubyGems: gem **nowpayments-ruby** → Trusted publishers

Packagist and Go read a git repository, not an uploaded archive. Public Packagist cannot install a package from a subdirectory, and the Go module path is `github.com/foisalislambd/nowpayments-go`. The workflow therefore copies `packages/php` to `github.com/foisalislambd/nowpayments-php` and `packages/go` to `github.com/foisalislambd/nowpayments-go`, commits on top of `main`, and pushes tag `vX.Y.Z`.

Add this repository secret:

- `MIRROR_GITHUB_TOKEN` — a fine-grained personal access token that can write contents on `nowpayments-php` and `nowpayments-go`

Optional, only to ask Packagist to update immediately. The GitHub hook on `nowpayments-php` also picks up the new tag.

- `PACKAGIST_USERNAME`
- `PACKAGIST_TOKEN`
