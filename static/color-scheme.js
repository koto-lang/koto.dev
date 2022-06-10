updateColorScheme = function(e) {
  const colorScheme = e.matches ? "dark" : "light";

  document.documentElement.setAttribute("color-scheme", colorScheme);

  if (document.body) {
    body = document.body;
    if (colorScheme == "dark") {
      body.classList.add("uk-light");
      body.classList.remove("uk-dark");
    } else {
      body.classList.add("uk-dark");
      body.classList.remove("uk-light");
    }
  }
}

prefersDark = window.matchMedia("(prefers-color-scheme: dark)");
updateColorScheme(prefersDark);
prefersDark.addEventListener("change", updateColorScheme);
window.onload = function() {
  updateColorScheme(prefersDark);
}
