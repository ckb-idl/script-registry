# syntax=docker/dockerfile:1
FROM rust:1-slim-bookworm AS builder
# Bind source only for compilation, avoiding redundant intermediate source layers.
# The optional CA secret supports builds behind a TLS-inspecting development proxy.
RUN --mount=type=bind,source=.,target=/source \
    --mount=type=secret,id=proxy_ca \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build \
    if [ -f /run/secrets/proxy_ca ]; then export CARGO_HTTP_CAINFO=/run/secrets/proxy_ca; fi; \
    cargo build --manifest-path /source/Cargo.toml --target-dir /build --release --locked \
    && cp /build/release/script-registry-server /tmp/server

FROM debian:bookworm-slim
WORKDIR /app
RUN mkdir -p /app/.local/object-storage && chown -R 10001:10001 /app/.local
COPY --from=builder /tmp/server /usr/local/bin/server
COPY --chmod=755 docker-healthcheck.sh /usr/local/bin/healthcheck
USER 10001:10001
ENV SERVER_ADDR=0.0.0.0:3000
ENV OBJECT_STORE_DIR=/app/.local/object-storage
EXPOSE 3000
HEALTHCHECK --interval=5s --timeout=3s --start-period=10s --retries=10 CMD ["healthcheck"]
ENTRYPOINT ["server"]
