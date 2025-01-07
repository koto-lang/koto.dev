+++
title = "Language Guide"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 1
+++

# The Koto Language Guide

As you're reading this guide, you're encouraged to play around with the examples to get a feel
for the language.

When you see a <span uk-icon="play"></span> icon below an example,
clicking it will open the example in the [Koto Playground](/play),
where you can run the code and see what happens as you make changes.

You can also try out the examples using the [Koto CLI](../cli).


## Language Basics

Koto programs contain a series of expressions that are evaluated by Koto's runtime.

As an example, this simple script prints a friendly greeting.

````koto
name = 'World'
print 'Hello, {name}!'
````

{% example_playground_link(version = "0.15") %}
name = 'World'
print 'Hello, {name}!'

{% end %}
### Comments

Single-line comments start with a `#`.

````koto
# This is a comment, everything until the end of the line is ignored.
````

{% example_playground_link(version = "0.15") %}
# This is a comment, everything until the end of the line is ignored.

{% end %}
Multi-line comments start with `#-` and end with `-#`.

````koto
#-
This is a
multi-line
comment.
-#
````

{% example_playground_link(version = "0.15") %}
#-
This is a
multi-line
comment.
-#

{% end %}
### Numbers

Numbers and arithmetic are expressed in a familiar way.

````koto
1
# -> 1

1 + 1
# -> 2

-1 - 10
# -> -11

3 * 4
# -> 12

9 / 2
# -> 4.5

12 % 5
# -> 2
````

{% example_playground_link(version = "0.15") %}
print 1
# -> 1

print 1 + 1
# -> 2

print -1 - 10
# -> -11

print 3 * 4
# -> 12

print 9 / 2
# -> 4.5

print 12 % 5
# -> 2

{% end %}
#### Parentheses

