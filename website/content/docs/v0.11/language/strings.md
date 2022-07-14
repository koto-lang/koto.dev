+++
title = "Strings"
slug = "strings"
weight = 2
+++

# Strings

Strings can be declared using `'` or `"` quotes. 

````koto
'Hello, World!'
# -> Hello, World!

"Welcome to Koto 👋"
# -> Welcome to Koto 👋

'This is a string
that spans
several lines.'
# -> This is a string
# -> that spans
# -> several lines.
````

{% example_playground_link() %}
play.clear_output()

print 'Hello, World!'
# -> Hello, World!

print "Welcome to Koto 👋"
# -> Welcome to Koto 👋

print 'This is a string
that spans
several lines.'
# -> This is a string
# -> that spans
# -> several lines.

{% end %}
Strings can be joined together with the `+` operator.

````koto
'a' + 'Bc' + 'Def'
# -> aBcDef
````

{% example_playground_link() %}
play.clear_output()

print 'a' + 'Bc' + 'Def'
# -> aBcDef

{% end %}
Individual elements of a String can be accessed via indexing with `[]` braces.

````koto
'abcdef'[3]
# -> d
'👋🥳😆'[1]
# -> 🥳
````

{% example_playground_link() %}
play.clear_output()

print 'abcdef'[3]
# -> d
print '👋🥳😆'[1]
# -> 🥳

{% end %}
## String interpolation

Assigned values can be included in a String by prefixing them with `$`.

````koto
xyz = 123
'The value of xyz is $xyz'
# -> The value of xyz is 123
````

{% example_playground_link() %}
play.clear_output()

xyz = 123
print 'The value of xyz is $xyz'
# -> The value of xyz is 123

{% end %}
The `$` prefix can also be used to include the results of expressions surrounded with `{}` curly braces.

````koto
'2 plus 3 is ${2 + 3}.'
# -> 2 plus 3 is 5.
````

{% example_playground_link() %}
play.clear_output()

print '2 plus 3 is ${2 + 3}.'
# -> 2 plus 3 is 5.

{% end %}
## String Escape codes

Strings can contain the following escape codes to define special characters,
all of which start with a `\`. 

* `\n`: Newline
* `\r`: Carriage Return
* `\t`: Tab
* `\'`: Single quote
* `\"`: Double quote
* `\\`: Backslash
* `\$`: Dollar
* `\u{NNNNNN}`: Unicode character
  * Up to 6 hexadecimal digits can be included within the `{}` braces.
    The maximum value is `\u{10ffff}`.
* `\xNN`: ASCII character
  * Exactly 2 hexadecimal digits follow the `\x`.

````koto
'\$\'\"'
# -> $'"
'Hi \u{1F44B}'
# -> Hi 👋
````

{% example_playground_link() %}
play.clear_output()

print '\$\'\"'
# -> $'"
print 'Hi \u{1F44B}'
# -> Hi 👋

{% end %}
