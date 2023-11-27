<picture>
    <source srcset="https://raw.githubusercontent.com/vaz/siamstr/main/docs/pic.png" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/vaz/siamstr/main/docs/pic.png" alt="siamstr Logo">
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

Copy these files to your remote server. The directory structure should be:

```text
siamstr
site/
users.json (optional)
database.db (auto-generated)
```

Set the following environment variables (updating for your project as needed):

```sh
export LEPTOS_OUTPUT_NAME="siamstr"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:8008"
export LEPTOS_RELOAD_PORT="3001"
```