Arithmetic operations follow the
[conventional order of precedence](https://en.wikipedia.org/wiki/Order_of_operations#Conventional_order).
Parentheses can be used to group expressions as needed.

````koto
# Without parentheses, multiplication is performed before addition
1 + 2 * 3 + 4
# -> 11
# With parentheses, the additions are performed first
(1 + 2) * (3 + 4)
# -> 21
````

{% example_playground_link(version = "0.15") %}
# Without parentheses, multiplication is performed before addition
print 1 + 2 * 3 + 4
# -> 11
# With parentheses, the additions are performed first
print (1 + 2) * (3 + 4)
# -> 21

{% end %}
### Booleans

Booleans are declared with the `true` and `false` keywords, and combined using
the `and` and `or` operators.

````koto
true and false
# -> false

true or false
# -> true
````

{% example_playground_link(version = "0.15") %}
print true and false
# -> false

print true or false
# -> true

{% end %}
Booleans can be negated with the `not` operator.

````koto
not true
# -> false

not false
# -> true
````

{% example_playground_link(version = "0.15") %}
print not true
# -> false

print not false
# -> true

{% end %}
Values can be compared for equality with the `==` and `!=` operators.

````koto
1 + 1 == 2
# -> true

99 != 100
# -> true
````

{% example_playground_link(version = "0.15") %}
print 1 + 1 == 2
# -> true

print 99 != 100
# -> true

{% end %}
### Null

The `null` keyword is used to declare a value of type `Null`,
which indicates the absence of a value.

````koto
null
# -> null
````

{% example_playground_link(version = "0.15") %}
print null
# -> null

{% end %}
#### Truthiness

In boolean contexts (such as logical operations), `null` is treated as being
equivalent to `false`. Every other value in Koto evaluates as `true`.

````koto
not null
# -> true

null or 42
# -> 42
````

{% example_playground_link(version = "0.15") %}
print not null
# -> true

print null or 42
# -> 42

{% end %}
### Assigning Variables

Values are assigned to named identifiers with `=`, and can be freely reassigned.
Named values like this are known as *variables*.

````koto
# Assign the value `42` to `x`
x = 42
x
# -> 42

# Replace the existing value of `x`
x = true
x
# -> true
````

{% example_playground_link(version = "0.15") %}
# Assign the value `42` to `x`
x = 42
print x
# -> 42

# Replace the existing value of `x`
x = true
print x
# -> true

{% end %}
[Compound assignment](https://en.wikipedia.org/wiki/Augmented_assignment) operators are also available.
For example, `x *= y` is a simpler way of writing `x = x * y`.

````koto
a = 100
a += 11
# -> 111
a
# -> 111

a *= 10
# -> 1110
a
# -> 1110
````

{% example_playground_link(version = "0.15") %}
a = 100
print a += 11
# -> 111
print a
# -> 111

print a *= 10
# -> 1110
print a
# -> 1110

{% end %}
### Debug

The `debug` keyword allows you to quickly display a value while working on a
program.

It prints the result of an expression, prefixed with its line number and the
original expression as a string.

````koto
x = 10 + 20
debug x / 10
# -> [2] x / 10: 3.0
````

{% example_playground_link(version = "0.15") %}
x = 10 + 20
debug x / 10
# -> [2] x / 10: 3.0

{% end %}
When using `debug`, the displayed value is also the result of the expression,
which can be useful if you want to quickly get feedback during development.

````koto
x = debug 2 + 2
# -> [1] 2 + 2: 4
x
# -> 4
````

{% example_playground_link(version = "0.15") %}
x = debug 2 + 2
# -> [1] 2 + 2: 4
print x
# -> 4

{% end %}
### Semicolons

Expressions are typically placed on separate lines,
but if necessary they can be separated with semicolons.

````koto
a = 1; b = 2; c = a + b
c
# -> 3
````

{% example_playground_link(version = "0.15") %}
a = 1; b = 2; c = a + b
print c
# -> 3

{% end %}
## Lists

Lists in Koto are created with `[]` square brackets and can contain a mix of
different value types.

Access list elements by *index* using square brackets, starting from `0`.

````koto
x = [99, null, true]
x[0]
# -> 99
x[1]
# -> null

x[2] = false
x[2]
# -> false
````

{% example_playground_link(version = "0.15") %}
x = [99, null, true]
print x[0]
# -> 99
print x[1]
# -> null

x[2] = false
print x[2]
# -> false

{% end %}
Once a list has been created, its underlying data is shared between other
instances of the same list.
Changes to one instance of the list are reflected in the other.

````koto
# Assign a list to x
x = [10, 20, 30]

# Assign another instance of the list to y
y = x

# Modify the list through y
y[1] = 99

# The change to y is also reflected in x
x
# -> [10, 99, 30]
````

{% example_playground_link(version = "0.15") %}
# Assign a list to x
x = [10, 20, 30]

# Assign another instance of the list to y
y = x

# Modify the list through y
y[1] = 99

# The change to y is also reflected in x
print x
# -> [10, 99, 30]

{% end %}
### Joining Lists

The `+` operator allows lists to be joined together, creating a new list that
contains their concatenated elements.

````koto
a = [98, 99, 100]
b = a + [1, 2, 3]
b
# -> [98, 99, 100, 1, 2, 3]
````

{% example_playground_link(version = "0.15") %}
a = [98, 99, 100]
b = a + [1, 2, 3]
print b
# -> [98, 99, 100, 1, 2, 3]

{% end %}
## Tuples

Tuples in Koto are similar to lists,
but are designed for sequences of values that have a fixed structure.

Unlike lists, tuples can't be resized after creation,
and values that are contained in the tuple can't be replaced.

Tuples are declared with a series of expressions separated by commas.

````koto
x = 100, true, -1
x
# -> (100, true, -1)
````

{% example_playground_link(version = "0.15") %}
x = 100, true, -1
print x
# -> (100, true, -1)

{% end %}
Parentheses can be used for grouping to avoid ambiguity.

````koto
(1, 2, 3), (4, 5, 6)
# -> ((1, 2, 3), (4, 5, 6))
````

{% example_playground_link(version = "0.15") %}
print (1, 2, 3), (4, 5, 6)
# -> ((1, 2, 3), (4, 5, 6))

{% end %}
You can access tuple elements by index using square brackets, starting from `0`.

````koto
x = false, 10
# -> (false, 10)
x[0]
# -> false
x[1]
# -> 10

y = true, 20
# -> (true, 20)
x, y
# -> ((false, 10), (true, 20))
````

{% example_playground_link(version = "0.15") %}
print x = false, 10
# -> (false, 10)
print x[0]
# -> false
print x[1]
# -> 10

print y = true, 20
# -> (true, 20)
print x, y
# -> ((false, 10), (true, 20))

{% end %}
### Joining Tuples

The `+` operator allows tuples to be joined together,
creating a new tuple containing their concatenated elements.

````koto
a = 1, 2, 3
b = a + (4, 5, 6)
b
# -> (1, 2, 3, 4, 5, 6)
````

{% example_playground_link(version = "0.15") %}
a = 1, 2, 3
b = a + (4, 5, 6)
print b
# -> (1, 2, 3, 4, 5, 6)

{% end %}
### Creating Empty Tuples

An empty pair of parentheses in Koto resolves to `null`.
If an empty tuple is needed then use a single `,` inside parentheses.

````koto
# An empty pair of parentheses resolves to null
()
# -> null

# A comma inside parentheses creates a tuple
(,)
# -> ()
````

{% example_playground_link(version = "0.15") %}
# An empty pair of parentheses resolves to null
print ()
# -> null

# A comma inside parentheses creates a tuple
print (,)
# -> ()

{% end %}
### Tuple Mutability

While tuples have a fixed structure and its contained elements can't be
replaced, [*mutable*](https://en.wikipedia.org/wiki/Immutable_object) value types (like [lists](#lists)) can be
modified while they're contained in tuples.

````koto
# A Tuple containing two lists
x = ([1, 2, 3], [4, 5, 6])

# Modify the second list in the tuple
x[1][0] = 99
x
# -> ([1, 2, 3], [99, 5, 6])
````

{% example_playground_link(version = "0.15") %}
# A Tuple containing two lists
x = ([1, 2, 3], [4, 5, 6])

# Modify the second list in the tuple
x[1][0] = 99
print x
# -> ([1, 2, 3], [99, 5, 6])

{% end %}
## Strings

Strings in Koto contain a sequence of [UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoded characters,
and can be declared using `'` or `"` quotes.

````koto
'Hello, World!'
# -> Hello, World!

"Welcome to Koto 👋"
# -> Welcome to Koto 👋
````

{% example_playground_link(version = "0.15") %}
print 'Hello, World!'
# -> Hello, World!

print "Welcome to Koto 👋"
# -> Welcome to Koto 👋

{% end %}
Strings can start on one line and finish on another.

````koto
'This is a string
that spans
several lines.'
# -> This is a string
# -> that spans
# -> several lines.
````

{% example_playground_link(version = "0.15") %}
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

{% example_playground_link(version = "0.15") %}
print 'a' + 'Bc' + 'Def'
# -> aBcDef

{% end %}
### String Interpolation

Variables can be easily included in a string by surrounding them with `{}` curly
braces.

````koto
xyz = 123
'The value of xyz is {xyz}'
# -> The value of xyz is 123
````

{% example_playground_link(version = "0.15") %}
xyz = 123
print 'The value of xyz is {xyz}'
# -> The value of xyz is 123

{% end %}
Including variables in a string this way is known as *string interpolation*.

Simple expressions can also be interpolated using the same syntax.

````koto
'2 plus 3 is {2 + 3}.'
# -> 2 plus 3 is 5.
````

{% example_playground_link(version = "0.15") %}
print '2 plus 3 is {2 + 3}.'
# -> 2 plus 3 is 5.

{% end %}
### String Escape Codes

Strings can contain the following escape codes to define special characters,
all of which start with a `\`.

* `\n`: Newline
* `\r`: Carriage Return
* `\t`: Tab
* `\'`: Single quote
* `\"`: Double quote
* `\\`: Backslash
* `\{`: Interpolation start
* `\u{NNNNNN}`: Unicode character
  * Up to 6 hexadecimal digits can be included within the `{}` braces.
    The maximum value is `\u{10ffff}`.
* `\xNN`: ASCII character
  * Exactly 2 hexadecimal digits follow the `\x`.

````koto
'\{\'\"}'
# -> {'"}
'Hi \u{1F44B}'
# -> Hi 👋
````

{% example_playground_link(version = "0.15") %}
print '\{\'\"}'
# -> {'"}
print 'Hi \u{1F44B}'
# -> Hi 👋

{% end %}
### Continuing a Long Line

The end of a line can be escaped with a `\`, which will skip the
newline and any leading whitespace on the next line.

````koto
foo = "This string \
       doesn't contain \
       newlines."
foo
# -> This string doesn't contain newlines.
````

{% example_playground_link(version = "0.15") %}
foo = "This string \
       doesn't contain \
       newlines."
print foo
# -> This string doesn't contain newlines.

{% end %}
### Single or Double Quotes

Both single `'` and double `"` quotes are valid for defining strings in Koto
and can be used interchangeably.

A practical reason to choose one over the other is that the alternate
quote type can be used in a string without needing to use escape characters.

````koto
print 'This string has to escape its \'single quotes\'.'
# -> This string has to escape its 'single quotes'.

print "This string contains unescaped 'single quotes'."
# -> This string contains unescaped 'single quotes'.
````

{% example_playground_link(version = "0.15") %}
print 'This string has to escape its \'single quotes\'.'
# -> This string has to escape its 'single quotes'.

print "This string contains unescaped 'single quotes'."
# -> This string contains unescaped 'single quotes'.

{% end %}
### String Indexing

Individual *bytes* of a string can be accessed via indexing with `[]` braces.

````koto
'abcdef'[3]
# -> d
'xyz'[1..]
# -> yz
````

{% example_playground_link(version = "0.15") %}
print 'abcdef'[3]
# -> d
print 'xyz'[1..]
# -> yz

{% end %}
Care must be taken when using indexing with strings that could contain
non-[ASCII](https://en.wikipedia.org/wiki/ASCII) data.
If the indexed bytes would produce invalid UTF-8 data then an
error will be thrown. To access Unicode characters see [`string.chars`](../core/string#chars).

### Raw Strings

When a string contains a lot of special characters, it can be preferable to use
a *raw string*.

Raw strings ignore escape characters and interpolated expressions,
providing the raw contents of the string between its *delimiters*.

Raw strings use single or double quotes as the delimiter, prefixed with an `r`.

````koto
print r'This string contains special characters: {foo}\n\t.'
# -> This string contains special characters: {foo}\n\t.
````

{% example_playground_link(version = "0.15") %}
print r'This string contains special characters: {foo}\n\t.'
# -> This string contains special characters: {foo}\n\t.

{% end %}
For more complex string contents, the delimiter can be extended using up to 255
`#` characters after the `r` prefix,

````koto
print r#'This string contains "both" 'quote' types.'#
# -> This string contains "both" 'quote' types.

print r##'This string also includes a '#' symbol.'##
# -> This string also includes a '#' symbol.
````

{% example_playground_link(version = "0.15") %}
print r#'This string contains "both" 'quote' types.'#
# -> This string contains "both" 'quote' types.

print r##'This string also includes a '#' symbol.'##
# -> This string also includes a '#' symbol.

{% end %}
## Functions

Functions in Koto are created using a pair of vertical bars (`||`),
with the function's *arguments* listed between the bars.
The *body* of the function follows the vertical bars.

````koto
hi = || 'Hello!'
add = |x, y| x + y
````

{% example_playground_link(version = "0.15") %}
hi = || 'Hello!'
add = |x, y| x + y

{% end %}
Functions are *called* with arguments contained in `()` parentheses.
The body of the function is evaluated and the result is returned to the caller.

````koto
hi = || 'Hello!'
hi()
# -> Hello!

add = |x, y| x + y
add(50, 5)
# -> 55
````

{% example_playground_link(version = "0.15") %}
hi = || 'Hello!'
print hi()
# -> Hello!

add = |x, y| x + y
print add(50, 5)
# -> 55

{% end %}
A function's body can be an indented block, where the last
expression in the body is evaluated as the function's result.

````koto
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
f(2, 3, 4)
# -> 234
````

{% example_playground_link(version = "0.15") %}
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
print f(2, 3, 4)
# -> 234

{% end %}
### Optional Call Parentheses

The parentheses for arguments when calling a function are optional and can be
omitted in simple expressions.

````koto
square = |x| x * x
square 8
# -> 64

add = |x, y| x + y
add 2, 3
# -> 5

# Equivalent to square(add(2, 3))
square add 2, 3
# -> 25
````

{% example_playground_link(version = "0.15") %}
square = |x| x * x
print square 8
# -> 64

add = |x, y| x + y
print add 2, 3
# -> 5

# Equivalent to square(add(2, 3))
print square add 2, 3
# -> 25

{% end %}
Something to watch out for is that whitespace is important in Koto, and because
of optional parentheses, `f(1, 2)` is *not the same* as `f (1, 2)`. The former
is parsed as a call to `f` with two arguments, whereas the latter is a call to
`f` with a tuple as the single argument.

### Return

When the function should be exited early, the `return` keyword can be used.

````koto
f = |n|
  return 42
  # This expression won't be reached
  n * n
f -1
# -> 42
````

{% example_playground_link(version = "0.15") %}
f = |n|
  return 42
  # This expression won't be reached
  n * n
print f -1
# -> 42

{% end %}
If a value isn't provided to `return`, then the returned value is `null`.

````koto
f = |n|
  return
  n * n
f 123
# -> null
````

{% example_playground_link(version = "0.15") %}
f = |n|
  return
  n * n
print f 123
# -> null

{% end %}
### Function Piping

The arrow operator (`->`) can be used to pass the result of one function to
another, working from left to right. This is known as *function piping*,
and can aid readability when working with a long chain of function calls.

````koto
add = |x, y| x + y
multiply = |x, y| x * y
square = |x| x * x

# Chained function calls can be a bit hard to follow for the reader.
x = multiply 2, square add 1, 3
# -> 32

# Parentheses don't help all that much...
x = multiply(2, square(add(1, 3)))
# -> 32

# Piping allows for a left-to-right flow of results.
x = add(1, 3) -> square -> multiply 2
# -> 32

# Call chains can also be broken across lines.
x = add 1, 3
  -> square
  -> multiply 2
# -> 32
````

{% example_playground_link(version = "0.15") %}
add = |x, y| x + y
multiply = |x, y| x * y
square = |x| x * x

# Chained function calls can be a bit hard to follow for the reader.
print x = multiply 2, square add 1, 3
# -> 32

# Parentheses don't help all that much...
print x = multiply(2, square(add(1, 3)))
# -> 32

# Piping allows for a left-to-right flow of results.
print x = add(1, 3) -> square -> multiply 2
# -> 32

# Call chains can also be broken across lines.
print x = add 1, 3
  -> square
  -> multiply 2
# -> 32

{% end %}
## Maps

*Maps* in Koto are containers that contain a series of
*entries* with *keys* that correspond to [associated](https://en.wikipedia.org/wiki/Associative_array) *values*.

The `.` dot operator returns the value associated with a particular key.

Maps can be created using *inline syntax* with `{}` braces:

````koto
m = {apples: 42, oranges: 99, lemons: 63}

# Get the value associated with the `oranges` key
m.oranges
# -> 99
````

{% example_playground_link(version = "0.15") %}
m = {apples: 42, oranges: 99, lemons: 63}

# Get the value associated with the `oranges` key
print m.oranges
# -> 99

{% end %}
...Or using *block syntax* with indented entries:

````koto
m =
  apples: 42
  oranges: 99
  lemons: 63
m.apples
# -> 42
````

{% example_playground_link(version = "0.15") %}
m =
  apples: 42
  oranges: 99
  lemons: 63
print m.apples
# -> 42

{% end %}
Once a map has been created, its underlying data is shared between other
instances of the same map. Changes to one instance are reflected in the other.

````koto
# Create a map and assign it to `a`.
a = {foo: 99}
a.foo
# -> 99

# Assign a new instance of the map to `z`.
z = a

# Modifying the data via `z` is reflected in `a`.
z.foo = 'Hi!'
a.foo
# -> Hi!
````

{% example_playground_link(version = "0.15") %}
# Create a map and assign it to `a`.
a = {foo: 99}
print a.foo
# -> 99

# Assign a new instance of the map to `z`.
z = a

# Modifying the data via `z` is reflected in `a`.
z.foo = 'Hi!'
print a.foo
# -> Hi!

{% end %}
### Entry Order

A map's entries are maintained in a consistent order,
representing the sequence in which its entries were added.

You can access map entries by index using square brackets, starting from `0`.

The entry is returned as a tuple containing the key and its associated value.

````koto
m = {apples: 42, oranges: 99, lemons: 63}
m[1]
# -> ('oranges', 99)
````

{% example_playground_link(version = "0.15") %}
m = {apples: 42, oranges: 99, lemons: 63}
print m[1]
# -> ('oranges', 99)

{% end %}
Entries can also be replaced by assigning a key/value tuple to the entry's index.

````koto
m = {apples: 42, oranges: 99, lemons: 63}
m[1] = ('pears', 123)
m
# -> {apples: 42, pears: 123, lemons: 63}
````

{% example_playground_link(version = "0.15") %}
m = {apples: 42, oranges: 99, lemons: 63}
m[1] = ('pears', 123)
print m
# -> {apples: 42, pears: 123, lemons: 63}

{% end %}
### Shorthand Values

Koto supports a shorthand notation when creating maps with inline syntax.
If a value isn't provided for a key, then Koto will look for a value in scope
that matches the key's name, and if one is found then it will be used as that
entry's value.

````koto
hi, bye = 'hi!', 'bye!'
m = {hi, x: 42, bye}
# -> {hi: 'hi!', x: 42, bye: 'bye!'}
````

{% example_playground_link(version = "0.15") %}
hi, bye = 'hi!', 'bye!'
print m = {hi, x: 42, bye}
# -> {hi: 'hi!', x: 42, bye: 'bye!'}

{% end %}
### Maps and Self

Maps can store any type of value, including functions,
which provides a convenient way to group functions together.

````koto
m =
  hello: |name| 'Hello, {name}!'
  bye: |name| 'Bye, {name}!'

m.hello 'World'
# -> Hello, World!
m.bye 'Friend'
# -> Bye, Friend!
````

{% example_playground_link(version = "0.15") %}
m =
  hello: |name| 'Hello, {name}!'
  bye: |name| 'Bye, {name}!'

print m.hello 'World'
# -> Hello, World!
print m.bye 'Friend'
# -> Bye, Friend!

{% end %}
`self` is a special identifier that refers to the instance of the container in
which the function is contained.

In maps, `self` allows functions to access and modify data from the map,
enabling object-like behaviour.

````koto
m =
  name: 'World'
  say_hello: || 'Hello, {self.name}!'

m.say_hello()
# -> Hello, World!

m.name = 'Friend'
m.say_hello()
# -> Hello, Friend!
````

{% example_playground_link(version = "0.15") %}
m =
  name: 'World'
  say_hello: || 'Hello, {self.name}!'

print m.say_hello()
# -> Hello, World!

m.name = 'Friend'
print m.say_hello()
# -> Hello, Friend!

{% end %}
### Joining Maps

The `+` operator allows maps to be joined together, creating a new map that
combines their entries.

````koto
a = {red: 100, blue: 150}
b = {green: 200, blue: 99}
c = a + b
c
# -> {red: 100, blue: 99, green: 200}
````

{% example_playground_link(version = "0.15") %}
a = {red: 100, blue: 150}
b = {green: 200, blue: 99}
c = a + b
print c
# -> {red: 100, blue: 99, green: 200}

{% end %}
### Quoted Map Keys

Map keys are usually defined and accessed without quotes, but they are stored in
the map as strings. Quotes can be used if a key needs to be defined that would be
otherwise be disallowed by Koto syntax rules
(e.g. a keyword, or using characters that aren't allowed in an identifier).
Quoted keys also allow dynamic keys to be generated by using string
interpolation.

````koto
x = 99
m =
  'true': 42
  'key{x}': x
m.'true'
# -> 42
m.key99
# -> 99
````

{% example_playground_link(version = "0.15") %}
x = 99
m =
  'true': 42
  'key{x}': x
print m.'true'
# -> 42
print m.key99
# -> 99

{% end %}
### Map Key Types

Map keys are typically strings, but any [*immutable*](https://en.wikipedia.org/wiki/Immutable_object) value can be
used as a map key by using the [`map.insert`](../core/map#insert) and [`map.get`](../core/map#get)
functions.

The immutable value types in Koto are [strings](#strings), [numbers](#numbers),
[booleans](#booleans), [ranges](#ranges), and [`null`](#null).

A [tuple](#tuples) is also considered to be immutable when its contained
elements are also immutable.

## Core Library

The [*Core Library*](../core) provides a collection of fundamental functions
and values for working with the Koto language, organized within *modules*.

````koto
# Get the size of a string
string.to_lowercase 'HELLO'
# -> hello

# Return the first element of the list
list.first [99, -1, 3]
# -> 99
````

{% example_playground_link(version = "0.15") %}
# Get the size of a string
print string.to_lowercase 'HELLO'
# -> hello

# Return the first element of the list
print list.first [99, -1, 3]
# -> 99

{% end %}
Values in Koto automatically have access to their corresponding core modules
via `.` access.

````koto
'xyz'.to_uppercase()
# -> XYZ

['abc', 123].first()
# -> abc

(7 / 2).round()
# -> 4

{apples: 42, pears: 99}.contains_key 'apples'
# -> true
````

{% example_playground_link(version = "0.15") %}
print 'xyz'.to_uppercase()
# -> XYZ

print ['abc', 123].first()
# -> abc

print (7 / 2).round()
# -> 4

print {apples: 42, pears: 99}.contains_key 'apples'
# -> true

{% end %}
The [documentation](../core) for the Core library (along with this guide) are
available in the `help` command of the [Koto CLI](..).

### Prelude

Koto's *prelude* is a collection of core library items that are automatically
made available in a Koto script without the need for first calling `import`.

The modules that make up the core library are all included by default in the
prelude. The following functions are also added to the prelude by default:

* [`io.print`](../core/io#print)
* [`koto.copy`](../core/koto#copy)
* [`koto.size`](../core/koto#size)
* [`koto.type`](../core/koto#type)
* [`test.assert`](../core/test#assert)
* [`test.assert_eq`](../core/test#assert-eq)
* [`test.assert_ne`](../core/test#assert-ne)
* [`test.assert_near`](../core/test#assert-near)

````koto
print 'io.print is available without needing to be imported'
# -> io.print is available without needing to be imported
````

{% example_playground_link(version = "0.15") %}
print 'io.print is available without needing to be imported'
# -> io.print is available without needing to be imported

{% end %}
## Conditional Expressions

Koto includes several ways of producing values that depend on *conditions*.

### `if`

`if` expressions come in two flavors; single-line:

````koto
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
# -> odd
````

{% example_playground_link(version = "0.15") %}
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
# -> odd

{% end %}
...And multi-line using indented blocks:

````koto
x = 24
if x < 0
  print 'negative'
else if x > 24
  print 'no way!'
else
  print 'ok'
# -> ok
````

{% example_playground_link(version = "0.15") %}
x = 24
if x < 0
  print 'negative'
else if x > 24
  print 'no way!'
else
  print 'ok'
# -> ok

{% end %}
The result of an `if` expression is the final expression in the branch that gets
executed.

````koto
x = if 1 + 1 == 2 then 3 else -1
x
# -> 3

# Assign the result of the if expression to foo
foo = if x > 0
  y = x * 10
  y + 3
else
  y = x * 100
  y * y

foo
# -> 33
````

{% example_playground_link(version = "0.15") %}
x = if 1 + 1 == 2 then 3 else -1
print x
# -> 3

# Assign the result of the if expression to foo
foo = if x > 0
  y = x * 10
  y + 3
else
  y = x * 100
  y * y

print foo
# -> 33

{% end %}
### `switch`

`switch` expressions can be used as a cleaner alternative to
`if`/`else if`/`else` cascades.

````koto
fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

fib 7
# -> 13
````

{% example_playground_link(version = "0.15") %}
fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

print fib 7
# -> 13

{% end %}
### `match`

`match` expressions can be used to match a value against a series of patterns,
with the matched pattern causing a specific branch of code to be executed.

Patterns can be literals or identifiers. An identifier will accept any value,
so they're often used with `if` conditions to refine the match.

````koto
match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: {x}'
  x if x < 50 then 'less than 50: {x}'
  x then 'other: {x}'
# -> less than 50: 42
````

{% example_playground_link(version = "0.15") %}
print match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: {x}'
  x if x < 50 then 'less than 50: {x}'
  x then 'other: {x}'
# -> less than 50: 42

{% end %}
The `_` wildcard match can be used to match against any value
(when the matched value itself can be ignored),
and `else` can be used for fallback branches.

````koto
fizz_buzz = |n|
  match n % 3, n % 5
    0, 0 then "Fizz Buzz"
    0, _ then "Fizz"
    _, 0 then "Buzz"
    else n

(10, 11, 12, 13, 14, 15)
  .each |n| fizz_buzz n
  .to_tuple()
# -> ('Buzz', 11, 'Fizz', 13, 14, 'Fizz Buzz')
````

{% example_playground_link(version = "0.15") %}
fizz_buzz = |n|
  match n % 3, n % 5
    0, 0 then "Fizz Buzz"
    0, _ then "Fizz"
    _, 0 then "Buzz"
    else n

print (10, 11, 12, 13, 14, 15)
  .each |n| fizz_buzz n
  .to_tuple()
# -> ('Buzz', 11, 'Fizz', 13, 14, 'Fizz Buzz')

{% end %}
List and tuple entries can be matched against by using parentheses,
with `...` available for capturing the rest of the sequence.

````koto
match ['a', 'b', 'c'].extend [1, 2, 3]
  ('a', 'b') then "A list containing 'a' and 'b'"
  (1, ...) then "Starts with '1'"
  (..., 'y', last) then "Ends with 'y' followed by '{last}'"
  ('a', x, others...) then
    "Starts with 'a', followed by '{x}', then {size others} others"
  unmatched then "other: {unmatched}"
# -> Starts with 'a', followed by 'b', then 4 others
````

{% example_playground_link(version = "0.15") %}
print match ['a', 'b', 'c'].extend [1, 2, 3]
  ('a', 'b') then "A list containing 'a' and 'b'"
  (1, ...) then "Starts with '1'"
  (..., 'y', last) then "Ends with 'y' followed by '{last}'"
  ('a', x, others...) then
    "Starts with 'a', followed by '{x}', then {size others} others"
  unmatched then "other: {unmatched}"
# -> Starts with 'a', followed by 'b', then 4 others

{% end %}
### Optional Chaining

The `?` operator can be used to short-circuit expression chains where `null`
might be encountered as an intermediate value. The `?` checks the current value
in the expression chain and if `null` is found then the chain is short-circuited
with `null` given as the expression's result.

This makes it easier to check for `null` when you want to avoid runtime errors.

````koto
info = {town: 'Hamburg', country: 'Germany'}

# `info` contains a value for 'town', which is then passed to to_uppercase():
info.get('town')?.to_uppercase()
# -> HAMBURG

# `info` doesn't contain a value for 'state',
# so the `?` operator short-circuits the expression, resulting in `null`:
info.get('state')?.to_uppercase()
# -> null

# Without the `?` operator an intermediate step is necessary:
country = info.get('country')
if country then country.to_uppercase()
# -> GERMANY
````

{% example_playground_link(version = "0.15") %}
info = {town: 'Hamburg', country: 'Germany'}

# `info` contains a value for 'town', which is then passed to to_uppercase():
print info.get('town')?.to_uppercase()
# -> HAMBURG

# `info` doesn't contain a value for 'state',
# so the `?` operator short-circuits the expression, resulting in `null`:
print info.get('state')?.to_uppercase()
# -> null

# Without the `?` operator an intermediate step is necessary:
country = info.get('country')
print if country then country.to_uppercase()
# -> GERMANY

{% end %}
Multiple `?` checks can be performed in an expression chain:

````koto
get_data = || {nested: {maybe_string: null}}
get_data()?
  .get('nested')?
  .get('maybe_string')?
  .to_uppercase()
# -> null
````

{% example_playground_link(version = "0.15") %}
get_data = || {nested: {maybe_string: null}}
print get_data()?
  .get('nested')?
  .get('maybe_string')?
  .to_uppercase()
# -> null

{% end %}
## Loops

Koto includes several ways of evaluating expressions repeatedly in a loop.

### `for`

`for` loops are repeated for each element in a sequence,
such as a list or tuple.

````koto
for n in [10, 20, 30]
  print n
# -> 10
# -> 20
# -> 30
````

{% example_playground_link(version = "0.15") %}
for n in [10, 20, 30]
  print n
# -> 10
# -> 20
# -> 30

{% end %}
### `while`

`while` loops continue to repeat *while* a condition is true.

````koto
x = 0
while x < 5
  x += 1
x
# -> 5
````

{% example_playground_link(version = "0.15") %}
x = 0
while x < 5
  x += 1
print x
# -> 5

{% end %}
### `until`

`until` loops continue to repeat *until* a condition is true.

````koto
z = [1, 2, 3]
until z.is_empty()
  # Remove the last element of the list
  print z.pop()
# -> 3
# -> 2
# -> 1
````

{% example_playground_link(version = "0.15") %}
z = [1, 2, 3]
until z.is_empty()
  # Remove the last element of the list
  print z.pop()
# -> 3
# -> 2
# -> 1

{% end %}
### `continue`

`continue` skips the remaining part of a loop's body and proceeds with the next repetition of the loop.

````koto
for n in (-2, -1, 1, 2)
  # Skip over any values less than 0
  if n < 0
    continue
  print n
# -> 1
# -> 2
````

{% example_playground_link(version = "0.15") %}
for n in (-2, -1, 1, 2)
  # Skip over any values less than 0
  if n < 0
    continue
  print n
# -> 1
# -> 2

{% end %}
### `break`

Loops can be terminated with the `break` keyword.

````koto
x = 0
while x < 100000
  if x >= 3
    # Break out of the loop when x is greater or equal to 3
    break
  x += 1
x
# -> 3
````

{% example_playground_link(version = "0.15") %}
x = 0
while x < 100000
  if x >= 3
    # Break out of the loop when x is greater or equal to 3
    break
  x += 1
print x
# -> 3

{% end %}
A value can be provided to `break`, which is then used as the result of the loop.

````koto
x = 0
y = while x < 100000
  if x >= 3
    # Break out of the loop, providing x + 100 as the loop's result
    break x + 100
  x += 1
y
# -> 103
````

{% example_playground_link(version = "0.15") %}
x = 0
y = while x < 100000
  if x >= 3
    # Break out of the loop, providing x + 100 as the loop's result
    break x + 100
  x += 1
print y
# -> 103

{% end %}
### `loop`

`loop` creates a loop that will repeat indefinitely.

````koto
x = 0
y = loop
  x += 1
  # Stop looping when x is greater than 4
  if x > 4
    break x * x
y
# -> 25
````

{% example_playground_link(version = "0.15") %}
x = 0
y = loop
  x += 1
  # Stop looping when x is greater than 4
  if x > 4
    break x * x
print y
# -> 25

{% end %}
## Iterators

The elements of a sequence can be accessed sequentially with an *iterator*,
created using the `.iter()` function.

An iterator yields values via [`.next()`](../core/iterator#next) until the end of the sequence is
reached, when `null` is returned.

````koto
i = [10, 20].iter()

i.next()
# -> IteratorOutput(10)
i.next()
# -> IteratorOutput(20)
i.next()
# -> null
````

{% example_playground_link(version = "0.15") %}
i = [10, 20].iter()

print i.next()
# -> IteratorOutput(10)
print i.next()
# -> IteratorOutput(20)
print i.next()
# -> null

{% end %}
### Iterator Generators

The [`iterator` module](../core/iterator) contains iterator *generators* like
[`once`](../core/iterator#once) and [`repeat`](../core/iterator#repeat) that generate output values
[*lazily*](https://en.wikipedia.org/wiki/Lazy_evaluation) during iteration.

````koto
# Create an iterator that repeats ! twice
i = iterator.repeat('!', 2)
i.next()
# -> IteratorOutput(!)
i.next()
# -> IteratorOutput(!)
i.next()
# -> null
````

{% example_playground_link(version = "0.15") %}
# Create an iterator that repeats ! twice
i = iterator.repeat('!', 2)
print i.next()
# -> IteratorOutput(!)
print i.next()
# -> IteratorOutput(!)
print i.next()
# -> null

{% end %}
### Iterator Adaptors

The output of an iterator can be modified using *adaptors* from the
[`iterator` module](../core/iterator).

````koto
# Create an iterator that keeps any value above 3
x = [1, 2, 3, 4, 5].keep |n| n > 3

x.next()
# -> IteratorOutput(4)
x.next()
# -> IteratorOutput(5)
x.next()
# -> null
````

{% example_playground_link(version = "0.15") %}
# Create an iterator that keeps any value above 3
x = [1, 2, 3, 4, 5].keep |n| n > 3

print x.next()
# -> IteratorOutput(4)
print x.next()
# -> IteratorOutput(5)
print x.next()
# -> null

{% end %}
### Using iterators with `for`

`for` loops accept any iterable value as input, including adapted iterators.

````koto
for x in 'abacad'.keep |c| c != 'a'
  print x
# -> b
# -> c
# -> d
````

{% example_playground_link(version = "0.15") %}
for x in 'abacad'.keep |c| c != 'a'
  print x
# -> b
# -> c
# -> d

{% end %}
### Iterator Chains

Iterator adaptors can be passed into other adaptors, creating *iterator chains*
that act as data processing pipelines.

````koto
i = (1, 2, 3, 4, 5)
  .skip 1
  .each |n| n * 10
  .keep |n| n <= 40
  .intersperse '--'

for x in i
  print x
# -> 20
# -> --
# -> 30
# -> --
# -> 40
````

{% example_playground_link(version = "0.15") %}
i = (1, 2, 3, 4, 5)
  .skip 1
  .each |n| n * 10
  .keep |n| n <= 40
  .intersperse '--'

for x in i
  print x
# -> 20
# -> --
# -> 30
# -> --
# -> 40

{% end %}
### Iterator Consumers

Iterators can also be *consumed* using functions like
[`.to_list()`](../core/iterator#to-list) and [`.to_tuple()`](../core/iterator#to-tuple),
allowing the output of an iterator to be easily captured in a container.

````koto
[1, 2, 3]
  .each |n| n * 2
  .to_tuple()
# -> (2, 4, 6)

(1, 2, 3, 4)
  .keep |n| n % 2 == 0
  .each |n| n * 11
  .to_list()
# -> [22, 44]
````

{% example_playground_link(version = "0.15") %}
print [1, 2, 3]
  .each |n| n * 2
  .to_tuple()
# -> (2, 4, 6)

print (1, 2, 3, 4)
  .keep |n| n % 2 == 0
  .each |n| n * 11
  .to_list()
# -> [22, 44]

{% end %}
## Value Unpacking

Multiple assignments can be performed in a single expression by separating the
variable names with commas.

````koto
a, b = 10, 20
a, b
# -> (10, 20)
````

{% example_playground_link(version = "0.15") %}
a, b = 10, 20
print a, b
# -> (10, 20)

{% end %}
If there's a single value being assigned, and the value is iterable,
then it gets *unpacked* into the target variables.

````koto
my_tuple = 1, 2
x, y = my_tuple
y, x
# -> (2, 1)
````

{% example_playground_link(version = "0.15") %}
my_tuple = 1, 2
x, y = my_tuple
print y, x
# -> (2, 1)

{% end %}
Unpacking works with any iterable value, including adapted iterators.

````koto
a, b, c = [1, 2, 3, 4, 5]
a, b, c
# -> (1, 2, 3)

x, y, z = 'a-b-c'.split '-'
x, y, z
# -> ('a', 'b', 'c')
````

{% example_playground_link(version = "0.15") %}
a, b, c = [1, 2, 3, 4, 5]
print a, b, c
# -> (1, 2, 3)

x, y, z = 'a-b-c'.split '-'
print x, y, z
# -> ('a', 'b', 'c')

{% end %}
If the value being unpacked doesn't contain enough values for the assignment,
then `null` is assigned to any remaining variables.

````koto
a, b, c = [-1, -2]
a, b, c
# -> (-1, -2, null)

x, y, z = 42
x, y, z
# -> (42, null, null)
````

{% example_playground_link(version = "0.15") %}
a, b, c = [-1, -2]
print a, b, c
# -> (-1, -2, null)

x, y, z = 42
print x, y, z
# -> (42, null, null)

{% end %}
Unpacking can also be used in `for` loops, which is particularly useful when
looping over the contents of a map.

````koto
my_map = {foo: 42, bar: 99}
for key, value in my_map
  print key, value
# -> ('foo', 42)
# -> ('bar', 99)
````

{% example_playground_link(version = "0.15") %}
my_map = {foo: 42, bar: 99}
for key, value in my_map
  print key, value
# -> ('foo', 42)
# -> ('bar', 99)

{% end %}
## Generators

Generators are iterators that are made by calling *generator functions*,
which are any functions that contain a `yield` expression.

The generator is paused each time `yield` is encountered,
waiting for the caller to continue execution.

````koto
my_first_generator = ||
  yield 1
  yield 2

x = my_first_generator()
x.next()
# -> IteratorOutput(1)
x.next()
# -> IteratorOutput(2)
x.next()
# -> null
````

{% example_playground_link(version = "0.15") %}
my_first_generator = ||
  yield 1
  yield 2

x = my_first_generator()
print x.next()
# -> IteratorOutput(1)
print x.next()
# -> IteratorOutput(2)
print x.next()
# -> null

{% end %}
Generator functions can accept arguments like any other function,
and each time they're called a new generator is created.

As with any other iterable value, the [`iterator`](../core/iterator) module's functions
are made available to generators.

````koto
make_generator = |x|
  for y in (1, 2, 3)
    yield x + y

make_generator(0).to_tuple()
# -> (1, 2, 3)
make_generator(10)
  .keep |n| n % 2 == 1
  .to_list()
# -> [11, 13]
````

{% example_playground_link(version = "0.15") %}
make_generator = |x|
  for y in (1, 2, 3)
    yield x + y

print make_generator(0).to_tuple()
# -> (1, 2, 3)
print make_generator(10)
  .keep |n| n % 2 == 1
  .to_list()
# -> [11, 13]

{% end %}
### Custom Iterator Adaptors

Generators can also serve as *iterator adaptors* by modifying the output of
another iterator.

Inserting a generator into the [`iterator`](../core/iterator) module makes it available
in any iterator chain.

````koto
# Make an iterator adaptor that yields every
# other value from the adapted iterator
iterator.every_other = ||
  n = 0
  # When the generator is created, self is initialized with the previous
  # iterator in the chain, allowing its output to be adapted.
  for output in self
    # If n is even, then yield a value
    if n % 2 == 0
      yield output
    n += 1

(1, 2, 3, 4, 5)
  .each |n| n * 10
  .every_other() # Skip over every other value in the iterator chain
  .to_list()
# -> [10, 30, 50]
````

{% example_playground_link(version = "0.15") %}
# Make an iterator adaptor that yields every
# other value from the adapted iterator
iterator.every_other = ||
  n = 0
  # When the generator is created, self is initialized with the previous
  # iterator in the chain, allowing its output to be adapted.
  for output in self
    # If n is even, then yield a value
    if n % 2 == 0
      yield output
    n += 1

print (1, 2, 3, 4, 5)
  .each |n| n * 10
  .every_other() # Skip over every other value in the iterator chain
  .to_list()
# -> [10, 30, 50]

{% end %}
## Ranges

Ranges of integers can be created with `..` or `..=`.

`..` creates a *non-inclusive* range,
which defines a range up to but *not including* the end of the range.

````koto
# Create a range from 10 to 20, not including 20
r = 10..20
# -> 10..20
r.start()
# -> 10
r.end()
# -> 20
r.contains 20
# -> false
````

{% example_playground_link(version = "0.15") %}
# Create a range from 10 to 20, not including 20
print r = 10..20
# -> 10..20
print r.start()
# -> 10
print r.end()
# -> 20
print r.contains 20
# -> false

{% end %}
`..=` creates an *inclusive* range, which includes the end of the range.

````koto
# Create a range from 10 to 20, including 20
r = 10..=20
# -> 10..=20
r.contains 20
# -> true
````

{% example_playground_link(version = "0.15") %}
# Create a range from 10 to 20, including 20
print r = 10..=20
# -> 10..=20
print r.contains 20
# -> true

{% end %}
If a value is missing from either side of the range operator then an *unbounded*
range is created.

````koto
# Create an unbounded range starting from 10
r = 10..
r.start()
# -> 10
r.end()
# -> null

# Create an unbounded range up to and including 100
r = ..=100
r.start()
# -> null
r.end()
# -> 100
````

{% example_playground_link(version = "0.15") %}
# Create an unbounded range starting from 10
r = 10..
print r.start()
# -> 10
print r.end()
# -> null

# Create an unbounded range up to and including 100
r = ..=100
print r.start()
# -> null
print r.end()
# -> 100

{% end %}
*Bounded* ranges are declared as iterable,
so they can be used in for loops and with the [`iterator`](../core/iterator) module.

````koto
for x in 1..=3
  print x
# -> 1
# -> 2
# -> 3

(0..5).to_list()
# -> [0, 1, 2, 3, 4]
````

{% example_playground_link(version = "0.15") %}
for x in 1..=3
  print x
# -> 1
# -> 2
# -> 3

print (0..5).to_list()
# -> [0, 1, 2, 3, 4]

{% end %}
### Slices

Ranges can be used to create a *slice* of a container's data.

````koto
x = (10, 20, 30, 40, 50)
x[1..=3]
# -> (20, 30, 40)
````

{% example_playground_link(version = "0.15") %}
x = (10, 20, 30, 40, 50)
print x[1..=3]
# -> (20, 30, 40)

{% end %}
For immutable containers like tuples and strings,
slices share the original value's data, with no copies being made.

For mutable containers like lists, creating a slice makes a copy of the sliced
portion of the underlying data.

````koto
x = 'abcdef'
# No copies are made when a string is sliced
y = x[3..6]
# -> def

a = [1, 2, 3]
# When a list is sliced, the sliced elements get copied into a new list
b = a[0..2]
# -> [1, 2]
b[0] = 42
# -> 42
a[0]
# -> 1
````

{% example_playground_link(version = "0.15") %}
x = 'abcdef'
# No copies are made when a string is sliced
print y = x[3..6]
# -> def

a = [1, 2, 3]
# When a list is sliced, the sliced elements get copied into a new list
print b = a[0..2]
# -> [1, 2]
print b[0] = 42
# -> 42
print a[0]
# -> 1

{% end %}
When creating a slice with an unbounded range,
if the start of the range if omitted then the slice starts from the beginning of the container.
If the end of the range is omitted, then the slice includes all remaining elements in the container.

````koto
z = 'Hëllø'.to_tuple()
z[..2]
# -> ('H', 'ë')
z[2..]
# -> ('l', 'l', 'ø')
````

{% example_playground_link(version = "0.15") %}
z = 'Hëllø'.to_tuple()
print z[..2]
# -> ('H', 'ë')
print z[2..]
# -> ('l', 'l', 'ø')

{% end %}
## Type Checks

Koto is a primarily a dynamically typed language, however in more complex programs
you might find it beneficial to add type checks.

These checks can help in catching errors earlier, and can also act as
documentation for the reader.

One way to add type checks to your program is to use the
[`type`](../core/koto#type) function, which returns a value's type as a string.

````koto
x = 123
assert_eq (type x), 'Number'
````

{% example_playground_link(version = "0.15") %}
x = 123
assert_eq (type x), 'Number'

{% end %}
Checking types this way is rather verbose, so Koto offers *type hints* as a more
ergonomic alternative.

### `let`

You can declare variables with type hints using a `let` expression.

If a value is assigned that doesn't match the declared type then an error will
be thrown.

````koto
let x: String = 'hello'
x
# -> hello

let a: Number, _, c: Bool = 123, x, true
a, c
# -> (123, true)
````

{% example_playground_link(version = "0.15") %}
let x: String = 'hello'
print x
# -> hello

let a: Number, _, c: Bool = 123, x, true
print a, c
# -> (123, true)

{% end %}
### `for` arguments

Type hints can also be added to `for` loop arguments.
The type will be checked on each iteration of the loop.

````koto
for i: Number, s: String in 'abc'.enumerate()
  print i, s
# -> (0, 'a')
# -> (1, 'b')
# -> (2, 'c')
````

{% example_playground_link(version = "0.15") %}
for i: Number, s: String in 'abc'.enumerate()
  print i, s
# -> (0, 'a')
# -> (1, 'b')
# -> (2, 'c')

{% end %}
### Functions

Function arguments can also be given type hints, and the type of the
return value can be checked with the `->` operator.

````koto
f = |s: String| -> Tuple
  s.to_tuple()
f 'abc'
# -> ('a', 'b', 'c')
````

{% example_playground_link(version = "0.15") %}
f = |s: String| -> Tuple
  s.to_tuple()
print f 'abc'
# -> ('a', 'b', 'c')

{% end %}
For [generator functions](#generators), the `->` type hint is used to check
the generator's `yield` expressions.

````koto
g = || -> Number
  yield 1
  yield 2
  yield 3
g().to_tuple()
# -> (1, 2, 3)
````

{% example_playground_link(version = "0.15") %}
g = || -> Number
  yield 1
  yield 2
  yield 3
print g().to_tuple()
# -> (1, 2, 3)

{% end %}
### `match` patterns

Type hints can be used in `match` patterns to check the type of the a value.
Rather than throwing an error, if a type check fails then the next
match pattern will be attempted.

````koto
match 'abc'
  x: Tuple then x
  x: String then x.to_tuple()
# -> ('a', 'b', 'c')
````

{% example_playground_link(version = "0.15") %}
print match 'abc'
  x: Tuple then x
  x: String then x.to_tuple()
# -> ('a', 'b', 'c')

{% end %}
### Optional Values

Sometimes a value can either be of a particular type, or otherwise it should `null`.

These kinds of values are referred to as [*optional*](https://en.wikipedia.org/wiki/Option_type),
and are useful for functions or expressions that return either a valid value, or nothing at all.

Optional value types are expressed by appending `?` to the type hint.

````koto
m = {foo: 'hi!'}

let foo: String? = m.get('foo')?.to_uppercase()
# -> HI!

let bar: String? = m.get('bar')?.to_uppercase()
# -> null
````

{% example_playground_link(version = "0.15") %}
m = {foo: 'hi!'}

print let foo: String? = m.get('foo')?.to_uppercase()
# -> HI!

print let bar: String? = m.get('bar')?.to_uppercase()
# -> null

{% end %}
### Special Types

#### `Any`

The `Any` type will result in a successful check with any value.

````koto
let x: Any = 'hello'
# -> hello
````

{% example_playground_link(version = "0.15") %}
print let x: Any = 'hello'
# -> hello

{% end %}
#### `Callable`

The `Callable` type hint will accept functions, or any object that can behave
like a function.

````koto
let say_hello: Callable = || 'hello'
say_hello()
# -> hello
````

{% example_playground_link(version = "0.15") %}
let say_hello: Callable = || 'hello'
print say_hello()
# -> hello

{% end %}
#### `Indexable`

The `Indexable` type hint will accept any value that supports `[]` indexing.

````koto
add_first_two = |x: Indexable| x[0] + x[1]
add_first_two (100, 99, -1)
# -> 199
````

{% example_playground_link(version = "0.15") %}
add_first_two = |x: Indexable| x[0] + x[1]
print add_first_two (100, 99, -1)
# -> 199

{% end %}
#### `Iterable`

The `Iterable` type is useful when any iterable value can be accepted.

````koto
let a: Iterable, b: Iterable = [1, 2], 3..=5
a.chain(b).to_tuple()
# -> (1, 2, 3, 4, 5)
````

{% example_playground_link(version = "0.15") %}
let a: Iterable, b: Iterable = [1, 2], 3..=5
print a.chain(b).to_tuple()
# -> (1, 2, 3, 4, 5)

{% end %}
## String Formatting

Interpolated string expressions can be formatted using formatting options
similar to [Rust's](https://doc.rust-lang.org/std/fmt/#formatting-parameters).

Inside an interpolated expression, options are provided after a `:` separator.

````koto
'{number.pi:𝜋^8.2}'
# -> 𝜋𝜋3.14𝜋𝜋
````

{% example_playground_link(version = "0.15") %}
print '{number.pi:𝜋^8.2}'
# -> 𝜋𝜋3.14𝜋𝜋

{% end %}
### Minimum Width and Alignment

A minimum width can be specified, ensuring that the formatted value takes up at
least that many characters.

````koto
foo = "abcd"
'_{foo:8}_'
# -> _abcd    _
````

{% example_playground_link(version = "0.15") %}
foo = "abcd"
print '_{foo:8}_'
# -> _abcd    _

{% end %}
The minimum width can be prefixed with an alignment modifier:

* `<` - left-aligned
* `^` - centered
* `>` - right-aligned

````koto
foo = "abcd"
'_{foo:^8}_'
# -> _  abcd  _
````

{% example_playground_link(version = "0.15") %}
foo = "abcd"
print '_{foo:^8}_'
# -> _  abcd  _

{% end %}
All values are left-aligned if an alignment modifier isn't specified,
except for numbers which are right-aligned by default.

````koto
x = 1.2
'_{x:8}_'
# -> _     1.2_
````

{% example_playground_link(version = "0.15") %}
x = 1.2
print '_{x:8}_'
# -> _     1.2_

{% end %}
The alignment modifier can be prefixed with a character which will be used to
fill any empty space in the formatted string (the default character being ` `).

````koto
x = 1.2
'_{x:~<8}_'
# -> _1.2~~~~~_
````

{% example_playground_link(version = "0.15") %}
x = 1.2
print '_{x:~<8}_'
# -> _1.2~~~~~_

{% end %}
For numbers, the minimum width can be prefixed with `0`, which will pad the
number to the specified width with zeroes.

````koto
x = 1.2
'{x:06}'
# -> 0001.2
````

{% example_playground_link(version = "0.15") %}
x = 1.2
print '{x:06}'
# -> 0001.2

{% end %}
### Maximum Width / Precision

A maximum width for the interpolated expression can be specified following a
`.` character.

````koto
foo = "abcd"
'{foo:_^8.2}'
# -> ___ab___
````

{% example_playground_link(version = "0.15") %}
foo = "abcd"
print '{foo:_^8.2}'
# -> ___ab___

{% end %}
For numbers, the maximum width acts as a 'precision' value, or in other words,
the number of decimal places that will be rendered for the number.

````koto
x = 1 / 3
'{x:.4}'
# -> 0.3333
````

{% example_playground_link(version = "0.15") %}
x = 1 / 3
print '{x:.4}'
# -> 0.3333

{% end %}
## Advanced Functions

Functions in Koto have some advanced features that are worth exploring.

### Captured Variables

When a variable is accessed in a function that wasn't declared locally,
then it gets *captured* by copying it into the function.

````koto
x = 1

my_function = |n|
  # x is assigned outside the function,
  # so it gets captured when the function is created.
  n + x

# Reassigning x here doesn't modify the value
# of x that was captured when my_function was created.
x = 100

my_function 2
# -> 3
````

{% example_playground_link(version = "0.15") %}
x = 1

my_function = |n|
  # x is assigned outside the function,
  # so it gets captured when the function is created.
  n + x

# Reassigning x here doesn't modify the value
# of x that was captured when my_function was created.
x = 100

print my_function 2
# -> 3

{% end %}
This behavior is different to many other languages,
where captures are often taken by *reference* rather than by *copy*.

It's also worth noting that captured variables will have the same starting value
each time the function is called.

````koto
x = 99
f = ||
  # Modifying x only happens with a local copy during a function call.
  # The value of x at the start of the call matches when the value it had when
  # it was captured.
  x += 1

f(), f(), f()
# -> (100, 100, 100)
````

{% example_playground_link(version = "0.15") %}
x = 99
f = ||
  # Modifying x only happens with a local copy during a function call.
  # The value of x at the start of the call matches when the value it had when
  # it was captured.
  x += 1

print f(), f(), f()
# -> (100, 100, 100)

{% end %}
To modify captured values, use a container (like a map) to hold on to mutable
data.

````koto
data = {x: 99}

f = ||
  # The data map gets captured by the function,
  # and its contained values can be modified between calls.
  data.x += 1

f(), f(), f()
# -> (100, 101, 102)
````

{% example_playground_link(version = "0.15") %}
data = {x: 99}

f = ||
  # The data map gets captured by the function,
  # and its contained values can be modified between calls.
  data.x += 1

print f(), f(), f()
# -> (100, 101, 102)

{% end %}
### Optional Arguments

When calling a function, any missing arguments will be replaced by `null`.

````koto
f = |a, b, c|
  print a, b, c

f 1
# -> (1, null, null)
f 1, 2
# -> (1, 2, null)
f 1, 2, 3
# -> (1, 2, 3)
````

{% example_playground_link(version = "0.15") %}
f = |a, b, c|
  print a, b, c

f 1
# -> (1, null, null)
f 1, 2
# -> (1, 2, null)
f 1, 2, 3
# -> (1, 2, 3)

{% end %}
Missing arguments can be replaced with default values by using `or`.

````koto
f = |a, b, c|
  print a or -1, b or -2, c or -3

f 42
# -> (42, -2, -3)
f 99, 100
# -> (99, 100, -3)
````

{% example_playground_link(version = "0.15") %}
f = |a, b, c|
  print a or -1, b or -2, c or -3

f 42
# -> (42, -2, -3)
f 99, 100
# -> (99, 100, -3)

{% end %}
`or` will reject `false`, so if `false` would be a valid input then a
direct comparison against `null` can be used instead.

````koto
f = |a|
  print if a == null then -1 else a

f()
# -> -1
f false
# -> false
````

{% example_playground_link(version = "0.15") %}
f = |a|
  print if a == null then -1 else a

f()
# -> -1
f false
# -> false

{% end %}
### Variadic Functions

A [*variadic function*](https://en.wikipedia.org/wiki/Variadic_function) can be created by appending `...` to the
last argument.
When the function is called any extra arguments will be collected into a tuple.

````koto
f = |a, b, others...|
  print "a: {a}, b: {b}, others: {others}"

f 1, 2, 3, 4, 5
# -> a: 1, b: 2, others: (3, 4, 5)
````

{% example_playground_link(version = "0.15") %}
f = |a, b, others...|
  print "a: {a}, b: {b}, others: {others}"

f 1, 2, 3, 4, 5
# -> a: 1, b: 2, others: (3, 4, 5)

{% end %}
### Argument Unpacking

Functions that expect containers as arguments can *unpack* the contained
elements directly in the argument declaration by using parentheses.

````koto
# A function that sums a container with three contained values
f = |(a, b, c)| a + b + c

x = [100, 10, 1]
f x
# -> 111
````

{% example_playground_link(version = "0.15") %}
# A function that sums a container with three contained values
f = |(a, b, c)| a + b + c

x = [100, 10, 1]
print f x
# -> 111

{% end %}
Any container that supports indexing operations (like lists and tuples)
with a matching number of elements will be unpacked,
otherwise an error will be thrown.

Unpacked arguments can also be nested.

````koto
# A function that sums elements from nested containers
f = |((a, b), (c, d, e))|
  a + b + c + d + e
x = ([1, 2], [3, 4, 5])
f x
# -> 15
````

{% example_playground_link(version = "0.15") %}
# A function that sums elements from nested containers
f = |((a, b), (c, d, e))|
  a + b + c + d + e
x = ([1, 2], [3, 4, 5])
print f x
# -> 15

{% end %}
Ellipses can be used to unpack any number of elements at the start or end of a
container.

````koto
f = |(..., last)| last * last
x = (1, 2, 3, 4)
f x
# -> 16
````

{% example_playground_link(version = "0.15") %}
f = |(..., last)| last * last
x = (1, 2, 3, 4)
print f x
# -> 16

{% end %}
A name can be added to ellipses to assign the unpacked elements.

````koto
f = |(first, others...)| first * others.sum()
x = (10, 1, 2, 3)
f x
# -> 60
````

{% example_playground_link(version = "0.15") %}
f = |(first, others...)| first * others.sum()
x = (10, 1, 2, 3)
print f x
# -> 60

{% end %}
### Ignoring Arguments

The wildcard `_` can be used to ignore function arguments.

````koto
# A function that sums the first and third elements of a container
f = |(a, _, c)| a + c

f [100, 10, 1]
# -> 101
````

{% example_playground_link(version = "0.15") %}
# A function that sums the first and third elements of a container
f = |(a, _, c)| a + c

print f [100, 10, 1]
# -> 101

{% end %}
If you would like to keep the name of the ignored value as a reminder,
then `_` can be used as a prefix for an identifier. Identifiers starting with
`_` can be written to, but can't be accessed.

````koto
my_map = {foo_a: 1, bar_a: 2, foo_b: 3, bar_b: 4}
my_map
  .keep |(key, _value)| key.starts_with 'foo'
  .to_tuple()
# -> (('foo_a', 1), ('foo_b', 3))
````

{% example_playground_link(version = "0.15") %}
my_map = {foo_a: 1, bar_a: 2, foo_b: 3, bar_b: 4}
print my_map
  .keep |(key, _value)| key.starts_with 'foo'
  .to_tuple()
# -> (('foo_a', 1), ('foo_b', 3))

{% end %}
## Objects and Metamaps

Value types with custom behaviour can be defined in Koto through the concept of
*objects*.

An object is any map that includes one or more *metakeys*
(keys prefixed with `@`), that are stored in the object's *metamap*.
Whenever operations are performed on the object, the runtime checks its metamap
for corresponding metakeys.

In the following example, addition and subtraction operators are overridden for
a custom `Foo` object:

````koto
# Declare a function that makes Foo objects
foo = |n|
  data: n

  # Overriding the addition operator
  @+: |other|
    # A new Foo is made using the result
    # of adding the two data values together
    foo self.data + other.data

  # Overriding the subtraction operator
  @-: |other|
    foo self.data - other.data

  # Overriding the multiply-assignment operator
  @*=: |other|
    self.data *= other.data
    self

a = foo 10
b = foo 20

(a + b).data
# -> 30
(a - b).data
# -> -10
a *= b
a.data
# -> 200
````

{% example_playground_link(version = "0.15") %}
# Declare a function that makes Foo objects
foo = |n|
  data: n

  # Overriding the addition operator
  @+: |other|
    # A new Foo is made using the result
    # of adding the two data values together
    foo self.data + other.data

  # Overriding the subtraction operator
  @-: |other|
    foo self.data - other.data

  # Overriding the multiply-assignment operator
  @*=: |other|
    self.data *= other.data
    self

a = foo 10
b = foo 20

print (a + b).data
# -> 30
print (a - b).data
# -> -10
a *= b
print a.data
# -> 200

{% end %}
### Meta Operators

All of the binary arithmetic and logic operators (`*`, `<`, `>=`, etc) can be
implemented following this pattern.

Additionally, the following metakeys can also be defined:

#### `@negate`

The `@negate` metakey overrides the negation operator.

````koto
foo = |n|
  data: n
  @negate: || foo -self.data

x = -foo(100)
x.data
# -> -100
````

{% example_playground_link(version = "0.15") %}
foo = |n|
  data: n
  @negate: || foo -self.data

x = -foo(100)
print x.data
# -> -100

{% end %}
#### `@size` and `@index`

The `@size` metakey defines how the object should report its size,
while the `@index` metakey defines what values should be returned when indexing is
performed on the object.

If `@size` is implemented, then `@index` should also be implemented.

````koto
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo ('a', 'b', 'c')
size x
# -> 3
x[1]
# -> b
````

{% example_playground_link(version = "0.15") %}
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo ('a', 'b', 'c')
print size x
# -> 3
print x[1]
# -> b

{% end %}
Implementing `@size` and `@index` allows an object to participate in argument unpacking.

The `@index` implementation can support indexing by any input values that make
sense for your object type, however for argument unpacking to work correctly, the
runtime expects that indexing should be supported for at least single indices and ranges.

````koto
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo (10, 20, 30, 40, 50)

# Unpack the first two elements in the value passed to the function and multiply them
multiply_first_two = |(a, b, ...)| a * b
multiply_first_two x
# -> 200

# Inspect the first element in the object
match x
  (first, others...) then 'first: {first}, remaining: {size others}'
# -> first: 10, remaining: 4
````

{% example_playground_link(version = "0.15") %}
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo (10, 20, 30, 40, 50)

# Unpack the first two elements in the value passed to the function and multiply them
multiply_first_two = |(a, b, ...)| a * b
print multiply_first_two x
# -> 200

# Inspect the first element in the object
print match x
  (first, others...) then 'first: {first}, remaining: {size others}'
# -> first: 10, remaining: 4

{% end %}
#### `@index_mut`

The `@index_mut` metakey defines how the object should behave when index-assignment is used.

The given value should be a function that takes an index as the first argument, and the value to be assigned as the second argument.

````koto
foo = |data|
  data: data
  @index: |index| self.data[index]
  @index_mut: |index, value| self.data[index] = value

x = foo ['a', 'b', 'c']
x[1] = 'hello'
x[1]
# -> hello
````

{% example_playground_link(version = "0.15") %}
foo = |data|
  data: data
  @index: |index| self.data[index]
  @index_mut: |index, value| self.data[index] = value

x = foo ['a', 'b', 'c']
x[1] = 'hello'
print x[1]
# -> hello

{% end %}
#### `@call`

The `@call` metakey defines how the object should behave when its called as a
function.

````koto
foo = |n|
  data: n
  @call: ||
    self.data *= 2
    self.data

x = foo 2
x()
# -> 4
x()
# -> 8
````

{% example_playground_link(version = "0.15") %}
foo = |n|
  data: n
  @call: ||
    self.data *= 2
    self.data

x = foo 2
print x()
# -> 4
print x()
# -> 8

{% end %}
#### `@iterator`

The `@iterator` metakey defines how iterators should be created when the object
is used in an iterable context.
When called, `@iterator` should return an iterable value that will then be used
for iterator operations.

````koto
foo = |n|
  # Return a generator that yields the three numbers following n
  @iterator: ||
    yield n + 1
    yield n + 2
    yield n + 3

(foo 0).to_tuple()
# -> (1, 2, 3)

(foo 100).to_list()
# -> [101, 102, 103]
````

{% example_playground_link(version = "0.15") %}
foo = |n|
  # Return a generator that yields the three numbers following n
  @iterator: ||
    yield n + 1
    yield n + 2
    yield n + 3

print (foo 0).to_tuple()
# -> (1, 2, 3)

print (foo 100).to_list()
# -> [101, 102, 103]

{% end %}
Note that this key will be ignored if the object also implements `@next`,
which implies that the object is *already* an iterator.

#### `@next`

The `@next` metakey allows for objects to behave as iterators.

Whenever the runtime needs to produce an iterator from an object, it will first
check the metamap for an implementation of `@next`, before looking for
`@iterator`.

The `@next` function will be called repeatedly during iteration,
with the returned value being used as the iterator's output.
When the returned value is `null` then the iterator will stop producing output.

````koto
foo = |start, end|
  start: start
  end: end
  @next: ||
    if self.start < self.end
      result = self.start
      self.start += 1
      result
    else
      null

foo(10, 15).to_tuple()
# -> (10, 11, 12, 13, 14)
````

{% example_playground_link(version = "0.15") %}
foo = |start, end|
  start: start
  end: end
  @next: ||
    if self.start < self.end
      result = self.start
      self.start += 1
      result
    else
      null

print foo(10, 15).to_tuple()
# -> (10, 11, 12, 13, 14)

{% end %}
#### `@next_back`

The `@next_back` metakey is used by
[`iterator.reversed`](../core/iterator#reversed) when producing a reversed
iterator.

The runtime will only look for `@next_back` if `@next` is implemented.

````koto
foo =
  n: 0
  @next: || self.n += 1
  @next_back: || self.n -= 1

foo
  .skip 3 # 0, 1, 2
  .reversed()
  .take 3 # 2, 1, 0
  .to_tuple()
# -> (2, 1, 0)
````

{% example_playground_link(version = "0.15") %}
foo =
  n: 0
  @next: || self.n += 1
  @next_back: || self.n -= 1

print foo
  .skip 3 # 0, 1, 2
  .reversed()
  .take 3 # 2, 1, 0
  .to_tuple()
# -> (2, 1, 0)

{% end %}
#### `@display`

The `@display` metakey defines how the object should be represented when
displaying the object as a string.

````koto
foo = |n|
  data: n
  @display: || 'Foo({self.data})'

foo 42
# -> Foo(42)

x = foo -1
"The value of x is '{x}'"
# -> The value of x is 'Foo(-1)'
````

{% example_playground_link(version = "0.15") %}
foo = |n|
  data: n
  @display: || 'Foo({self.data})'

print foo 42
# -> Foo(42)

x = foo -1
print "The value of x is '{x}'"
# -> The value of x is 'Foo(-1)'

{% end %}
#### `@type`

The `@type` metakey takes a string as a value which is used when checking the
value's type, e.g. with [`koto.type`](../core/koto#type)

````koto
foo = |n|
  data: n
  @type: "Foo"

koto.type (foo 42)
# -> Foo
````

{% example_playground_link(version = "0.15") %}
foo = |n|
  data: n
  @type: "Foo"

print koto.type (foo 42)
# -> Foo

{% end %}
#### `@base`

Objects can inherit properties and behavior from other values,
establishing a *base value* through the `@base` metakey.
This allows objects to share common functionality while maintaining their own
unique attributes.

In the following example, two kinds of animals are created that share the
`speak` function from their base value.

````koto
animal = |name|
  name: name
  speak: || '{self.noise}! My name is {self.name}!'

dog = |name|
  @base: animal name
  noise: 'Woof'

cat = |name|
  @base: animal name
  noise: 'Meow'

dog('Fido').speak()
# -> Woof! My name is Fido!

cat('Smudge').speak()
# -> Meow! My name is Smudge!
````

{% example_playground_link(version = "0.15") %}
animal = |name|
  name: name
  speak: || '{self.noise}! My name is {self.name}!'

dog = |name|
  @base: animal name
  noise: 'Woof'

cat = |name|
  @base: animal name
  noise: 'Meow'

print dog('Fido').speak()
# -> Woof! My name is Fido!

print cat('Smudge').speak()
# -> Meow! My name is Smudge!

{% end %}
#### `@meta`

The `@meta` metakey allows named metakeys to be added to the metamap.
Metakeys defined with `@meta` are accessible via `.` access,
similar to regular object `keys`, but they don't appear as part of the object's
main data entries when treated as a regular map.

````koto
foo = |n|
  data: n
  @meta hello: "Hello!"
  @meta get_info: ||
    info = match self.data
      0 then "zero"
      n if n < 0 then "negative"
      else "positive"
    "{self.data} is {info}"

x = foo -1
x.hello
# -> Hello!

print x.get_info()
# -> -1 is negative

print map.keys(x).to_tuple()
# -> ('data')
````

{% example_playground_link(version = "0.15") %}
foo = |n|
  data: n
  @meta hello: "Hello!"
  @meta get_info: ||
    info = match self.data
      0 then "zero"
      n if n < 0 then "negative"
      else "positive"
    "{self.data} is {info}"

x = foo -1
print x.hello
# -> Hello!

print x.get_info()
# -> -1 is negative

print map.keys(x).to_tuple()
# -> ('data')

{% end %}
### Sharing Metamaps

Metamaps can be shared between objects by using
[`Map.with_meta`](../core/map#with-meta), which helps to avoid inefficient
duplication when creating a lot of objects.

In the following example, behavior is overridden in a single metamap, which is
then shared between object instances.

````koto
# Create an empty map for global values
global = {}

# Define a function that makes a Foo object
foo = |data|
  # Make a new map that contains `data`,
  # and then attach a shared copy of the metamap from foo_meta.
  {data}.with_meta global.foo_meta

# Define some metakeys in foo_meta
global.foo_meta =
  # Override the + operator
  @+: |other| foo self.data + other.data

  # Define how the object should be displayed
  @display: || "Foo({self.data})"

(foo 10) + (foo 20)
# -> Foo(30)
````

{% example_playground_link(version = "0.15") %}
# Create an empty map for global values
global = {}

# Define a function that makes a Foo object
foo = |data|
  # Make a new map that contains `data`,
  # and then attach a shared copy of the metamap from foo_meta.
  {data}.with_meta global.foo_meta

# Define some metakeys in foo_meta
global.foo_meta =
  # Override the + operator
  @+: |other| foo self.data + other.data

  # Define how the object should be displayed
  @display: || "Foo({self.data})"

print (foo 10) + (foo 20)
# -> Foo(30)

{% end %}
## Error Handling

Errors can be *thrown* in the Koto runtime, which then cause the runtime to stop
execution.

A `try` / `catch` expression can be used to *catch* any thrown errors,
allowing execution to continue.
An optional `finally` block can be used for cleanup actions that need to
performed whether or not an error was caught.

````koto
x = [1, 2, 3]
try
  # Accessing an invalid index will throw an error
  print x[100]
catch error
  print "Caught an error"
finally
  print "...and finally"
# -> Caught an error
# -> ...and finally
````

{% example_playground_link(version = "0.15") %}
x = [1, 2, 3]
try
  # Accessing an invalid index will throw an error
  print x[100]
catch error
  print "Caught an error"
finally
  print "...and finally"
# -> Caught an error
# -> ...and finally

{% end %}
`throw` can be used to explicitly throw an error when an exceptional condition
has occurred.

`throw` accepts strings or objects that implement `@display`.

````koto
f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: '{error}'"
# -> Caught an error: '!Error!'
````

{% example_playground_link(version = "0.15") %}
f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: '{error}'"
# -> Caught an error: '!Error!'

{% end %}
### Type checks on `catch` blocks

Type hints can also be used in `try` expressions to implement different
error handling logic depending on the type of error that has been thrown.
A series of `catch` blocks can be added to the `try` expression, each catching
an error that has a particular type.

The final `catch` block needs to *not* have a type check so that it can catch
any errors that were missed by the other blocks.

````koto
f = || throw 'Throwing a String'

try
  f()
catch n: Number
  print 'An error occurred: {n}'
catch error: String
  print error
catch other
  print 'Some other error occurred: {other}'
# -> Throwing a String
````

{% example_playground_link(version = "0.15") %}
f = || throw 'Throwing a String'

try
  f()
catch n: Number
  print 'An error occurred: {n}'
catch error: String
  print error
catch other
  print 'Some other error occurred: {other}'
# -> Throwing a String

{% end %}
## Testing

Koto includes a simple testing framework that help you to check that your code
is behaving as you expect through automated checks.

### Assertions

The core library includes a collection of *assertion* functions in the
[`test` module](../core/test),
which are included by default in the [prelude](#prelude).

````koto
try
  assert 1 + 1 == 3
catch error
  print 'An assertion failed'
# -> An assertion failed

try
  assert_eq 'hello', 'goodbye'
catch error
  print 'An assertion failed'
# -> An assertion failed
````

{% example_playground_link(version = "0.15") %}
try
  assert 1 + 1 == 3
catch error
  print 'An assertion failed'
# -> An assertion failed

try
  assert_eq 'hello', 'goodbye'
catch error
  print 'An assertion failed'
# -> An assertion failed

{% end %}
### Organizing Tests

Tests can be organized by collecting `@test` functions in a map.

The tests can then be run manually with [`test.run_tests`](../core/test#run-tests).

For automatic testing, see the description of exporting `@test` functions in the
[following section](#modules).

````koto
basic_tests =
  @test add: || assert_eq 1 + 1, 2
  @test subtract: || assert_eq 1 - 1, 0

test.run_tests basic_tests
````

{% example_playground_link(version = "0.15") %}
basic_tests =
  @test add: || assert_eq 1 + 1, 2
  @test subtract: || assert_eq 1 - 1, 0

test.run_tests basic_tests

{% end %}
For setup and cleanup operations shared across tests,
`@pre_test` and `@post_test` metakeys can be implemented.
`@pre_test` will be run before each `@test`, and `@post_test` will be run after.

````koto
make_x = |n|
  data: n
  @+: |other| make_x self.data + other.data
  @-: |other| make_x self.data - other.data

x_tests =
  @pre_test: ||
    self.x1 = make_x 100
    self.x2 = make_x 200

  @post_test: ||
    print 'Test complete'

  @test addition: ||
    print 'Testing addition'
    assert_eq self.x1 + self.x2, make_x 300

  @test subtraction: ||
    print 'Testing subtraction'
    assert_eq self.x1 - self.x2, make_x -100

  @test failing_test: ||
    print 'About to fail'
    assert false

try
  test.run_tests x_tests
catch _
  print 'A test failed'
# -> Testing addition
# -> Test complete
# -> Testing subtraction
# -> Test complete
# -> About to fail
# -> A test failed
````

{% example_playground_link(version = "0.15") %}
make_x = |n|
  data: n
  @+: |other| make_x self.data + other.data
  @-: |other| make_x self.data - other.data

x_tests =
  @pre_test: ||
    self.x1 = make_x 100
    self.x2 = make_x 200

  @post_test: ||
    print 'Test complete'

  @test addition: ||
    print 'Testing addition'
    assert_eq self.x1 + self.x2, make_x 300

  @test subtraction: ||
    print 'Testing subtraction'
    assert_eq self.x1 - self.x2, make_x -100

  @test failing_test: ||
    print 'About to fail'
    assert false

try
  test.run_tests x_tests
catch _
  print 'A test failed'
# -> Testing addition
# -> Test complete
# -> Testing subtraction
# -> Test complete
# -> About to fail
# -> A test failed

{% end %}
## Modules

Koto includes a module system that helps you to organize and re-use your code
when your program grows too large for a single file.

### `import`

Values from other modules can be brought into the current scope using `import`.

````koto
from list import last
from number import abs

x = [1, 2, 3]
last x
# -> 3

abs -42
# -> 42
````

{% example_playground_link(version = "0.15") %}
from list import last
from number import abs

x = [1, 2, 3]
print last x
# -> 3

print abs -42
# -> 42

{% end %}
Multiple values from a single module can be imported at the same time.

````koto
from tuple import contains, first, last

x = 'a', 'b', 'c'
first x
# -> a
last x
# -> c
contains x, 'b'
# -> true
````

{% example_playground_link(version = "0.15") %}
from tuple import contains, first, last

x = 'a', 'b', 'c'
print first x
# -> a
print last x
# -> c
print contains x, 'b'
# -> true

{% end %}
Imported values can be renamed using `as` for clarity or to avoid conflicts.

````koto
from list import first as list_first
from tuple import first as tuple_first
list_first [1, 2]
# -> 1
tuple_first (3, 2, 1)
# -> 3
````

{% example_playground_link(version = "0.15") %}
from list import first as list_first
from tuple import first as tuple_first
print list_first [1, 2]
# -> 1
print tuple_first (3, 2, 1)
# -> 3

{% end %}
### `export`

A value can only be imported from a module if the module has *exported* it.

`export` is used to add values to the current module's *exports map*,
making them available to be imported by other modules.

````koto
##################
# my_module.koto #
##################

# hello is a local variable, and isn't exported
hello = 'Hello'

# export say_hello to make it available to other modules
export say_hello = |name| '{hello}, {name}!'

##################
#   other.koto   #
##################

from my_module import say_hello

say_hello 'Koto'
# -> 'Hello, Koto!'
````

{% example_playground_link(version = "0.15") %}
##################
# my_module.koto #
##################

# hello is a local variable, and isn't exported
hello = 'Hello'

# export say_hello to make it available to other modules
export say_hello = |name| '{hello}, {name}!'

##################
#   other.koto   #
##################

from my_module import say_hello

say_hello 'Koto'
# -> 'Hello, Koto!'

{% end %}
To add a [type check](#type-checks) to an exported assignment, use a `let` expression:

````koto
export let foo: Number = -1
````

{% example_playground_link(version = "0.15") %}
export let foo: Number = -1

{% end %}
`export` also supports map syntax, which can be convenient when exporting a lot of values:

````koto
##################
# my_module.koto #
##################

# Define some local values
a, b, c = 1, 2, 3

# Inline maps allow for shorthand syntax
export { a, b, c, foo: 42 }

# Map blocks can also be used with export
export
  bar: 99
  baz: 'baz'
````

{% example_playground_link(version = "0.15") %}
##################
# my_module.koto #
##################

# Define some local values
a, b, c = 1, 2, 3

# Inline maps allow for shorthand syntax
export { a, b, c, foo: 42 }

# Map blocks can also be used with export
export
  bar: 99
  baz: 'baz'

{% end %}
Exported values are available anywhere in the module that exported them.

````koto
get_x = ||
  # x hasn't been created yet. When the function is called, the runtime
  # will check the exports map for a matching value.
  x

export x = 123

get_x()
# -> 123
````

{% example_playground_link(version = "0.15") %}
get_x = ||
  # x hasn't been created yet. When the function is called, the runtime
  # will check the exports map for a matching value.
  x

export x = 123

print get_x()
# -> 123

{% end %}
The exports map can be accessed and modified directly via [`koto.exports`](../core/koto#exports).

````koto
export a, b = 1, 2

# koto.exports() returns the current module's exports map
exports = koto.exports()
# -> {a: 1, b: 2}

# Values can be inserted directly into the exports map
exports.insert 'c', 3
c
# -> 3
````

{% example_playground_link(version = "0.15") %}
export a, b = 1, 2

# koto.exports() returns the current module's exports map
print exports = koto.exports()
# -> {a: 1, b: 2}

# Values can be inserted directly into the exports map
exports.insert 'c', 3
print c
# -> 3

{% end %}
Assigning a new value to a variable that was previously exported won't change
the exported value. If you need to update the exported value, then use `export`
(or update the exports map via [`koto.exports`](../core/koto#exports)).

````koto
export x = 99

# Reassigning a new value to x doesn't affect the previously exported value
x = 123
# -> 123

koto.exports().x
# -> 99
````

{% example_playground_link(version = "0.15") %}
export x = 99

# Reassigning a new value to x doesn't affect the previously exported value
print x = 123
# -> 123

print koto.exports().x
# -> 99

{% end %}
### `@test` functions and `@main`

A module can export `@test` functions, which by default will be automatically run
after the module has been compiled and initialized.

The runtime can be configured to skip running tests, so scripts shouldn't rely on
tests being run.

Additionally, a module can export a `@main` function.
The `@main` function will be called after the module has been compiled and
initialized, and after any exported `@test` functions have been successfully run.

The use of `export` is optional when assigning to metakeys like `@main` and `@test`.

````koto
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

# Equivalent to `export @main = ...`
@main = ||
  print '`my_module` initialized'

@test hello_world = ||
  print 'Testing...'
  assert_eq (say_hello 'World'), 'Hello, World!'

##################
#   other.koto   #
##################

from my_module import say_hello
# -> Testing...
# -> Successfully initialized `my_module`

say_hello 'Koto'
# -> 'Hello, Koto!'
````

{% example_playground_link(version = "0.15") %}
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

# Equivalent to `export @main = ...`
@main = ||
  print '`my_module` initialized'

@test hello_world = ||
  print 'Testing...'
  assert_eq (say_hello 'World'), 'Hello, World!'

##################
#   other.koto   #
##################

from my_module import say_hello
# -> Testing...
# -> Successfully initialized `my_module`

say_hello 'Koto'
# -> 'Hello, Koto!'

{% end %}
### Module Paths

When looking for a module, `import` will look for a `.koto` file with a matching
name, or for a folder with a matching name that contains a `main.koto` file.

E.g. When an `import foo` expression is run, then a `foo.koto` file will be
looked for in the same location as the current script,
and if `foo.koto` isn't found then the runtime will look for `foo/main.koto`.

---