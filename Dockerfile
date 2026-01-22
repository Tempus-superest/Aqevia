FROM rust:1.92.0-slim-trixie as builder
WORKDIR /app
COPY . .
WORKDIR /app/src
RUN cargo fmt --all -- --check && \
    cargo clippy --all-targets --all-features -- -D warnings && \
    cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends libsqlite3-0 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r aqevia && useradd --no-log-init -r -g aqevia aqevia
WORKDIR /app
COPY --from=builder /app/src/target/release/aqevia-engine /usr/local/bin/aqevia-engine
COPY VERSION /app/VERSION
RUN mkdir -p /data && chown aqevia:aqevia /data
VOLUME /data
ENV AQEVIA_SQLITE_PATH=/data/storage.sqlite \
    AQEVIA_OBSERVABILITY_ADDR=0.0.0.0:7878 \
    PERSIST_FLUSH_INTERVAL_MS=1000 \
    PERSIST_BATCH_CAPACITY=10
USER aqevia
ENTRYPOINT ["aqevia-engine"]
