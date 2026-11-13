# CKB Script Registry

Testnet-first infrastructure for publishing, discovering, and independently
verifying CKB script interfaces. The current scaffold exposes `GET /health`
and initializes a local filesystem object store.

## Local development

Requirements: Rust stable and Docker Engine/Desktop with Docker Compose v2.
Run these commands from the repository root:

```sh
docker compose up -d --wait
cargo run --manifest-path server/Cargo.toml --locked
```

In another terminal:

```sh
curl --fail http://localhost:3000/health
```

Compose starts **only PostgreSQL 17 Alpine**, published at `127.0.0.1:5432`,
with a readiness check and persistent Docker volume. The API runs directly with
Cargo, so ordinary development does not build a Rust container image.
Redis, MinIO, AWS CLI, and a Go toolchain are not required.

Environment configuration is optional at this stage. To customize it later,
copy `.env.example` to `.env`. The API loads it via dotenvy; Compose also reads it.
Local-only database defaults are `registry` / `registry-local-password`, with
database `script_registry`. Use different credentials for deployed environments.
Database integration and migrations are not implemented yet, and `/health`
reports process health rather than database readiness.

## Local object storage

The Rust `object_store` crate uses `LocalFileSystem` directly. The server creates
`.local/object-storage/` at startup and makes the store available as router state.
No object-storage service, bucket, network endpoint, or storage credentials are
needed. Cloud-provider features are disabled for now.

`OBJECT_STORE_DIR` overrides the folder. Relative paths resolve from the process
working directory: running Cargo from the repository root, as above, keeps data
in the root `.local/` directory. `.env` and `.local/` are ignored by Git.
Avoid launching from `server/` with the default path, which would create a second
storage directory there.

Objects persist across API restarts. This scaffold does not yet expose artifact
upload/download endpoints or enforce content-addressed release publication.
Do not edit stored objects behind the application's back. A cloud storage backend
can be introduced through the same `ObjectStore` abstraction when needed.

## Docker persistence and lifecycle

```sh
docker compose ps
docker compose logs -f postgres
docker compose down
```

The `script-registry_postgres-alpine-data` volume survives `docker compose down`.
`docker compose down --volumes` deletes that database; deleting
`.local/object-storage/` deletes local artifacts. Both are destructive resets.

The Alpine configuration uses a new volume rather than reusing the previous
Debian-based PostgreSQL data directory. Any old `script-registry_postgres-data`
volume is left untouched. No database migration or data deletion is performed.

## Optional API container

`server/Dockerfile` remains available for containerized builds. It uses a
multi-stage build: the final Debian slim image contains the server and health
check, without the Rust compiler. Building it still needs temporary compiler
space, but it is not part of the default Compose startup.

```sh
docker build -t script-registry-server ./server
mkdir -p .local/object-storage
docker run --rm --init --name registry-api \
  --user "$(id -u):$(id -g)" \
  -p 127.0.0.1:3000:3000 \
  --mount "type=bind,src=$(pwd)/.local/object-storage,dst=/app/.local/object-storage" \
  script-registry-server
```

The host UID/GID lets the container write to the mounted folder. This command is
for POSIX shells. Without a mount, container-local objects are ephemeral.
The API binds to `0.0.0.0:3000` inside Docker, uses a health check, and handles
SIGTERM for graceful shutdown. Host-run defaults bind to `127.0.0.1:3000`.

For a build behind a TLS-inspecting proxy, supply a trusted combined CA bundle
as the optional BuildKit secret `proxy_ca`:

```sh
docker build --secret id=proxy_ca,src=/path/to/ca-bundle.pem \
  -t script-registry-server ./server
```

## Validation

```sh
cargo fmt --manifest-path server/Cargo.toml --all -- --check
cargo test --manifest-path server/Cargo.toml --locked
cargo build --manifest-path server/Cargo.toml --locked
docker compose config --quiet
```

CI runs formatting, tests, build, and Compose configuration validation on pushes
to `dev` and `master`, and pull requests targeting either branch.

## Branch and release workflow

```text
feature/* or fix/* -> dev -> staging -> master
```

Implement changes on separate feature/fix branches based on `dev`, then merge
reviewed pull requests into `dev`. Promote groups of integrated changes into
`staging` for batch validation, and then into `master` for release. Do not write
features directly on `dev` or skip staging for release promotion.

The remaining M0 setup/documentation work is authorized directly on `dev`.
New feature and fix work follows the separate-branch workflow above.

CI currently runs for pushes and pull requests on `dev` and `master` only;
`staging` is not yet included in its trigger configuration. Validate staging
batches before promoting them.

## Roadmap and release plan

The [confirmed ecosystem release plan](docs/RELEASE_PLAN.md) covers work across
the specification, crates, clients, CLI, registry, examples, and planned Molecule
tooling. Follow progress on the [public project board](https://github.com/orgs/ckb-idl/projects/1)
and review the [project proposal](https://talk.nervos.org/t/dis-ckb-idl-interface-description-language-and-discoverability-for-ckb-scripts/11031).

## Contributing and security

See [CONTRIBUTION.md](CONTRIBUTION.md) for development and pull request guidance,
and [SECURITY.md](SECURITY.md) for vulnerability reporting and trust boundaries.

## License

This project is licensed under the [MIT License](LICENSE). Third-party dependencies
retain their respective licenses.
