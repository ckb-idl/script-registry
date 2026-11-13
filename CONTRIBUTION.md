# Contributing

CKB Script Registry is a testnet-first project for publishing, discovering, and
independently verifying CKB script interfaces. Contributions to code,
documentation, tests, and integration examples are welcome.

## Discuss and scope the change

Use a GitHub issue to describe a bug or propose a feature. Include the current
behavior, expected outcome, and acceptance criteria. Discuss architectural
changes before building them, particularly changes to package identity,
publication, storage, authentication, and verification claims.

For suspected vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of
posting sensitive details in a public issue.

Keep changes focused on the current milestone. Mainnet deployment, arbitrary
source builds or uploaded-code execution, a registry contract, and package
dependency resolution are outside the initial project scope.

## Development setup

Requirements: Rust stable and Docker with Compose v2. Follow the
[README](README.md) for configuration and optional API container instructions.
From the repository root:

```sh
docker compose up -d --wait
cargo run --manifest-path server/Cargo.toml --locked
```

The default Compose stack starts only PostgreSQL. The API runs locally and uses
the `object_store` filesystem backend at `.local/object-storage/`. An `.env` file
is optional for the current defaults. Never commit `.env`, credentials, database
files, or local artifact data.

## Branches and pull requests

The branch flow is:

```text
feature/* or fix/* -> dev -> staging -> master
```

- Create a focused feature or fix branch from the latest `dev`. Implement and
  commit the change on that branch, not directly on `dev`.
- Open a pull request from the feature/fix branch into `dev` for review and
  integration. Keep individual changes scoped to their issues.
- Promote a reviewed batch of integrated changes from `dev` into `staging`.
  Use `staging` to validate the combined update before release.
- Promote a validated batch from `staging` into `master`. Release tags identify
  reviewed commits on `master`; do not bypass staging with direct dev-to-master
  release merges.
- Link the relevant issue, explain the resulting behavior, and describe how the
  change was verified. State any remaining limitations.

For the current M0 foundation work, the maintainer has authorized the remaining
setup and documentation updates directly on `dev`. Once new feature or fix work
begins, use the separate-branch workflow above.

Keep dependency additions limited to functionality the change actually uses.
Commit `server/Cargo.lock` when dependencies change. Document new configuration,
API behavior, and storage or migration requirements alongside the code.

## Validation

Run from the repository root before submitting:

```sh
cargo fmt --manifest-path server/Cargo.toml --all -- --check
cargo test --manifest-path server/Cargo.toml --locked
cargo build --manifest-path server/Cargo.toml --locked
docker compose config --quiet
```

Add regression tests for behavior changes and fixes. Where applicable, cover
malformed input, failure paths, resource limits, and persistence. For Docker
changes, also verify the affected image or service starts and passes its health
check. Document environment-specific checks that could not run.

CI checks formatting, tests, build, and Compose configuration on pushes to
`dev` and `master` and on pull requests targeting those branches. Passing CI is
required for a change to be considered ready for review; maintainers review and
merge pull requests. Staging batches must also be validated before promotion;
the current CI trigger configuration covers `dev` and `master`, not `staging`.

## Registry design expectations

- Scripts may be registered without an IDL artifact.
- Deployment evidence, cryptographic IDL binding, publisher attestation, source
  availability, and audits are separate claims. Do not collapse them into an
  ambiguous verified badge.
- Clients must be able to independently verify interface commitments and the
  relevant chain evidence. A registry listing is not an audit or endorsement.
- Publication should preserve immutable releases and content-addressed artifacts
  as those features are implemented.
- Treat publisher metadata, uploaded artifacts, and RPC responses as untrusted
  input. Do not execute uploaded code or fetch arbitrary URLs without explicit
  design review and appropriate controls.

## License

Contributions are submitted under the project's [MIT License](LICENSE). Preserve
applicable third-party license notices when reusing code or other materials.
