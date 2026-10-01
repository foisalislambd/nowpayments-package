# Security policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 1.0.x   | Yes       |

Security fixes are published as a new patch release of the affected package.

## Reporting a vulnerability

Please do not open a public GitHub issue for a security problem.

Use **Security → Report a vulnerability** on this repository (private vulnerability reporting). Include:

- The package and version (`nowpayments-node`, `nowpayments-py`, `nowpayments-php`, `nowpayments-go`, `nowpayments-ruby`, or `nowpayments-rust`)
- What an attacker can do
- A minimal way to reproduce it, if you have one

If private reporting is unavailable, contact the maintainer through [GitHub](https://github.com/Foisalislambd).

You should hear back within 7 days. Please give the maintainer a reasonable window to ship a fix before any public write-up.

## What to know about these SDKs

API keys and IPN secrets belong in environment variables or a secret store, never in source control. IPN verification must use the raw request body and a timing-safe compare. A bug in signature verification is in scope for this policy. NOWPayments account, dashboard, or API-availability problems should go to NOWPayments support, not this repository.
