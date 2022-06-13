const colorSchemeKey = "color-scheme";

setColorScheme = function(colorScheme) {
  document.documentElement.setAttribute(colorSchemeKey, colorScheme);

  const body = document.body;
  if (body) {
    lightSwitch = document.getElementById("light-switch-icon");

    if (colorScheme == "dark") {
      body.classList.add("uk-light");
      body.classList.remove("uk-dark");
      lightSwitch.src = "/sun.svg";
    } else {
      body.classList.add("uk-dark");
      body.classList.remove("uk-light");
      lightSwitch.src = "/moon.svg";
    }
  }
}

setColorSchemeFromSystemPreference = function(e) {
  const colorScheme = e.matches ? "dark" : "light";
  console.log(colorScheme);
  setColorScheme(colorScheme);
  if (localStorage) {
    localStorage.setItem(colorSchemeKey, "system");
  }
}

const systemPreference = window.matchMedia("(prefers-color-scheme: dark)");
systemPreference.addEventListener("change", setColorSchemeFromSystemPreference);

const localStorage = window.localStorage;
const storedPreference = localStorage.getItem(colorSchemeKey);
if (storedPreference !== "system") {
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
