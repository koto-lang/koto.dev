const colorSchemeKey = "color-scheme";

setColorScheme = function(colorScheme) {
  document.documentElement.setAttribute(colorSchemeKey, colorScheme);

  const body = document.body;
  if (body) {
    const lightSwitches = document.getElementsByClassName("light-switch-icon");
    // UIKit doesn't support dark mode for offscreen elements
    const mobileNav = document.getElementById("mobile-nav-contents");

    if (colorScheme == "dark") {
      body.classList.add("uk-light");
      body.classList.remove("uk-dark");

      for (let i=0; i < lightSwitches.length; i++) {
        lightSwitches[i].src = "/sun.svg";
      }

      mobileNav.classList.add("background-dark");
      mobileNav.classList.remove("background-light");
    } else {
      body.classList.add("uk-dark");
      body.classList.remove("uk-light");

      for (let i=0; i < lightSwitches.length; i++) {
        lightSwitches[i].src = "/moon.svg";
      }

      mobileNav.classList.add("background-light");
      mobileNav.classList.remove("background-dark");
    }
  }
}

setColorSchemeFromSystemPreference = function(e) {
  const colorScheme = e.matches ? "dark" : "light";
  setColorScheme(colorScheme);
  if (localStorage) {
    localStorage.setItem(colorSchemeKey, "system");
  }
}

const systemPreference = window.matchMedia("(prefers-color-scheme: dark)");
systemPreference.addEventListener("change", setColorSchemeFromSystemPreference);

const localStorage = window.localStorage;
const storedPreference = localStorage.getItem(colorSchemeKey);
if (storedPreference && storedPreference !== "system") {
  // Apply immediately to set the color-scheme attribute, avoids bg flashes on refresh
  setColorScheme(storedPreference);

  // Re-apply on window load to modify body elements
  window.onload = function() {
    setColorScheme(storedPreference);
  }
} else {
  setColorSchemeFromSystemPreference(systemPreference);

  window.onload = function() {
    setColorSchemeFromSystemPreference(systemPreference);
  }
}

toggleColorScheme = function() {
  if (localStorage) {
    const storedColorScheme = localStorage.getItem(colorSchemeKey);

    const colorScheme = (storedColorScheme === "system")
      ? systemPreference.matches ? "light" : "dark"
      : storedColorScheme === "dark" ? "light" : "dark";

    setColorScheme(colorScheme);
    localStorage.setItem(colorSchemeKey, colorScheme);
  }
}
