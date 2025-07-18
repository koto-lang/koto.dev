const colorSchemeKey = "color-scheme";

function setColorScheme(colorScheme) {
  document.documentElement.setAttribute(colorSchemeKey, colorScheme);

  const body = document.body;
  if (body) {
    const lightSwitches = document.getElementsByClassName("light-switch-icon");
    // UIKit doesn't support dark mode for offscreen elements
    const mobileNav = document.getElementById("mobile-nav-contents");

    if (colorScheme == "dark") {
      body.classList.add("uk-light");
      body.classList.remove("uk-dark");

      for (let i = 0; i < lightSwitches.length; i++) {
        lightSwitches[i].src = "/sun.svg";
      }

      mobileNav.classList.add("background-dark");
      mobileNav.classList.remove("background-light");
    } else {
      body.classList.add("uk-dark");
      body.classList.remove("uk-light");

      for (let i = 0; i < lightSwitches.length; i++) {
        lightSwitches[i].src = "/moon.svg";
      }

      mobileNav.classList.add("background-light");
      mobileNav.classList.remove("background-dark");
    }
  }
}

const localStorage = window.localStorage;

function getStoredColorScheme() {
  if (localStorage) {
    return localStorage.getItem(colorSchemeKey);
  }
}

function setStoredColorScheme(colorScheme) {
  if (localStorage) {
    localStorage.setItem(colorSchemeKey, colorScheme);
  }
}

function setColorSchemeFromPreferenceForDark(event) {
  const colorScheme = event.matches ? "dark" : "light";
  setColorScheme(colorScheme);
  setStoredColorScheme("system");
}

const preferenceForDark = window.matchMedia("(prefers-color-scheme: dark)");
preferenceForDark.addEventListener("change", setColorSchemeFromPreferenceForDark);
const storedPreference = getStoredColorScheme();

if (storedPreference && storedPreference !== "system") {
  // Apply immediately to set the color-scheme attribute, avoids bg flashes on refresh
  setColorScheme(storedPreference);

  // Re-apply on window load to modify body elements
  window.onload = () => {
    setColorScheme(storedPreference);
  }
} else {
  setColorSchemeFromPreferenceForDark(preferenceForDark);

  window.onload = () => {
    setColorSchemeFromPreferenceForDark(preferenceForDark);
  }
}

// This file needs to be executed immediately in the head tag,
// so it isn't loaded as a module, and exports can't be used here.

window.toggleColorScheme = () => {
  const storedColorScheme = getStoredColorScheme();
  if (storedColorScheme) {
    const colorScheme = (storedColorScheme === "system")
      ? preferenceForDark.matches ? "light" : "dark"
      : storedColorScheme === "dark" ? "light" : "dark";

    setColorScheme(colorScheme);
    setStoredColorScheme(colorScheme)
  }
}
