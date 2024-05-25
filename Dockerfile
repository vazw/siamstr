FROM debian:bookworm-slim AS builder

WORKDIR /work

RUN apt-get update && apt-get install -y clang gcc cc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
COPY . .
RUN mkdir -p target/site
# after successful tests, build it
RUN rustup toolchain install nightly-2024-02-03
RUN rustup default nightly-2024-02-03
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
RUN cargo leptos build --release

##

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y openssl libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Must match your `output-name` from the `metadata.leptos` until the next release
ENV LEPTOS_OUTPUT_NAME="siamstr"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8008"
ENV LEPTOS_RELOAD_PORT="3001"

USER 10001

COPY --chown=10001:10001 --from=builder /work/target/site/ ./site/
COPY --chown=10001:10001 --from=builder /work/target/server/release/siamstr .

# must match your final server executable name
ENTRYPOINT ["/app/siamstr"]
