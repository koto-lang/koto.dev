export function show_notification(text, icon) {
  const message =
    "<span uk-icon='icon: " +
    icon +
    "'></span><span class='uk-margin-small-left uk-text-small'>" +
    text +
    "</span>";

  UIkit.notification({
    message: message,
    status: "default",
    timeout: 1000,
  });
}
