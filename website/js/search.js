import Fuse from 'fuse.js'

const docsVersionMeta = document.querySelector('meta[name="docs-version"]');
if (docsVersionMeta) {
  const docsVersion = docsVersionMeta.getAttribute('content');

  const indexUrl = `/search-index-${docsVersion}.json`;
  let searchIndex = null;

  // DOM elements
  const article = document.getElementById('docs-article');
  const searchResults = document.getElementById('docs-search-results');
  const searchResultsHeading = document.getElementById('docs-search-results-heading');
  const resultsList = document.getElementById('docs-results-list');
  let articleScrollY = 0;

  // Initialize the search index when the search box gets focus
  document.querySelectorAll('#docs-search-input').forEach((input) => {
    input.addEventListener('focus', async () => {
      await initSearch();
      await search(input.value);
    });

    input.addEventListener('input', async (event) => {
      search(event.target.value.trim());
    });
  })

  // Lazily load and build the index
  async function initSearch() {
    if (searchIndex) {
      return searchIndex
    };

    const data = await fetch(indexUrl).then(response => response.json());

    searchIndex = new Fuse(data, {
      keys: [
        { name: 'title', weight: 0.8 },
        { name: 'contents', weight: 0.3 },
        { name: 'keywords', weight: 0.7 },
        { name: 'module', weight: 0.2 },
      ],
      includeScore: true,
      useExtendedSearch: true,
    });

    return searchIndex;
  }

  async function search(query) {
    if (!query) {
      hideSearchResults();
      return;
    }

    const index = await initSearch();
    const results = index.search(query);

    resultsList.innerHTML = '';

    if (results.length > 0) {
      results.filter(result => result.score < 0.4).forEach(result => {
        const resultItem = result.item;
        const listItem = document.createElement('li');

        listItem.innerHTML = `
<a href="${resultItem.url}" class="uk-link-reset">
  ${resultItem.title}
</a>
<div class="uk-text-muted uk-text-small">
  ${resultItem.module}
</div>
`;

        resultsList.appendChild(listItem);
      });
    }

    if (article.hidden === false) {
      articleScrollY = window.scrollY;
      article.hidden = true;
      searchResults.hidden = false;
    }

    requestAnimationFrame(() => {
      searchResultsHeading.scrollIntoView();
    });
  }


  function hideSearchResults() {
    if (article.hidden === true) {
      article.hidden = false;
      searchResults.hidden = true;
      requestAnimationFrame(() => {
        window.scrollTo(0, articleScrollY);
      });
    }
  }

  // If the user clicks on a search result that's on the current page,
  // then scroll to the anchor instead of leaving the page
  resultsList.addEventListener('click', event => {
    const link = event.target.closest('a');
    if (!link) {
      return;
    }

    const url = new URL(link.href, location.href);
    if (url.pathname === location.pathname && url.search === location.search) {
      event.preventDefault();
      jumpToAnchor(url.hash);
    }
  });

  function jumpToAnchor(hash) {
    if (!hash) {
      return;
    }

    hideSearchResults();

    requestAnimationFrame(() => {
      const id = hash.slice(1);
      const target = document.getElementById(id);
      if (!target) return;

      if (!target.hasAttribute('tabindex')) {
        target.setAttribute('tabindex', '-1');
      }

      target.scrollIntoView({ behavior: 'smooth', block: 'start' });
      target.focus({ preventScroll: true });
    });
  }
}

