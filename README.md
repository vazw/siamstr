<picture>
    <source srcset="https://github.com/vazw/siamstr/blob/main/docs/pic.png?raw=true" media="(prefers-color-scheme: dark)">
    <img src="https://github.com/vazw/siamstr/blob/main/docs/pic.png?raw=true" alt="siamstr Logo">
</picture>

# Siamstr Nostr Address Provider

## Running project

### Rust Toolchain

You'll need to use the nightly Rust toolchain, and install the `wasm32-unknown-unknown` target as well as the Trunk and `cargo-leptos` tools:

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```

### TailwindCSS

Install TailwindCSS with `npm install -D tailwindcss`

### Run

To run the project locally,

1. run `npx tailwindcss -i ./input.css -o ./style/output.css --watch` in a terminal - this will build `style/output.css` and automatically rebuild when a change is detected in `input.css`
1. `cargo leptos watch` in the project directory.
1. In in your browser, navigate to [http://localhost:8008/?](http://localhost:8008/?)

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`
   - css files will be in the target/site too.
3. The `Dockerfile` and `docker-compose.yml` located in `docs/` directory
4. you may alreay set your domain-name dns pointing to your remote server IP Address

Copy these files to your remote server. The directory structure should be:

```text
siamstr
site/
Dockerfile
docker-compose.yml
users.json (optional)
database.db (auto-generated)
```

you should have `certbot` `docker` `nginx` installed on your remote server if not:

Ubuntu

```sh
sudo apt install certbot nginx docker docker-compose
```

Debian should follow the installtion process on docker [website](https://docs.docker.com/engine/install/debian/)

```sh
sudo apt install certbot nginx
```

setting up certbot and nginx config:

```sh
sudo certbot certonly --nginx -d example.com
# sudo certbot certonly --nginx -d www.siamstr.com (optional)
```

Copy `text` inside `nginx.config` and paste into `/etc/nginx/site-enabled/default` and restart nginx service

```sh
sudo systemctl restart nginx.service
```

Then start docker compose:

```sh
docker compose up
```

# แค่เนี้ย ง่าย ๆ เสร็จแล้ว
