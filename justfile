serve:
  trunk serve --open

worker:
  wrangler dev gist-worker/index.js

setup-worker:
  pnpm install --dir gist-worker

publish-worker:
  wrangler publish gist-worker/index.js

test:
  cargo watch -x test
