export function setup_editor() {
  var editorDiv = document.getElementById("editor");
  editorDiv.innerHTML = `\
# Fizz buzz in Koto

fizz_buzz = |n|
  match n % 3, n % 5
    0, 0 then "Fizz Buzz"
    0, _ then "Fizz"
    _, 0 then "Buzz"
    else n

for n in 1..20
  print fizz_buzz n
`;

  var editor = ace.edit("editor");
  editor.getSession().setMode("ace/mode/koto");
  editor.setTheme("ace/theme/solarized_dark");
  editor.setShowPrintMargin(false);
  editor.setBehavioursEnabled(false);
}
