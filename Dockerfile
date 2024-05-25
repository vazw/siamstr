FROM leptos-builder-musl AS builder

WORKDIR /work

# IMPORTANT: have a `.dockerignore` file and exclude at least your `target`
# directory to not copy huge amounts of data into the docker context
#
# !!! EVEN MORE IMPORTANT !!!
# If you have any secrets in a `.env` file or something, add this to `.dockerignore` too!
COPY . .

# this small workaround fixes a chicken and egg problem with `rust_embed` in this template
# so we can check clippy before actually compiling
RUN mkdir -p target/site
# make sure we exit early if clippy is not happy
RUN cargo clippy -- -D warnings

# execute tests first
RUN cargo leptos test

# after successful tests, build it
RUN cargo leptos build --release

########################################
########################################
########################################

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
COPY ./database.db .

# must match your final server executable name
ENTRYPOINT ["/app/siamstr"]
