docs:
  cargo run -p convert-koto-docs

serve:
  @just docs && zola serve --open --fast
