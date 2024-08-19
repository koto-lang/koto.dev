# The Koto Website

## Submodules

This repo uses submodules for its dependencies, run `git submodule update --init --recursive` to make sure they're up to date.

## Dependencies

Useful commands for working on the website are collected in a [`justfile`](https://github.com/casey/just), installation instructions for `just` are [here](https://github.com/casey/just#installation).

[git-lfs](https://git-lfs.com) is used for archiving old versions of the 
playground.

### Main Site

The main site is built with [Zola](https://getzola.org), installation instructions [here](https://www.getzola.org/documentation/getting-started/installation/).

### Playground

The playground is built with [Trunk](https://trunkrs.dev), installation instructions [here](https://trunkrs.dev/#install).

The Rust Wasm build target should be installed, e.g. `rustup target add wasm32-unknown-unknown`.

### Gist Worker

The playground's worker for creating gists uses [Cloudflare Workers](https://workers.cloudflare.com), and the CLI tool for testing and deploying is [Wrangler](https://github.com/cloudflare/wrangler).
