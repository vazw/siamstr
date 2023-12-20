FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y openssl libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY ./siamstr /app/siamstr
COPY ./site/ /app/site/
COPY ./database.db /app/database.db
ENV LEPTOS_OUTPUT_NAME="siamstr"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8008"
ENV LEPTOS_RELOAD_PORT="3001"
CMD ["./siamstr"]
