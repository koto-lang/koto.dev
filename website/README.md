# The Koto Website

## Dependencies

Useful commands for working on the website are collected in a [`justfile`](https://github.com/casey/just), installation instructions for `just` are [here](https://github.com/casey/just#installation).

### Main Site

The main site is built with [Zola](https://getzola.org), installation instructions [here](https://www.getzola.org/documentation/getting-started/installation/).

### Playground

The playground is built with [Trunk](https://trunkrs.dev), installation instructions [here](https://trunkrs.dev/#install).

The Rust Wasm build target should be installed, e.g. `rustup target add wasm32-unknown-unknown`.

Koto is used in a post-processing build step so the CLI should be available in your path, e.g. via `cargo install koto_cli`.

### Gist Worker

The playground's worker for creating gists uses [Cloudflare Workers](https://workers.cloudflare.com), and the CLI tool for testing and deploying is [Wrangler](https://github.com/cloudflare/wrangler).

## Submodules

This repo uses submodules for its dependencies, run `submodule update --init --recursive` to make sure they're up to date.
