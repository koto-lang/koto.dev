# Contributing to the Koto website

Thank you for your interesting in contributing to the Koto website!

## Improving documentation

The source material for the website docs is maintained in the [Koto repository](https://github.com/koto-lang/koto/tree/main/crates/cli/docs), which is included here as a [submodule](./modules/koto). Versioned snapshots are maintained in [this repository](./website/content/docs/).

It can be helpful to work on documentation improvements while seeing how they look on the website, and the following workflow is suggested:

- Create a branch in the [Koto submodule](./modules/koto).
- Changes to the docs should be made in the [docs folder](./modules/koto/crates/cli/docs/).
- In the [website folder], run `just docs-watch` to automatically build the docs when files are changed. 
- In another shell, run `just serve` to open the site in a browser. The site will be automatically refreshed when the docs are updated. 
