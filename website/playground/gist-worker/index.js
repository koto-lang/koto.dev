// https://octokit.github.io/rest.js/v18#gists

import { Octokit } from '@octokit/rest'
import { Router } from 'itty-router'

const router = Router()
const octokit = new Octokit({ auth: GITHUB_TOKEN })

router.get('/play/create-gist/version', () => new Response('0.1.3'))

router.post('/play/create-gist', async (request) => {
  const script = await request.text()

  const { data: gist } = await octokit.rest.gists.create({
    files: { 'playground.koto': { content: script } },
    public: true,
  })

  const result = {
    id: gist.id,
    url: gist.html_url,
  }

  return new Response(JSON.stringify(result), {
    headers: { 'content-type': 'application/json;charset=UTF-8' },
  })
})

router.all('*', () => new Response('404', { status: 404 }))

addEventListener('fetch', (e) => {
  e.respondWith(router.handle(e.request))
})
