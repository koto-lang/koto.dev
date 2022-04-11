serve:
  trunk serve --open

worker:
  wrangler dev gist-worker/index.js

setup-worker:
  pnpm install --dir gist-worker

test:
  cargo watch -x test
