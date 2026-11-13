# Security policy

## Current status and support

CKB Script Registry is an early development, testnet-first project. There is no
stable supported release or formal third-party security audit yet. Security fixes
are implemented on separate fix branches and reviewed into `dev`, then validated
through `staging` before promotion to `master`. Coordinate private reports before
publishing sensitive fix details. Older development snapshots do not have a
maintenance guarantee.

The current scaffold provides an API health endpoint, a local filesystem object
store, and a PostgreSQL development container. Publisher authentication, artifact
publication APIs, database integration, and on-chain verification are not yet
implemented. This setup is intended for local development, not public deployment.

## Reporting a vulnerability

Use GitHub's private
[Report a vulnerability](https://github.com/ckb-idl/script-registry/security/advisories/new)
flow when private vulnerability reporting is enabled for this repository.

Private reporting is currently disabled and no dedicated security email has been
published. Until a private channel is available, open a minimal public issue
titled "Request for private security contact" so a maintainer can arrange one.
Do not include exploit details, credentials, sensitive logs, or affected private
data in that issue. Maintainers should enable private reporting or publish a
confidential contact before operating a public service.

In the private report, include:

- Affected commit/release, component, and relevant environment details.
- Expected behavior, actual behavior, and potential impact.
- Minimal reproduction steps using synthetic data and redacted logs.
- A suggested mitigation, if known, and a way to contact you privately.

There is no guaranteed response time or bounty program at this stage. Please
coordinate disclosure with the maintainer while a report is investigated and
remediated. If a credential is exposed, revoke or rotate it through its provider;
removing it from a file or Git history does not invalidate it.

## Current security boundaries

- Compose credentials are local development defaults. Do not reuse them for a
  deployed service. Published Docker ports bind to loopback.
- `/health` reports process health; it does not prove database readiness or
  successful on-chain verification.
- The object store uses the server process's filesystem permissions. Its root
  must be operator-controlled. It is not a filesystem sandbox, and the local
  backend may follow symlinks. Untrusted users must not modify the storage root
  or introduce links into it.
- Keep `.env`, credentials, database data, and local artifacts out of Git and
  logs. Back up persistent data before destructive resets or storage changes.
- Report actual vulnerabilities in this code or its deployment, including path
  traversal, unintended data access, secret disclosure, or resource exhaustion.
  For dependency findings, identify the affected usage and coordinate with the
  upstream project when appropriate.

## Registry trust model

As publication and verification features are implemented, deployment evidence,
cryptographic IDL binding, and publisher attestation must remain separate claims.
A listing or an identified publisher is not an audit or endorsement. A matching
IDL commitment establishes a relationship to executable bytes; it does not prove
that the script behaves correctly or is safe.

Consumers must retain independent verification of chain facts and commitments.
RPC failure or stale evidence must not silently become successful verification.
The initial scope does not execute uploaded scripts or rebuild arbitrary source.
Testnet operation does not establish mainnet readiness.

This policy should be updated as authentication, publication, verification, and
public deployments are introduced.
