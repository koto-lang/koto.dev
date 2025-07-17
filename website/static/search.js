const docsVersion = document
  .querySelector('meta[name="docs-version"]')
  .getAttribute('content');
const indexUrl = `/search-index-${docsVersion}.json`;
let searchIndex = null;
let displayStore = [];

// DOM elements
const article = document.getElementById('docs-article');
const searchInput = document.getElementById('docs-search-input');
const searchResults = document.getElementById('docs-search-results');
const searchResultsHeading = document.getElementById('docs-search-results-heading');
const resultsList = document.getElementById('docs-results-list');
let articleScrollY = 0;

// Initialize the search index when the search box gets focus
searchInput.addEventListener('focus', async () => {
  await initSearch();
  await search(searchInput.value);
});

searchInput.addEventListener('input', async (event) => {
  search(event.target.value.trim());
});

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

/* delegate one listener for the whole list */
resultsList.addEventListener('click', e => {
  const link = e.target.closest('a');
  if (!link) return;                           // not a click on a link

  const url = new URL(link.href, location.href);

  if (url.pathname === location.pathname && url.search === location.search) {
    e.preventDefault();                        // stop default “do nothing”
    jumpToAnchor(url.hash);                    // "#section-name"
  }
});

function jumpToAnchor(hash) {
  if (!hash) return;

  hideSearchResults();

  requestAnimationFrame(() => {
    const id = hash.slice(1);
    const target = document.getElementById(id);
    if (!target) return;

    if (!target.hasAttribute('tabindex')) {
      target.setAttribute('tabindex', '-1');
    }

    target.scrollIntoView({ behavior: 'smooth', block: 'start' });
    target.focus({ preventScroll: true });         // announce destination
  });

  /* 3. update the address bar so users can copy/send the link */
  history.replaceState(null, '', hash);
}
