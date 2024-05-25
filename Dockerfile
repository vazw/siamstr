FROM node AS style
WORKDIR /node
COPY . .

RUN npm install -D tailwindcss
RUN mkdir -p style
RUN npx tailwindcss -i ./input.css -o ./style/output.css

FROM debian:bookworm-slim AS builder

WORKDIR /work

RUN apt-get update && apt-get install -y clang gcc curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust.sh && sh rust.sh -y
RUN . "$HOME/.cargo/env" && rustup toolchain install nightly && rustup default nightly
RUN . "$HOME/.cargo/env" && rustup target add wasm32-unknown-unknown
RUN . "$HOME/.cargo/env" && cargo install cargo-leptos
COPY . .
COPY --from=style /node/style/ ./style/
RUN mkdir -p target/site
# after successful tests, build it
RUN . "$HOME/.cargo/env" && cargo update -p wasm-bindgen --precise 0.2.92 && cargo install -f wasm-bindgen-cli --version 0.2.92
RUN . "$HOME/.cargo/env" && RUSTFLAGS=--cfg=web_sys_unstable_apis cargo leptos build --release

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
