FROM docker.io/node AS style
WORKDIR /node
COPY . .

RUN npm install -D tailwindcss@3
RUN mkdir -p style
RUN npx tailwindcss -i ./input.css -o ./style/output.css

FROM docker.io/debian:bookworm-slim AS builder

WORKDIR /work

RUN apt-get update && apt-get install -y clang gcc curl pkg-config openssl libssl3 libssl-dev ca-certificates perl make
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust.sh && sh rust.sh -y
RUN . "$HOME/.cargo/env" && rustup toolchain install nightly && rustup default nightly
RUN . "$HOME/.cargo/env" && rustup target add wasm32-unknown-unknown
RUN . "$HOME/.cargo/env" && cargo install cargo-leptos
COPY --from=style /node/ /work/
RUN mkdir -p target/site
RUN . "$HOME/.cargo/env" && RUSTFLAGS=--cfg=web_sys_unstable_apis cargo leptos build --release

##

FROM docker.io/debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y openssl libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /work/target/site/ ./site/
COPY --from=builder /work/target/release/siamstr .

# must match your final server executable name
ENTRYPOINT ["/app/siamstr"]
