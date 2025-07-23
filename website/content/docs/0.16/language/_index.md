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

Koto programs contain a series of expressions that are evaluated in
top-to-bottom order by Koto's runtime.

As an example, this simple script prints a friendly greeting.

````koto
name = 'World'
print 'Hello, {name}!'
````

{% example_playground_link(version = "0.16") %}
name = 'World'
print 'Hello, {name}!'

{% end %}
### Comments

Single-line comments start with a `#`.

````koto
# This is a comment, everything until the end of the line is ignored.
````

{% example_playground_link(version = "0.16") %}
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

{% example_playground_link(version = "0.16") %}
#-
This is a
multi-line
comment.
-#

{% end %}
### Numbers and Arithmetic

Numbers and arithmetic are expressed in a familiar way.

````koto
1
#: 1

# Addition
1 + 1
#: 2

# Negation and Subtraction
-1 - 10
#: -11

# Multiplication
3 * 4
#: 12

# Division
9 / 2
#: 4.5

# Remainder
12.5 % 5
#: 2.5

# Power / Exponentiation
2 ^ 3
#: 8
````

{% example_playground_link(version = "0.16") %}
print 1
#: 1

# Addition
print 1 + 1
#: 2

# Negation and Subtraction
print -1 - 10
#: -11

# Multiplication
print 3 * 4
#: 12

# Division
print 9 / 2
#: 4.5

# Remainder
print 12.5 % 5
#: 2.5

# Power / Exponentiation
print 2 ^ 3
#: 8

{% end %}
Underscores can be used as separators to aid readability in long numbers.

````koto
1_000_000
#: 1000000
````

{% example_playground_link(version = "0.16") %}
print 1_000_000
#: 1000000

{% end %}
#### Parentheses

Arithmetic operations follow the
[conventional order of precedence][operation-order].
Parentheses can be used to group expressions as needed.

````koto
# Without parentheses, multiplication is performed before addition
1 + 2 * 3 + 4
#: 11
# With parentheses, the additions are performed first
(1 + 2) * (3 + 4)
#: 21
````

{% example_playground_link(version = "0.16") %}
# Without parentheses, multiplication is performed before addition
print 1 + 2 * 3 + 4
#: 11
# With parentheses, the additions are performed first
print (1 + 2) * (3 + 4)
#: 21

{% end %}
#### Non-decimal Numbers

Numbers can be expressed with non-decimal bases.

````koto
# Hexadecimal numbers begin with 0x
0xcafe
#: 51966

# Octal numbers begin with 0o
0o7060
#: 3632

# Binary numbers begin with 0b
0b1001
#: 9
````

{% example_playground_link(version = "0.16") %}
# Hexadecimal numbers begin with 0x
print 0xcafe
#: 51966

# Octal numbers begin with 0o
print 0o7060
#: 3632

# Binary numbers begin with 0b
print 0b1001
#: 9

{% end %}
### Booleans

Booleans are declared with the `true` and `false` keywords, and combined using
the `and` and `or` operators.

````koto
true and false
#: false

true or false
#: true
````

{% example_playground_link(version = "0.16") %}
print true and false
#: false

print true or false
#: true

{% end %}
Booleans can be negated with the `not` operator.

````koto
not true
#: false

not false
#: true
````

{% example_playground_link(version = "0.16") %}
print not true
#: false

print not false
#: true

{% end %}
Values can be compared for equality with the `==` and `!=` operators.

````koto
1 + 1 == 2
#: true

99 != 100
#: true
````

{% example_playground_link(version = "0.16") %}
print 1 + 1 == 2
#: true

print 99 != 100
#: true

{% end %}
### Null

The `null` keyword is used to declare a value of type `Null`,
which indicates the absence of a value.

````koto
null
#: null
````

{% example_playground_link(version = "0.16") %}
print null
#: null

{% end %}
#### Truthiness

In boolean contexts (such as logical operations), `null` is treated as being
equivalent to `false`. Every other value in Koto evaluates as `true`.

````koto
not null
#: true

null or 42
#: 42
````

{% example_playground_link(version = "0.16") %}
print not null
#: true

print null or 42
#: 42

{% end %}
### Assigning Variables

Values are assigned to named identifiers with `=`, and can be freely reassigned.
Named values like this are known as *variables*.

````koto
# Assign the value `42` to `x`
x = 42
x
#: 42

# Replace the existing value of `x`
x = true
x
#: true
````

{% example_playground_link(version = "0.16") %}
# Assign the value `42` to `x`
x = 42
print x
#: 42

# Replace the existing value of `x`
x = true
print x
#: true

{% end %}
The result of an assignment is the value that's being assigned, so chained assignments are possible.

````koto
x = 1
#: 1

a = b = 100
#: 100
a + b
#: 200
````

{% example_playground_link(version = "0.16") %}
print x = 1
#: 1

print a = b = 100
#: 100
print a + b
#: 200

{% end %}
[Compound assignment][compound-assignment] operators are also available.
For example, `x *= y` is a simpler way of writing `x = x * y`.

````koto
a = 100
a += 11
#: 111
a
#: 111

a *= 10
#: 1110
a
#: 1110
````

{% example_playground_link(version = "0.16") %}
a = 100
print a += 11
#: 111
print a
#: 111

print a *= 10
#: 1110
print a
#: 1110

{% end %}
### Debug

The `debug` keyword allows you to quickly display a value while working on a
program.

It prints the result of an expression, prefixed with its line number and the
original expression as a string.

````koto
x = 10 + 20
debug x / 10
#: [2] x / 10: 3.0
````

{% example_playground_link(version = "0.16") %}
x = 10 + 20
debug x / 10
#: [2] x / 10: 3.0

{% end %}
When using `debug`, the displayed value is also the result of the expression,
which can be useful if you want to quickly get feedback during development.

````koto
x = debug 2 + 2
#: [1] 2 + 2: 4
x
#: 4
````

{% example_playground_link(version = "0.16") %}
x = debug 2 + 2
#: [1] 2 + 2: 4
print x
#: 4

{% end %}
### Semicolons

Expressions are typically placed on separate lines,
but if necessary they can be separated with semicolons.

````koto
a = 1; b = 2; c = a + b
c
#: 3
````

{% example_playground_link(version = "0.16") %}
a = 1; b = 2; c = a + b
print c
#: 3

{% end %}
## Lists

Lists in Koto are created with `[]` square brackets and can contain a mix of
different value types.

Access list elements by *index* using square brackets, starting from `0`.

````koto
x = [99, null, true]
x[0]
#: 99
x[1]
#: null

x[2] = false
x[2]
#: false
````

{% example_playground_link(version = "0.16") %}
x = [99, null, true]
print x[0]
#: 99
print x[1]
#: null

x[2] = false
print x[2]
#: false

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
#: [10, 99, 30]
````

{% example_playground_link(version = "0.16") %}
# Assign a list to x
x = [10, 20, 30]

# Assign another instance of the list to y
y = x

# Modify the list through y
y[1] = 99

# The change to y is also reflected in x
print x
#: [10, 99, 30]

{% end %}
If no value is given between commas then `null` is added to the list at that position.

````koto
[10, , 30, , 50]
#: [10, null, 30, null, 50]
````

{% example_playground_link(version = "0.16") %}
print [10, , 30, , 50]
#: [10, null, 30, null, 50]

{% end %}
### Joining Lists

The `+` operator allows lists to be joined together, creating a new list that
contains their concatenated elements.

````koto
a = [98, 99, 100]
b = a + [1, 2, 3]
b
#: [98, 99, 100, 1, 2, 3]
````

{% example_playground_link(version = "0.16") %}
a = [98, 99, 100]
b = a + [1, 2, 3]
print b
#: [98, 99, 100, 1, 2, 3]

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
#: (100, true, -1)
````

{% example_playground_link(version = "0.16") %}
x = 100, true, -1
print x
#: (100, true, -1)

{% end %}
Parentheses can be used for grouping to avoid ambiguity.

````koto
(1, 2, 3), (4, 5, 6)
#: ((1, 2, 3), (4, 5, 6))
````

{% example_playground_link(version = "0.16") %}
print (1, 2, 3), (4, 5, 6)
#: ((1, 2, 3), (4, 5, 6))

{% end %}
You can access tuple elements by index using square brackets, starting from `0`.

````koto
x = false, 10
#: (false, 10)
x[0]
#: false
x[1]
#: 10

y = true, 20
#: (true, 20)
x, y
#: ((false, 10), (true, 20))
````

{% example_playground_link(version = "0.16") %}
print x = false, 10
#: (false, 10)
print x[0]
#: false
print x[1]
#: 10

print y = true, 20
#: (true, 20)
print x, y
#: ((false, 10), (true, 20))

{% end %}
If no value is given between commas then `null` is added to the tuple at that position.

````koto
x = 10, , 20, , 30
x
#: (10, null, 20, null, 30)
````

{% example_playground_link(version = "0.16") %}
x = 10, , 20, , 30
print x
#: (10, null, 20, null, 30)

{% end %}
### Empty and Single Element Tuples

An empty tuple (a tuple that contains zero elements) is created using empty parentheses.

````koto
x = ()
x
#: ()
````

{% example_playground_link(version = "0.16") %}
x = ()
print x
#: ()

{% end %}
A tuple that contains a single element can be created by including a trailing comma.

````koto
# An expression inside parentheses simply resolves to the result of the expression
(1 + 2)
#: 3

# To place the result of the expression in a tuple, use a trailing comma
(1 + 2,)
#: (3)

# Single element tuples can also be created without parentheses
x = 1 + 2,
x
#: (3)
````

{% example_playground_link(version = "0.16") %}
# An expression inside parentheses simply resolves to the result of the expression
print (1 + 2)
#: 3

# To place the result of the expression in a tuple, use a trailing comma
print (1 + 2,)
#: (3)

# Single element tuples can also be created without parentheses
x = 1 + 2,
print x
#: (3)

{% end %}
### Joining Tuples

The `+` operator allows tuples to be joined together,
creating a new tuple containing their concatenated elements.

````koto
a = 1, 2, 3
b = a + (4, 5, 6)
b
#: (1, 2, 3, 4, 5, 6)
````

{% example_playground_link(version = "0.16") %}
a = 1, 2, 3
b = a + (4, 5, 6)
print b
#: (1, 2, 3, 4, 5, 6)

{% end %}
### Tuple Mutability

While tuples have a fixed structure and its contained values can't be
replaced, [*mutable*][immutable] value types (like [lists](#lists)) can be
modified while they're contained in tuples.

````koto
# A Tuple containing two lists
x = ([1, 2, 3], [4, 5, 6])

# Modify the second list in the tuple
x[1][0] = 99
x
#: ([1, 2, 3], [99, 5, 6])
````

{% example_playground_link(version = "0.16") %}
# A Tuple containing two lists
x = ([1, 2, 3], [4, 5, 6])

# Modify the second list in the tuple
x[1][0] = 99
print x
#: ([1, 2, 3], [99, 5, 6])

{% end %}
## Strings

Strings in Koto contain a sequence of [UTF-8][utf-8] encoded characters,
and can be declared using `'` or `"` quotes.

````koto
'Hello, World!'
#: Hello, World!

"Welcome to Koto 👋"
#: Welcome to Koto 👋
````

{% example_playground_link(version = "0.16") %}
print 'Hello, World!'
#: Hello, World!

print "Welcome to Koto 👋"
#: Welcome to Koto 👋

{% end %}
Strings can start on one line and finish on another.

````koto
'This is a string
that spans
several lines.'
#: This is a string
#: that spans
#: several lines.
````

{% example_playground_link(version = "0.16") %}
print 'This is a string
that spans
several lines.'
#: This is a string
#: that spans
#: several lines.

{% end %}
Strings can be joined together with the `+` operator.

````koto
'a' + 'Bc' + 'Def'
#: aBcDef
````

{% example_playground_link(version = "0.16") %}
print 'a' + 'Bc' + 'Def'
#: aBcDef

{% end %}
### String Interpolation

Variables can be easily included in a string by surrounding them with `{}` curly
braces.

````koto
xyz = 123
'The value of xyz is {xyz}'
#: The value of xyz is 123
````

{% example_playground_link(version = "0.16") %}
xyz = 123
print 'The value of xyz is {xyz}'
#: The value of xyz is 123

{% end %}
Including variables in a string this way is known as *string interpolation*.

Simple expressions can also be interpolated using the same syntax.

````koto
'2 plus 3 is {2 + 3}.'
#: 2 plus 3 is 5.
````

{% example_playground_link(version = "0.16") %}
print '2 plus 3 is {2 + 3}.'
#: 2 plus 3 is 5.

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
#: {'"}
'Hi \u{1F44B}'
#: Hi 👋
````

{% example_playground_link(version = "0.16") %}
print '\{\'\"}'
#: {'"}
print 'Hi \u{1F44B}'
#: Hi 👋

{% end %}
### Continuing a Long Line

The end of a line can be escaped with a `\`, which will skip the
newline and any leading whitespace on the next line.

````koto
foo = "This string \
       doesn't contain \
       newlines."
foo
#: This string doesn't contain newlines.
````

{% example_playground_link(version = "0.16") %}
foo = "This string \
       doesn't contain \
       newlines."
print foo
#: This string doesn't contain newlines.

{% end %}
### String Indexing

Individual *bytes* of a string can be accessed via indexing with `[]` braces.

````koto
'abcdef'[3]
#: d
'xyz'[1..]
#: yz
````

{% example_playground_link(version = "0.16") %}
print 'abcdef'[3]
#: d
print 'xyz'[1..]
#: yz

{% end %}
Care must be taken when using indexing with strings that could contain
non-[ASCII][ascii] data.
If the indexed bytes would produce invalid UTF-8 data then an
error will be thrown.
To access a string's Unicode characters individually, see [`string.chars`][chars].

### Single or Double Quotes

Both single `'` and double `"` quotes are valid for defining strings in Koto
and have the same meaning.

A practical reason to choose one over the other is that the alternate
quote type can be used in a string without needing to use escape characters.

````koto
print 'This string has to escape its \'single quotes\'.'
#: This string has to escape its 'single quotes'.

print "This string contains unescaped 'single quotes'."
#: This string contains unescaped 'single quotes'.
````

{% example_playground_link(version = "0.16") %}
print 'This string has to escape its \'single quotes\'.'
#: This string has to escape its 'single quotes'.

print "This string contains unescaped 'single quotes'."
#: This string contains unescaped 'single quotes'.

{% end %}
### Raw Strings

When a string contains a lot of special characters, it can be preferable to use
a *raw string*.

Raw strings ignore escape characters and interpolated expressions,
providing the raw contents of the string between its *delimiters*.

Raw strings use single or double quotes as the delimiter, prefixed with an `r`.

````koto
print r'This string contains special characters: {foo}\n\t.'
#: This string contains special characters: {foo}\n\t.
````

{% example_playground_link(version = "0.16") %}
print r'This string contains special characters: {foo}\n\t.'
#: This string contains special characters: {foo}\n\t.

{% end %}
For more complex string contents, the delimiter can be extended using up to 255
`#` characters after the `r` prefix,

````koto
print r#'This string contains "both" 'quote' types.'#
#: This string contains "both" 'quote' types.

print r##'This string also includes a '#' symbol.'##
#: This string also includes a '#' symbol.
````

{% example_playground_link(version = "0.16") %}
print r#'This string contains "both" 'quote' types.'#
#: This string contains "both" 'quote' types.

print r##'This string also includes a '#' symbol.'##
#: This string also includes a '#' symbol.

{% end %}
## Functions

Functions in Koto are created using a pair of vertical bars (`||`),
with the function's *arguments* listed between the bars.
The *body* of the function follows the vertical bars.

````koto
hi = || 'Hello!'
add = |x, y| x + y
````

{% example_playground_link(version = "0.16") %}
hi = || 'Hello!'
add = |x, y| x + y

{% end %}
Functions are *called* with arguments contained in `()` parentheses.
The body of the function is evaluated and the result is returned to the caller.

````koto
hi = || 'Hello!'
hi()
#: Hello!

add = |x, y| x + y
add(50, 5)
#: 55
````

{% example_playground_link(version = "0.16") %}
hi = || 'Hello!'
print hi()
#: Hello!

add = |x, y| x + y
print add(50, 5)
#: 55

{% end %}
A function's body can be an indented block, where the last
expression in the body is evaluated as the function's result.

````koto
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
f(2, 3, 4)
#: 234
````

{% example_playground_link(version = "0.16") %}
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
print f(2, 3, 4)
#: 234

{% end %}
### Optional Call Parentheses

In simple expressions, the parentheses for function call arguments are optional and can be omitted.

````koto
square = |x| x * x
square 8
#: 64

add = |x, y| x + y
add 2, 3
#: 5

square add 2, 3 # Equivalent to square(add(2, 3))
#: 25

first = |x| x[0]
first ('a', 'b') # Equivalent to first(('a', 'b'))
#: a
````

{% example_playground_link(version = "0.16") %}
square = |x| x * x
print square 8
#: 64

add = |x, y| x + y
print add 2, 3
#: 5

print square add 2, 3 # Equivalent to square(add(2, 3))
#: 25

first = |x| x[0]
print first ('a', 'b') # Equivalent to first(('a', 'b'))
#: a

{% end %}
### Return

When the function should be exited early, the `return` keyword can be used.

````koto
f = |n|
  return 42
  # This expression won't be reached
  n * n
f -1
#: 42
````

{% example_playground_link(version = "0.16") %}
f = |n|
  return 42
  # This expression won't be reached
  n * n
print f -1
#: 42

{% end %}
If a value isn't provided to `return`, then the returned value is `null`.

````koto
f = |n|
  return
  n * n
f 123
#: null
````

{% example_playground_link(version = "0.16") %}
f = |n|
  return
  n * n
print f 123
#: null

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
#: 32

# Parentheses don't help all that much...
x = multiply(2, square(add(1, 3)))
#: 32

# Piping allows for a left-to-right flow of results.
x = add(1, 3) -> square -> multiply 2
#: 32

# Call chains can also be broken across lines.
x = add 1, 3
  -> square
  -> multiply 2
#: 32
````

{% example_playground_link(version = "0.16") %}
add = |x, y| x + y
multiply = |x, y| x * y
square = |x| x * x

# Chained function calls can be a bit hard to follow for the reader.
print x = multiply 2, square add 1, 3
#: 32

# Parentheses don't help all that much...
print x = multiply(2, square(add(1, 3)))
#: 32

# Piping allows for a left-to-right flow of results.
print x = add(1, 3) -> square -> multiply 2
#: 32

# Call chains can also be broken across lines.
print x = add 1, 3
  -> square
  -> multiply 2
#: 32

{% end %}
Piped arguments are inserted as the first argument for the following call.

````koto
get_name = || 'Ada'
say = |name, greeting| print '{greeting}, {name}!'

get_name() -> say 'Hello'
#: Hello, Ada!
````

{% example_playground_link(version = "0.16") %}
get_name = || 'Ada'
say = |name, greeting| print '{greeting}, {name}!'

get_name() -> say 'Hello'
#: Hello, Ada!

{% end %}
## Maps

Maps in Koto are [associative containers][associated] that contain a series of
*entries* with *keys* that correspond to associated values.

Maps can be created using *inline syntax*, with `{}` braces containing a series of entries separated by commas.

The `.` operator returns the value associated with a particular key.

````koto
m = {apples: 42, oranges: 99, lemons: 63}

# Get the value associated with the `oranges` key
m.oranges
#: 99
````

{% example_playground_link(version = "0.16") %}
m = {apples: 42, oranges: 99, lemons: 63}

# Get the value associated with the `oranges` key
print m.oranges
#: 99

{% end %}
Maps can also be created using *block syntax*, with each entry on a new indented line:

````koto
m =
  apples: 42
  oranges: 99
  lemons: 63
m.apples
#: 42
````

{% example_playground_link(version = "0.16") %}
m =
  apples: 42
  oranges: 99
  lemons: 63
print m.apples
#: 42

{% end %}
Once a map has been created, its underlying data is shared between other
instances of the same map. Changes to one instance are reflected in the other.

````koto
# Create a map and assign it to `a`.
a = {foo: 99}
a.foo
#: 99

# Assign a new instance of the map to `z`.
z = a

# Modifying the data via `z` is reflected in `a`.
z.foo = 'Hi!'
a.foo
#: Hi!
````

{% example_playground_link(version = "0.16") %}
# Create a map and assign it to `a`.
a = {foo: 99}
print a.foo
#: 99

# Assign a new instance of the map to `z`.
z = a

# Modifying the data via `z` is reflected in `a`.
z.foo = 'Hi!'
print a.foo
#: Hi!

{% end %}
### Entry Order

A map's entries are maintained in a consistent order,
representing the sequence in which its entries were added.

You can access map entries by index using square brackets, starting from `0`.

The entry is returned as a tuple containing the key and its associated value.

````koto
m = {apples: 42, oranges: 99, lemons: 63}
m[1]
#: ('oranges', 99)
````

{% example_playground_link(version = "0.16") %}
m = {apples: 42, oranges: 99, lemons: 63}
print m[1]
#: ('oranges', 99)

{% end %}
Entries can also be replaced by assigning a key/value tuple to the entry's index.

````koto
m = {apples: 42, oranges: 99, lemons: 63}
m[1] = ('pears', 123)
m
#: {apples: 42, pears: 123, lemons: 63}
````

{% example_playground_link(version = "0.16") %}
m = {apples: 42, oranges: 99, lemons: 63}
m[1] = ('pears', 123)
print m
#: {apples: 42, pears: 123, lemons: 63}

{% end %}
### Shorthand Values

When creating maps with inline syntax, Koto supports a shorthand notation that
simplifies adding existing values to the map.

If a value isn't provided for a key, then Koto will look for a value
that matches the key's name, and if one is found then it will be used as that
entry's value.

````koto
hi, bye = 'hi!', 'bye!'
m = {hi, x: 42, bye}
#: {hi: 'hi!', x: 42, bye: 'bye!'}
````

{% example_playground_link(version = "0.16") %}
hi, bye = 'hi!', 'bye!'
print m = {hi, x: 42, bye}
#: {hi: 'hi!', x: 42, bye: 'bye!'}

{% end %}
### Maps and Self

Maps can store any type of value, including functions,
which provides a convenient way to group functions together.

````koto
m =
  hello: |name| 'Hello, {name}!'
  bye: |name| 'Bye, {name}!'

m.hello 'World'
#: Hello, World!
m.bye 'Friend'
#: Bye, Friend!
````

{% example_playground_link(version = "0.16") %}
m =
  hello: |name| 'Hello, {name}!'
  bye: |name| 'Bye, {name}!'

print m.hello 'World'
#: Hello, World!
print m.bye 'Friend'
#: Bye, Friend!

{% end %}
`self` is a special identifier that refers to the instance of the container in
which the function is contained.

In maps, `self` allows functions to access and modify data from the map,
enabling [*object*][object-wiki]-like behaviour.

````koto
m =
  name: 'World'
  say_hello: || 'Hello, {self.name}!'

m.say_hello()
#: Hello, World!

m.name = 'Friend'
m.say_hello()
#: Hello, Friend!
````

{% example_playground_link(version = "0.16") %}
m =
  name: 'World'
  say_hello: || 'Hello, {self.name}!'

print m.say_hello()
#: Hello, World!

m.name = 'Friend'
print m.say_hello()
#: Hello, Friend!

{% end %}
### Joining Maps

The `+` operator allows maps to be joined together, creating a new map that
combines their entries.

````koto
a = {red: 100, blue: 150}
b = {green: 200, blue: 99}
c = a + b
c
#: {red: 100, blue: 99, green: 200}
````

{% example_playground_link(version = "0.16") %}
a = {red: 100, blue: 150}
b = {green: 200, blue: 99}
c = a + b
print c
#: {red: 100, blue: 99, green: 200}

{% end %}
### Quoted Map Keys

Map keys are usually defined and accessed without quotes, but they are stored in
the map as strings. Quotes can be used if a key needs to be defined that would be
otherwise be disallowed by Koto syntax rules
(e.g. a keyword, or using characters that aren't allowed in an identifier).
Quoted keys also allow key names to be generated dynamically by using string
interpolation.

````koto
x = 99
m =
  'true': 42
  'key{x}': x
m.'true'
#: 42
m.key99
#: 99
````

{% example_playground_link(version = "0.16") %}
x = 99
m =
  'true': 42
  'key{x}': x
print m.'true'
#: 42
print m.key99
#: 99

{% end %}
### Map Key Types

Map keys are typically strings, but any [*immutable*][immutable] value can be
used as a map key by using the [`map.insert`][map-insert] and [`map.get`][map-get]
functions.

The immutable value types in Koto are [strings](#strings), [numbers](#numbers-and-arithmetic),
[booleans](#booleans), [ranges](#ranges), and [`null`](#null).
[Tuples](#tuples) are also considered to be immutable when their contained
elements are all immutable.

````koto
m = {}

m.insert 0, 'zero'
m.get 0
#: zero

m.insert (1, 2, 3), 'xxx'
m.get (1, 2, 3)
#: xxx
````

{% example_playground_link(version = "0.16") %}
m = {}

m.insert 0, 'zero'
print m.get 0
#: zero

m.insert (1, 2, 3), 'xxx'
print m.get (1, 2, 3)
#: xxx

{% end %}
## Core Library

The [*Core Library*][core] provides a collection of fundamental functions
and values for working with the Koto language, organized within *modules*.

````koto
# Convert a string to lowercase
string.to_lowercase 'HELLO'
#: hello

# Get the first element of a list
list.first [99, -1, 3]
#: 99
````

{% example_playground_link(version = "0.16") %}
# Convert a string to lowercase
print string.to_lowercase 'HELLO'
#: hello

# Get the first element of a list
print list.first [99, -1, 3]
#: 99

{% end %}
Koto's built-in value types automatically have access to their corresponding
core library modules via `.` access.

````koto
# Convert a string to uppercase
'xyz'.to_uppercase()
#: XYZ

# Get the last element in a list
['abc', 123].last()
#: 123

# Round a floating-point number to the closest integer
(7 / 2).round()
#: 4

# Check if a map contains an 'apples' key
{apples: 42, pears: 99}.contains_key 'apples'
#: true
````

{% example_playground_link(version = "0.16") %}
# Convert a string to uppercase
print 'xyz'.to_uppercase()
#: XYZ

# Get the last element in a list
print ['abc', 123].last()
#: 123

# Round a floating-point number to the closest integer
print (7 / 2).round()
#: 4

# Check if a map contains an 'apples' key
print {apples: 42, pears: 99}.contains_key 'apples'
#: true

{% end %}
The [documentation][core] for the Core library (along with this guide) is
also available in the `help` command of the [Koto CLI][cli].

### Prelude

Koto's *prelude* is a collection of items that are automatically
made available in a Koto script without the need for first calling [`import`](#import).

The core library's modules are all included by default in the prelude,
along with the following functions:

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
#: io.print is available without needing to be imported
````

{% example_playground_link(version = "0.16") %}
print 'io.print is available without needing to be imported'
#: io.print is available without needing to be imported

{% end %}
## Conditional Expressions

Koto includes several ways of producing values that depend on *conditions*.

### `if`

`if` expressions come in two flavors; single-line:

````koto
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
#: odd
````

{% example_playground_link(version = "0.16") %}
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
#: odd

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
#: ok
````

{% example_playground_link(version = "0.16") %}
x = 24
if x < 0
  print 'negative'
else if x > 24
  print 'no way!'
else
  print 'ok'
#: ok

{% end %}
The result of an `if` expression is the final expression in the branch that gets
executed.

````koto
x = if 1 + 1 == 2 then 3 else -1
x
#: 3

# Assign the result of the if expression to foo
foo = if x > 0
  y = x * 10
  y + 3
else
  y = x * 100
  y * y

foo
#: 33
````

{% example_playground_link(version = "0.16") %}
x = if 1 + 1 == 2 then 3 else -1
print x
#: 3

# Assign the result of the if expression to foo
foo = if x > 0
  y = x * 10
  y + 3
else
  y = x * 100
  y * y

print foo
#: 33

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
#: 13
````

{% example_playground_link(version = "0.16") %}
fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

print fib 7
#: 13

{% end %}
### `match`

`match` expressions can be used to match a value against a series of patterns,
with the matched pattern causing a specific branch of code to be executed.

Patterns can be literals or identifiers. An identifier will match any value,
so they're often used with `if` conditions to refine the match.

````koto
match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: {x}'
  x if x < 50 then 'less than 50: {x}'
  x then 'other: {x}'
#: less than 50: 42
````

{% example_playground_link(version = "0.16") %}
print match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: {x}'
  x if x < 50 then 'less than 50: {x}'
  x then 'other: {x}'
#: less than 50: 42

{% end %}
Ignored values (any identifier starting with `_`) match against any value,
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
#: ('Buzz', 11, 'Fizz', 13, 14, 'Fizz Buzz')
````

{% example_playground_link(version = "0.16") %}
fizz_buzz = |n|
  match n % 3, n % 5
    0, 0 then "Fizz Buzz"
    0, _ then "Fizz"
    _, 0 then "Buzz"
    else n

print (10, 11, 12, 13, 14, 15)
  .each |n| fizz_buzz n
  .to_tuple()
#: ('Buzz', 11, 'Fizz', 13, 14, 'Fizz Buzz')

{% end %}
List and tuple entries can be matched against by using parentheses,
with `...` available for capturing the rest of the sequence.

````koto
match ['a', 'b', 'c'].extend [1, 2, 3]
  ('a', 'b') then
    "A list containing 'a' and 'b'"
  (1, ...) then
    "Starts with '1'"
  (..., 'y', last) then
    "Ends with 'y' followed by '{last}'"
  ('a', x, others...) then
    "Starts with 'a', followed by '{x}', then {size others} others"
  unmatched then "other: {unmatched}"
#: Starts with 'a', followed by 'b', then 4 others
````

{% example_playground_link(version = "0.16") %}
print match ['a', 'b', 'c'].extend [1, 2, 3]
  ('a', 'b') then
    "A list containing 'a' and 'b'"
  (1, ...) then
    "Starts with '1'"
  (..., 'y', last) then
    "Ends with 'y' followed by '{last}'"
  ('a', x, others...) then
    "Starts with 'a', followed by '{x}', then {size others} others"
  unmatched then "other: {unmatched}"
#: Starts with 'a', followed by 'b', then 4 others

{% end %}
### Optional Chaining

Checking optional values for `null` in expression chains can feel a bit
cumbersome, with `if` checks interrupting an expression's natural flow.

The `?` operator can be used to simplify expression chains that contain optional results.
If `?` finds `null` when checking an optional value,
then the chain gets *short-circuited* with `null` given as the chain's overall result.

````koto
info = {town: 'Hamburg', country: 'Germany'}

# `info` contains a value for 'town', which is then passed to to_uppercase():
info.get('town')?.to_uppercase()
#: HAMBURG

# `info` doesn't contain a value for 'state',
# so the `?` operator short-circuits the expression, resulting in `null`:
info.get('state')?.to_uppercase()
#: null

# Without the `?` operator, an intermediate step is necessary:
country = info.get('country')
if country then country.to_uppercase()
#: GERMANY
````

{% example_playground_link(version = "0.16") %}
info = {town: 'Hamburg', country: 'Germany'}

# `info` contains a value for 'town', which is then passed to to_uppercase():
print info.get('town')?.to_uppercase()
#: HAMBURG

# `info` doesn't contain a value for 'state',
# so the `?` operator short-circuits the expression, resulting in `null`:
print info.get('state')?.to_uppercase()
#: null

# Without the `?` operator, an intermediate step is necessary:
country = info.get('country')
print if country then country.to_uppercase()
#: GERMANY

{% end %}
Multiple `?` checks can be performed in an expression chain:

````koto
get_data = || {nested: {maybe_string: null}}
get_data()?
  .get('nested')?
  .get('maybe_string')?
  .to_uppercase()
#: null
````

{% example_playground_link(version = "0.16") %}
get_data = || {nested: {maybe_string: null}}
print get_data()?
  .get('nested')?
  .get('maybe_string')?
  .to_uppercase()
#: null

{% end %}
## Loops

Koto includes several ways of evaluating expressions repeatedly in a loop.

### `for`

`for` loops are repeated for each element in a sequence,
such as a list or tuple.

````koto
for n in [10, 20, 30]
  print n
#: 10
#: 20
#: 30
````

{% example_playground_link(version = "0.16") %}
for n in [10, 20, 30]
  print n
#: 10
#: 20
#: 30

{% end %}
### `while`

`while` loops continue to repeat *while* a condition is true.

````koto
x = 0
while x < 5
  x += 1
x
#: 5
````

{% example_playground_link(version = "0.16") %}
x = 0
while x < 5
  x += 1
print x
#: 5

{% end %}
### `until`

`until` loops continue to repeat *until* a condition is true.

````koto
z = [1, 2, 3]
until z.is_empty()
  # Remove the last element of the list
  print z.pop()
#: 3
#: 2
#: 1
````

{% example_playground_link(version = "0.16") %}
z = [1, 2, 3]
until z.is_empty()
  # Remove the last element of the list
  print z.pop()
#: 3
#: 2
#: 1

{% end %}
### `continue`

The `continue` keyword skips the remaining part of a loop's body and proceeds with the next repetition of the loop.

````koto
for n in (-2, -1, 1, 2)
  # Skip over any values less than 0
  if n < 0
    continue
  print n
#: 1
#: 2
````

{% example_playground_link(version = "0.16") %}
for n in (-2, -1, 1, 2)
  # Skip over any values less than 0
  if n < 0
    continue
  print n
#: 1
#: 2

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
#: 3
````

{% example_playground_link(version = "0.16") %}
x = 0
while x < 100000
  if x >= 3
    # Break out of the loop when x is greater or equal to 3
    break
  x += 1
print x
#: 3

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
#: 103
````

{% example_playground_link(version = "0.16") %}
x = 0
y = while x < 100000
  if x >= 3
    # Break out of the loop, providing x + 100 as the loop's result
    break x + 100
  x += 1
print y
#: 103

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
#: 25
````

{% example_playground_link(version = "0.16") %}
x = 0
y = loop
  x += 1
  # Stop looping when x is greater than 4
  if x > 4
    break x * x
print y
#: 25

{% end %}
## Iterators

The elements of a sequence can be accessed sequentially with an *iterator*,
created using the `.iter()` function.

An iterator yields values via [`.next()`][next] until the end of the sequence is
reached, when `null` is returned.

````koto
i = [10, 20].iter()

i.next()
#: IteratorOutput(10)
i.next()
#: IteratorOutput(20)
i.next()
#: null
````

{% example_playground_link(version = "0.16") %}
i = [10, 20].iter()

print i.next()
#: IteratorOutput(10)
print i.next()
#: IteratorOutput(20)
print i.next()
#: null

{% end %}
### Iterator Generators

The [`iterator`][iterator] module contains iterator *generators* like
[`once`][once] and [`repeat`][repeat] that generate output values
[*lazily*][lazy] during iteration.

````koto
# Create an iterator that repeats ! twice
i = iterator.repeat('!', 2)
i.next()
#: IteratorOutput(!)
i.next()
#: IteratorOutput(!)
i.next()
#: null
````

{% example_playground_link(version = "0.16") %}
# Create an iterator that repeats ! twice
i = iterator.repeat('!', 2)
print i.next()
#: IteratorOutput(!)
print i.next()
#: IteratorOutput(!)
print i.next()
#: null

{% end %}
### Iterator Adaptors

The output of an iterator can be modified using *adaptors* from the
[`iterator`][iterator] module.

The `iterator` module is available to any value which is declared to be *iterable*
(which includes Koto's containers like lists and strings),
so it's not necessary to call `.iter()` before using an adaptor.

````koto
# Create an iterator that outputs any value in the list above 3
x = [1, 2, 3, 4, 5].keep |n| n > 3

x.next()
#: IteratorOutput(4)
x.next()
#: IteratorOutput(5)
x.next()
#: null
````

{% example_playground_link(version = "0.16") %}
# Create an iterator that outputs any value in the list above 3
x = [1, 2, 3, 4, 5].keep |n| n > 3

print x.next()
#: IteratorOutput(4)
print x.next()
#: IteratorOutput(5)
print x.next()
#: null

{% end %}
### Using iterators with `for`

`for` loops accept any iterable value as input, including adapted iterators.

````koto
for x in 'abacad'.keep |c| c != 'a'
  print x
#: b
#: c
#: d
````

{% example_playground_link(version = "0.16") %}
for x in 'abacad'.keep |c| c != 'a'
  print x
#: b
#: c
#: d

{% end %}
### Iterator Chains

Any iterator can be passed into an adaptor, including other adaptors,
creating *iterator chains* that act as data processing pipelines.

````koto
i = (1, 2, 3, 4, 5)
  .skip 1
  .each |n| n * 10
  .keep |n| n <= 40
  .intersperse '--'

for x in i
  print x
#: 20
#: --
#: 30
#: --
#: 40
````

{% example_playground_link(version = "0.16") %}
i = (1, 2, 3, 4, 5)
  .skip 1
  .each |n| n * 10
  .keep |n| n <= 40
  .intersperse '--'

for x in i
  print x
#: 20
#: --
#: 30
#: --
#: 40

{% end %}
### Iterator Consumers

Iterators can also be *consumed* using functions like
[`.to_list()`][to_list] and [`.to_tuple()`][to_tuple],
allowing the output of an iterator to be easily captured in a container.

````koto
[1, 2, 3]
  .each |n| n * 2
  .to_tuple()
#: (2, 4, 6)

(1, 2, 3, 4)
  .keep |n| n % 2 == 0
  .each |n| n * 11
  .to_list()
#: [22, 44]
````

{% example_playground_link(version = "0.16") %}
print [1, 2, 3]
  .each |n| n * 2
  .to_tuple()
#: (2, 4, 6)

print (1, 2, 3, 4)
  .keep |n| n % 2 == 0
  .each |n| n * 11
  .to_list()
#: [22, 44]

{% end %}
Iterator consumers are also available for creating [strings][to_string] and [maps][to_map],
as well as operations like [counting the number of values][iterator-count] yielded from an
iterator, or getting the [total sum][iterator-sum] of an iterator's output.

## Value Unpacking

Multiple assignments can be performed in a single expression by separating the
variable names with commas.

````koto
a, b = 10, 20
a, b
#: (10, 20)
````

{% example_playground_link(version = "0.16") %}
a, b = 10, 20
print a, b
#: (10, 20)

{% end %}
If there's a single value being assigned, and the value is iterable,
then it gets *unpacked* into the target variables.

````koto
my_tuple = 1, 2
x, y = my_tuple
y, x
#: (2, 1)
````

{% example_playground_link(version = "0.16") %}
my_tuple = 1, 2
x, y = my_tuple
print y, x
#: (2, 1)

{% end %}
Unpacking works with any iterable value, including adapted iterators.

````koto
a, b, c = [1, 2, 3, 4, 5]
a, b, c
#: (1, 2, 3)

x, y, z = 'a-b-c'.split '-'
x, y, z
#: ('a', 'b', 'c')
````

{% example_playground_link(version = "0.16") %}
a, b, c = [1, 2, 3, 4, 5]
print a, b, c
#: (1, 2, 3)

x, y, z = 'a-b-c'.split '-'
print x, y, z
#: ('a', 'b', 'c')

{% end %}
If the value being unpacked doesn't contain enough values for the assignment,
then `null` is assigned to any remaining variables.

````koto
a, b, c = [-1, -2]
a, b, c
#: (-1, -2, null)

x, y, z = 42
x, y, z
#: (42, null, null)
````

{% example_playground_link(version = "0.16") %}
a, b, c = [-1, -2]
print a, b, c
#: (-1, -2, null)

x, y, z = 42
print x, y, z
#: (42, null, null)

{% end %}
Unpacking can also be used in `for` loops, which is particularly useful when
looping over the contents of a map.

````koto
my_map = {foo: 42, bar: 99}
for key, value in my_map
  print key, value
#: ('foo', 42)
#: ('bar', 99)
````

{% example_playground_link(version = "0.16") %}
my_map = {foo: 42, bar: 99}
for key, value in my_map
  print key, value
#: ('foo', 42)
#: ('bar', 99)

{% end %}
### Ignoring Unpacked Values

`_` can be used as a placeholder for unpacked values that aren't needed elsewhere
in the code and can therefore be ignored.

If you would like to add a name to the ignored value as a reminder, then the name can be appended to `_`.

Ignored values (any variables starting with `_`) can be written to, but can't be accessed.

````koto
a, _, c = 10..20
a, c
#: (10, 12)

_first, second = 'xyz'
second
#: y
````

{% example_playground_link(version = "0.16") %}
a, _, c = 10..20
print a, c
#: (10, 12)

_first, second = 'xyz'
print second
#: y

{% end %}
## Ranges

Ranges of integers can be created with `..` or `..=`.

`..` creates a *non-inclusive* range,
which defines a range up to but *not including* the end of the range.

````koto
# Create a range from 10 to 20, not including 20
r = 10..20
#: 10..20
r.start()
#: 10
r.end()
#: 20
r.contains 20
#: false
````

{% example_playground_link(version = "0.16") %}
# Create a range from 10 to 20, not including 20
print r = 10..20
#: 10..20
print r.start()
#: 10
print r.end()
#: 20
print r.contains 20
#: false

{% end %}
`..=` creates an *inclusive* range, which includes the end of the range.

````koto
# Create a range from 10 to 20, including 20
r = 10..=20
#: 10..=20
r.contains 20
#: true
````

{% example_playground_link(version = "0.16") %}
# Create a range from 10 to 20, including 20
print r = 10..=20
#: 10..=20
print r.contains 20
#: true

{% end %}
If a value is missing from either side of the range operator then an *unbounded*
range is created.

````koto
# Create an unbounded range starting from 10
r = 10..
r.start()
#: 10
r.end()
#: null

# Create an unbounded range up to and including 100
r = ..=100
r.start()
#: null
r.end()
#: 100
````

{% example_playground_link(version = "0.16") %}
# Create an unbounded range starting from 10
r = 10..
print r.start()
#: 10
print r.end()
#: null

# Create an unbounded range up to and including 100
r = ..=100
print r.start()
#: null
print r.end()
#: 100

{% end %}
Ranges that have a defined start can be indexed using square brackets.

````koto
r = 100..200
r[50]
#: 150

r = 10..
r[100]
#: 110
````

{% example_playground_link(version = "0.16") %}
r = 100..200
print r[50]
#: 150

r = 10..
print r[100]
#: 110

{% end %}
*Bounded* ranges are declared as iterable,
so they can be used in for loops and with the [`iterator`][iterator] module.

````koto
for x in 1..=3
  print x
#: 1
#: 2
#: 3

(0..5).to_list()
#: [0, 1, 2, 3, 4]
````

{% example_playground_link(version = "0.16") %}
for x in 1..=3
  print x
#: 1
#: 2
#: 3

print (0..5).to_list()
#: [0, 1, 2, 3, 4]

{% end %}
### Slices

Ranges can be used to create a *slice* of a container's data.

````koto
x = (10, 20, 30, 40, 50)
x[1..=3]
#: (20, 30, 40)
````

{% example_playground_link(version = "0.16") %}
x = (10, 20, 30, 40, 50)
print x[1..=3]
#: (20, 30, 40)

{% end %}
For immutable containers like tuples and strings,
slices share the original value's data, with no copies being made.

For mutable containers like lists, creating a slice makes a copy of the sliced
portion of the underlying data.

````koto
x = 'abcdef'
# No copies are made when a string is sliced
y = x[3..6]
#: def

a = [1, 2, 3]
# When a list is sliced, the sliced elements get copied into a new list
b = a[0..2]
#: [1, 2]
b[0] = 42
#: 42
a[0]
#: 1
````

{% example_playground_link(version = "0.16") %}
x = 'abcdef'
# No copies are made when a string is sliced
print y = x[3..6]
#: def

a = [1, 2, 3]
# When a list is sliced, the sliced elements get copied into a new list
print b = a[0..2]
#: [1, 2]
print b[0] = 42
#: 42
print a[0]
#: 1

{% end %}
When creating a slice with an unbounded range,
if the start of the range if omitted then the slice starts from the beginning of the container.
If the end of the range is omitted, then the slice includes all remaining elements in the container.

````koto
z = 'Hëllø'.to_tuple()
z[..2]
#: ('H', 'ë')
z[2..]
#: ('l', 'l', 'ø')
````

{% example_playground_link(version = "0.16") %}
z = 'Hëllø'.to_tuple()
print z[..2]
#: ('H', 'ë')
print z[2..]
#: ('l', 'l', 'ø')

{% end %}
## Type Checks

Koto is a primarily a dynamically typed language, however in more complex programs
you might find it beneficial to add *type checks*.

These checks can help in catching errors earlier, and can also act as
documentation for the reader.

One way to add type checks to your program is to use the
[`type`][koto-type] function, which returns a value's type as a string.

````koto
x = 123
assert_eq (type x), 'Number'
````

{% example_playground_link(version = "0.16") %}
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
#: hello

let a: Number, _, c: Bool = 123, x, true
a, c
#: (123, true)
````

{% example_playground_link(version = "0.16") %}
let x: String = 'hello'
print x
#: hello

let a: Number, _, c: Bool = 123, x, true
print a, c
#: (123, true)

{% end %}
### `for` arguments

Type hints can also be added to `for` loop arguments.
The type will be checked on each iteration of the loop.

````koto
for i: Number, s: String in 'abc'.enumerate()
  print i, s
#: (0, 'a')
#: (1, 'b')
#: (2, 'c')
````

{% example_playground_link(version = "0.16") %}
for i: Number, s: String in 'abc'.enumerate()
  print i, s
#: (0, 'a')
#: (1, 'b')
#: (2, 'c')

{% end %}
### Functions

Function arguments can also be given type hints, and the type of the
return value can be checked with the `->` operator.

````koto
f = |s: String| -> Tuple
  s.to_tuple()
f 'abc'
#: ('a', 'b', 'c')
````

{% example_playground_link(version = "0.16") %}
f = |s: String| -> Tuple
  s.to_tuple()
print f 'abc'
#: ('a', 'b', 'c')

{% end %}
### `match` patterns

Type hints can be used in `match` patterns to check the type of the a value.
Rather than throwing an error, if a type check fails then the next
match pattern will be attempted.

````koto
match 'abc'
  x: Tuple then x
  x: String then x.to_tuple()
#: ('a', 'b', 'c')
````

{% example_playground_link(version = "0.16") %}
print match 'abc'
  x: Tuple then x
  x: String then x.to_tuple()
#: ('a', 'b', 'c')

{% end %}
### Optional Values

Sometimes a value can either be of a particular type, or otherwise it should `null`.

These kinds of values are referred to as [*optional*][optional-type],
and are useful for functions or expressions that return either a valid value, or nothing at all.

Optional value types are expressed by appending `?` to the type hint.

````koto
m = {foo: 'hi!'}

let foo: String? = m.get('foo')?.to_uppercase()
#: HI!

let bar: String? = m.get('bar')?.to_uppercase()
#: null
````

{% example_playground_link(version = "0.16") %}
m = {foo: 'hi!'}

print let foo: String? = m.get('foo')?.to_uppercase()
#: HI!

print let bar: String? = m.get('bar')?.to_uppercase()
#: null

{% end %}
### Special Types

#### `Any`

The `Any` type hint will result in a successful check with any value.

````koto
let x: Any = 'hello'
#: hello
````

{% example_playground_link(version = "0.16") %}
print let x: Any = 'hello'
#: hello

{% end %}
#### `Callable`

The `Callable` type hint will accept functions, or any object that can behave
like a function.

````koto
let say_hello: Callable = || 'hello'
say_hello()
#: hello
````

{% example_playground_link(version = "0.16") %}
let say_hello: Callable = || 'hello'
print say_hello()
#: hello

{% end %}
#### `Indexable`

The `Indexable` type hint will accept any value that supports `[]` indexing.

````koto
add_first_two = |x: Indexable| x[0] + x[1]
add_first_two (100, 99, -1)
#: 199
````

{% example_playground_link(version = "0.16") %}
add_first_two = |x: Indexable| x[0] + x[1]
print add_first_two (100, 99, -1)
#: 199

{% end %}
#### `Iterable`

The `Iterable` type hint is useful when any iterable value can be accepted.

````koto
let a: Iterable, b: Iterable = [1, 2], 3..=5
a.chain(b).to_tuple()
#: (1, 2, 3, 4, 5)
````

{% example_playground_link(version = "0.16") %}
let a: Iterable, b: Iterable = [1, 2], 3..=5
print a.chain(b).to_tuple()
#: (1, 2, 3, 4, 5)

{% end %}
## String Formatting

Interpolated string expressions can be formatted using formatting options
similar to [Rust's][rust-format-options].

Inside an interpolated expression, options are provided after a `:` separator.

````koto
'{number.pi:𝜋^8.2}'
#: 𝜋𝜋3.14𝜋𝜋
````

{% example_playground_link(version = "0.16") %}
print '{number.pi:𝜋^8.2}'
#: 𝜋𝜋3.14𝜋𝜋

{% end %}
### Minimum Width and Alignment

A minimum width can be specified, ensuring that the formatted value takes up at
least that many characters.

````koto
foo = "abcd"
'_{foo:8}_'
#: _abcd    _
````

{% example_playground_link(version = "0.16") %}
foo = "abcd"
print '_{foo:8}_'
#: _abcd    _

{% end %}
The minimum width can be prefixed with an alignment modifier:

* `<` - left-aligned
* `^` - centered
* `>` - right-aligned

````koto
foo = "abcd"
'_{foo:^8}_'
#: _  abcd  _
````

{% example_playground_link(version = "0.16") %}
foo = "abcd"
print '_{foo:^8}_'
#: _  abcd  _

{% end %}
All values are left-aligned if an alignment modifier isn't specified,
except for numbers which are right-aligned by default.

````koto
x = 1.2
'_{x:8}_'
#: _     1.2_
````

{% example_playground_link(version = "0.16") %}
x = 1.2
print '_{x:8}_'
#: _     1.2_

{% end %}
The alignment modifier can be prefixed with a character which will be used to
fill any empty space in the formatted string (the default character being ` `).

````koto
x = 1.2
'_{x:~<8}_'
#: _1.2~~~~~_
````

{% example_playground_link(version = "0.16") %}
x = 1.2
print '_{x:~<8}_'
#: _1.2~~~~~_

{% end %}
For numbers, the minimum width can be prefixed with `0`, which will pad the
number to the specified width with zeroes.

````koto
x = 1.2
'{x:06}'
#: 0001.2
````

{% example_playground_link(version = "0.16") %}
x = 1.2
print '{x:06}'
#: 0001.2

{% end %}
### Maximum Width / Precision

A maximum width for the interpolated expression can be specified following a
`.` character.

````koto
foo = "abcd"
'{foo:_^8.2}'
#: ___ab___
````

{% example_playground_link(version = "0.16") %}
foo = "abcd"
print '{foo:_^8.2}'
#: ___ab___

{% end %}
For numbers, the maximum width acts as a 'precision' value, or in other words,
the number of decimal places that will be rendered for the number.

````koto
x = 1 / 3
'{x:.4}'
#: 0.3333
````

{% example_playground_link(version = "0.16") %}
x = 1 / 3
print '{x:.4}'
#: 0.3333

{% end %}
### Representation

Values can be formatted with alternative representations, with representations chosen with a character at the end of the format options.

* `?` - The value will be formatted with additional debug information when available.

The following representations are only supported for numbers:

* `e` - exponential (lower-case)
* `E` - exponential (upper-case)

The following representations are only supported for integers:

* `b` - binary
* `o` - octal
* `x` - hexadecimal (lower-case)
* `X` - hexadecimal (upper-case)

````koto
z = 60
'{z:?}'
#: 60
'{z:x}'
#: 3c
'0x{z:X}'
#: 0x3C
'{z:o}'
#: 74
'0b{z:08b}'
#: 0b00111100
'{z * 1000:e}'
#: 6e4
'{z * 1_000_000:E}'
#: 6E7
````

{% example_playground_link(version = "0.16") %}
z = 60
print '{z:?}'
#: 60
print '{z:x}'
#: 3c
print '0x{z:X}'
#: 0x3C
print '{z:o}'
#: 74
print '0b{z:08b}'
#: 0b00111100
print '{z * 1000:e}'
#: 6e4
print '{z * 1_000_000:E}'
#: 6E7

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
#: 3
````

{% example_playground_link(version = "0.16") %}
x = 1

my_function = |n|
  # x is assigned outside the function,
  # so it gets captured when the function is created.
  n + x

# Reassigning x here doesn't modify the value
# of x that was captured when my_function was created.
x = 100

print my_function 2
#: 3

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
#: (100, 100, 100)
````

{% example_playground_link(version = "0.16") %}
x = 99
f = ||
  # Modifying x only happens with a local copy during a function call.
  # The value of x at the start of the call matches when the value it had when
  # it was captured.
  x += 1

print f(), f(), f()
#: (100, 100, 100)

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
#: (100, 101, 102)
````

{% example_playground_link(version = "0.16") %}
data = {x: 99}

f = ||
  # The data map gets captured by the function,
  # and its contained values can be modified between calls.
  data.x += 1

print f(), f(), f()
#: (100, 101, 102)

{% end %}
### Variadic Functions

A [*variadic function*][variadic] can be created by appending `...` to the last argument.
When the function is called, any extra arguments will be collected into a tuple.

````koto
f = |a, b, others...|
  print "a: {a}, b: {b}, others: {others}"

f 1, 2, 3, 4, 5
#: a: 1, b: 2, others: (3, 4, 5)
f 10, 20
#: a: 10, b: 20, others: ()
````

{% example_playground_link(version = "0.16") %}
f = |a, b, others...|
  print "a: {a}, b: {b}, others: {others}"

f 1, 2, 3, 4, 5
#: a: 1, b: 2, others: (3, 4, 5)
f 10, 20
#: a: 10, b: 20, others: ()

{% end %}
### Optional Arguments

Arguments can be made optional by assigning default values.

````koto
f = |a, b = 2, c = 3|
  print a, b, c

f 1
#: (1, 2, 3)
f 1, -2
#: (1, -2, 3)
f 1, -2, -3
#: (1, -2, -3)
````

{% example_playground_link(version = "0.16") %}
f = |a, b = 2, c = 3|
  print a, b, c

f 1
#: (1, 2, 3)
f 1, -2
#: (1, -2, 3)
f 1, -2, -3
#: (1, -2, -3)

{% end %}
Default argument values behave like [captured variables](#captured-variables),
with the same value being applied each time the function is called.

````koto
f = |x = 10|
  x += 1
  x

f()
#: 11
f()
#: 11
````

{% example_playground_link(version = "0.16") %}
f = |x = 10|
  x += 1
  x

print f()
#: 11
print f()
#: 11

{% end %}
All arguments following an optional argument must also be optional,
unless the last argument is [variadic](#variadic-functions).

````koto
# f = |a = 1, b| a, b
#             ^ Error!

f = |a = 1, b...| a, b
#           ^ Ok!

f()
#: (1, ())
f(1, 2, 3)
#: (1, (2, 3))
````

{% example_playground_link(version = "0.16") %}
# f = |a = 1, b| a, b
#             ^ Error!

f = |a = 1, b...| a, b
#           ^ Ok!

print f()
#: (1, ())
print f(1, 2, 3)
#: (1, (2, 3))

{% end %}
### Unpacking Container Arguments

Functions that expect containers as arguments can *unpack* the container's
elements directly by using parentheses.

````koto
# A function that sums a value that contains three values
f = |(a, b, c)| a + b + c

x = [100, 10, 1]
f x
#: 111
````

{% example_playground_link(version = "0.16") %}
# A function that sums a value that contains three values
f = |(a, b, c)| a + b + c

x = [100, 10, 1]
print f x
#: 111

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
#: 15
````

{% example_playground_link(version = "0.16") %}
# A function that sums elements from nested containers
f = |((a, b), (c, d, e))|
  a + b + c + d + e
x = ([1, 2], [3, 4, 5])
print f x
#: 15

{% end %}
An ellipsis (`...`) can be used to unpack any number of elements at the start or end of a container.

````koto
f = |(..., last)| last * last
x = (1, 2, 3, 4)
f x
#: 16
````

{% example_playground_link(version = "0.16") %}
f = |(..., last)| last * last
x = (1, 2, 3, 4)
print f x
#: 16

{% end %}
A name can be added an ellipsis to capture the unpacked elements in a tuple.

````koto
f = |(first, others...)| first * others.sum()
x = (10, 1, 2, 3)
f x
#: 60
````

{% example_playground_link(version = "0.16") %}
f = |(first, others...)| first * others.sum()
x = (10, 1, 2, 3)
print f x
#: 60

{% end %}
### Ignoring Arguments

As with [assignments](#ignoring-unpacked-values), `_` can be used to ignore function arguments.

````koto
# A function that sums the first and third elements of a container
f = |(a, _, c)| a + c

f [100, 10, 1]
#: 101
````

{% example_playground_link(version = "0.16") %}
# A function that sums the first and third elements of a container
f = |(a, _, c)| a + c

print f [100, 10, 1]
#: 101

{% end %}

````koto
my_map = {foo1: 1, bar1: 2, foo2: 3, bar2: 4}

my_map
  .keep |(key, _value)| key.starts_with 'foo'
  .to_tuple()
#: (('foo1', 1), ('foo2', 3))
````

{% example_playground_link(version = "0.16") %}
my_map = {foo1: 1, bar1: 2, foo2: 3, bar2: 4}

print my_map
  .keep |(key, _value)| key.starts_with 'foo'
  .to_tuple()
#: (('foo1', 1), ('foo2', 3))

{% end %}
### Packed Call Arguments

When calling a function, a *packed argument* is any argument to which `...` is appended.
The runtime will replace the packed argument with the output of iterating over the argument's contents.
Any iterable value can be unpacked.

````koto
f = |a, b, c| a + b + c

x = 10, 20, 30
f x...
#: 60

f (1..10).take(3)...
#: 6
````

{% example_playground_link(version = "0.16") %}
f = |a, b, c| a + b + c

x = 10, 20, 30
print f x...
#: 60

print f (1..10).take(3)...
#: 6

{% end %}
This is especially useful when [variadic arguments](#variadic-functions) need
to be forwarded to another variadic function.

````koto
f = |args...|
  for i, arg in args.enumerate()
    print '{i}: {arg}'

g = |args...| f args...
g 2, 4, 6, 8
#: 0: 2
#: 1: 4
#: 2: 6
#: 3: 8
````

{% example_playground_link(version = "0.16") %}
f = |args...|
  for i, arg in args.enumerate()
    print '{i}: {arg}'

g = |args...| f args...
g 2, 4, 6, 8
#: 0: 2
#: 1: 4
#: 2: 6
#: 3: 8

{% end %}
More than one argument can be unpacked during a call.

````koto
f = |args...|
  for i, arg in args.enumerate()
    print '{i}: {arg}'

x = 10, 20
y = 99, 100
f x..., -1, y...
#: 0: 10
#: 1: 20
#: 2: -1
#: 3: 99
#: 4: 100
````

{% example_playground_link(version = "0.16") %}
f = |args...|
  for i, arg in args.enumerate()
    print '{i}: {arg}'

x = 10, 20
y = 99, 100
f x..., -1, y...
#: 0: 10
#: 1: 20
#: 2: -1
#: 3: 99
#: 4: 100

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
#: IteratorOutput(1)
x.next()
#: IteratorOutput(2)
x.next()
#: null
````

{% example_playground_link(version = "0.16") %}
my_first_generator = ||
  yield 1
  yield 2

x = my_first_generator()
print x.next()
#: IteratorOutput(1)
print x.next()
#: IteratorOutput(2)
print x.next()
#: null

{% end %}
Generator functions can accept arguments like any other function,
and each time they're called a new generator is created.

As with any other iterable value, the [`iterator`][iterator] module's functions
are made available to generators.

````koto
make_generator = |x|
  for y in (1, 2, 3)
    yield x + y

make_generator(0).to_tuple()
#: (1, 2, 3)

make_generator(10)
  # Keep odd numbers, and discard even numbers 
  .keep |n| n % 2 == 1
  .to_list()
#: [11, 13]
````

{% example_playground_link(version = "0.16") %}
make_generator = |x|
  for y in (1, 2, 3)
    yield x + y

print make_generator(0).to_tuple()
#: (1, 2, 3)

print make_generator(10)
  # Keep odd numbers, and discard even numbers 
  .keep |n| n % 2 == 1
  .to_list()
#: [11, 13]

{% end %}
When defining a generator, a `->` [type hint](#type-checks) is used to check
the type of the generator's `yield` expressions.

````koto
g = || -> Number
  yield 1
  yield 2
  yield 3
g().to_tuple()
#: (1, 2, 3)
````

{% example_playground_link(version = "0.16") %}
g = || -> Number
  yield 1
  yield 2
  yield 3
print g().to_tuple()
#: (1, 2, 3)

{% end %}
### Custom Iterator Adaptors

Generators can also serve as *iterator adaptors* by modifying the output of
another iterator.

Inserting a generator into the [`iterator`][iterator] module makes it available
in any iterator chain.

````koto
# Make an iterator adaptor that yields every
# other value from the adapted iterator
iterator.every_other = |iter = null|
  n = 0
  # If the iterator to be adapted is provided as an argument then use it,
  # otherwise defer to `self`, which is set by the runtime when the
  # generator is used in an iterator chain.
  for output in iter or self
    # If n is even, then yield a value
    if n % 2 == 0
      yield output
    n += 1

# The adaptor can be called directly...
iterator.every_other('abcdef').to_string()
#: ace

# ...or anywhere in an iterator chain
(1, 2, 3, 4, 5)
  .each |n| n * 10
  .every_other()
  .to_list()
#: [10, 30, 50]
````

{% example_playground_link(version = "0.16") %}
# Make an iterator adaptor that yields every
# other value from the adapted iterator
iterator.every_other = |iter = null|
  n = 0
  # If the iterator to be adapted is provided as an argument then use it,
  # otherwise defer to `self`, which is set by the runtime when the
  # generator is used in an iterator chain.
  for output in iter or self
    # If n is even, then yield a value
    if n % 2 == 0
      yield output
    n += 1

# The adaptor can be called directly...
print iterator.every_other('abcdef').to_string()
#: ace

# ...or anywhere in an iterator chain
print (1, 2, 3, 4, 5)
  .each |n| n * 10
  .every_other()
  .to_list()
#: [10, 30, 50]

{% end %}
## Objects and Metamaps

Value types with custom behaviour can be defined in Koto through the concept of
*objects*.

An object is any map that includes one or more *metakeys*
(keys prefixed with `@`), that are stored in the object's *metamap*.
Whenever operations are performed on the object, the runtime checks its metamap
for corresponding metakeys.

In the following example, the addition and multiply-assignment operators are
implemented for a custom `Foo` object:

````koto
# Declare a function that makes Foo objects
foo = |n|
  data: n

  # Declare the object's type
  @type: 'Foo'

  # Implement the addition operator
  @+: |other|
    # A new Foo is made using the result
    # of adding the two data values together
    foo self.data + other.data

  # Implement the multiply-assignment operator
  @*=: |other|
    self.data *= other.data
    self

a = foo 10

type a
#: Foo

b = foo 20

(a + b).data
#: 30

a *= b
a.data
#: 200
````

{% example_playground_link(version = "0.16") %}
# Declare a function that makes Foo objects
foo = |n|
  data: n

  # Declare the object's type
  @type: 'Foo'

  # Implement the addition operator
  @+: |other|
    # A new Foo is made using the result
    # of adding the two data values together
    foo self.data + other.data

  # Implement the multiply-assignment operator
  @*=: |other|
    self.data *= other.data
    self

a = foo 10

print type a
#: Foo

b = foo 20

print (a + b).data
#: 30

a *= b
print a.data
#: 200

{% end %}
### Arithmetic Operators

All arithmetic operators used in binary expressions can be implemented in an object's metamap
by implementing functions for the appropriate metakeys.

When the object is on the left-hand side (*LHS*) of the expression the metakeys are
`@+`, `@-`, `@*`, `@/`, `@%`, and `@^`.

If the value on the LHS of the expression doesn't support the operation and the object is on the
right-hand side (*RHS*), then the metakeys are `@r+`, `@r-`, `@r*`, `@r/`, `@r%`, and `@r^`.

If your type only supports an operation when the input has a certain type,
then throw a [`koto.unimplemented`][koto-unimplemented] error to let the runtime know that
the RHS value should be checked. The runtime will catch the error and then attempt the operation
with the implementation provided by the RHS value.

````koto
foo = |n|
  data: n

  @type: 'Foo'

  # The * operator when the object is on the LHS
  @*: |rhs|
    match type rhs
      'Foo' then foo self.data * rhs.data
      'Number' then foo self.data * rhs
      else throw koto.unimplemented

  # The * operator when the object is on the RHS
  @r*: |lhs| foo lhs * self.data

a = foo 2
b = foo 3

(a * b).data
#: 6

(10 * a).data
#: 20
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n

  @type: 'Foo'

  # The * operator when the object is on the LHS
  @*: |rhs|
    match type rhs
      'Foo' then foo self.data * rhs.data
      'Number' then foo self.data * rhs
      else throw koto.unimplemented

  # The * operator when the object is on the RHS
  @r*: |lhs| foo lhs * self.data

a = foo 2
b = foo 3

print (a * b).data
#: 6

print (10 * a).data
#: 20

{% end %}
### Comparison Operators

Comparison operators can also be implemented in an object's metamap
by using the metakeys `@==`, `@!=`, `@<`, `@<=`, `@>`, and `@>=`.

By default, `@!=` will invert the result of calling `@==`,
so it's only necessary to implement it for types with special equality properties.

Types that represent a [total order][total-order] only need to implement `@<` and `@==`,
and the runtime will automatically derive results for `@<=`, `@>`, and `@>=`.

````koto
foo = |n|
  data: n

  @==: |other| self.data == other.data
  @<: |other| self.data < other.data

a = foo 100
b = foo 200

a == a
#: true

# The result of != is derived by inverting the result of @==
a != a
#: false

a < b
#: true

# The result of > is derived from the implementations of @< and @==
a > b
#: false
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n

  @==: |other| self.data == other.data
  @<: |other| self.data < other.data

a = foo 100
b = foo 200

print a == a
#: true

# The result of != is derived by inverting the result of @==
print a != a
#: false

print a < b
#: true

# The result of > is derived from the implementations of @< and @==
print a > b
#: false

{% end %}
### Metakeys

#### `@negate`

The `@negate` metakey overrides the `-` negation operator.

````koto
foo = |n|
  data: n
  @negate: || foo -self.data

x = -foo(100)
x.data
#: -100
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n
  @negate: || foo -self.data

x = -foo(100)
print x.data
#: -100

{% end %}
#### `@size` and `@index`

The `@size` metakey defines how an object should report its size,
while the `@index` metakey defines which values should be returned
when indexing is performed.

If `@size` is implemented, then `@index` should also be implemented.

````koto
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo ('a', 'b', 'c')
size x
#: 3
x[1]
#: b
````

{% example_playground_link(version = "0.16") %}
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo ('a', 'b', 'c')
print size x
#: 3
print x[1]
#: b

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
#: 200

# Inspect the first element in the object
match x
  (first, others...) then 'first: {first}, remaining: {size others}'
#: first: 10, remaining: 4
````

{% example_playground_link(version = "0.16") %}
foo = |data|
  data: data
  @size: || size self.data
  @index: |index| self.data[index]

x = foo (10, 20, 30, 40, 50)

# Unpack the first two elements in the value passed to the function and multiply them
multiply_first_two = |(a, b, ...)| a * b
print multiply_first_two x
#: 200

# Inspect the first element in the object
print match x
  (first, others...) then 'first: {first}, remaining: {size others}'
#: first: 10, remaining: 4

{% end %}
#### `@index_mut`

The `@index_mut` metakey defines how an object should behave when index-assignment is used.

The given value should be a function that takes an index as the first argument,
with the second argument being the value to be assigned.

````koto
foo = |data|
  data: data
  @index: |index| self.data[index]
  @index_mut: |index, value| self.data[index] = value

x = foo ['a', 'b', 'c']
x[1] = 'hello'
x[1]
#: hello
````

{% example_playground_link(version = "0.16") %}
foo = |data|
  data: data
  @index: |index| self.data[index]
  @index_mut: |index, value| self.data[index] = value

x = foo ['a', 'b', 'c']
x[1] = 'hello'
print x[1]
#: hello

{% end %}
#### `@call`

The `@call` metakey defines how an object should behave when its called as a
function.

````koto
foo = |n|
  data: n
  @call: |arg|
    self.data *= arg

x = foo 2
x(10)
#: 20
x(4)
#: 80
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n
  @call: |arg|
    self.data *= arg

x = foo 2
print x(10)
#: 20
print x(4)
#: 80

{% end %}
#### `@iterator`

The `@iterator` metakey defines how iterators should be created when an object
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

foo(0).to_tuple()
#: (1, 2, 3)

foo(100).to_list()
#: [101, 102, 103]
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  # Return a generator that yields the three numbers following n
  @iterator: ||
    yield n + 1
    yield n + 2
    yield n + 3

print foo(0).to_tuple()
#: (1, 2, 3)

print foo(100).to_list()
#: [101, 102, 103]

{% end %}
Note that the `@iterator` metakey will be ignored if the object also implements `@next`,
which implies that the object is *already* an iterator.

#### `@next`

The `@next` metakey allows for objects to treated as iterators.

Whenever the runtime needs to produce an iterator from an object, it will first
check the metamap for an implementation of `@next`, before looking for `@iterator`.

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
#: (10, 11, 12, 13, 14)
````

{% example_playground_link(version = "0.16") %}
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
#: (10, 11, 12, 13, 14)

{% end %}
#### `@next_back`

The `@next_back` metakey is used by [`iterator.reversed`][iterator-reversed]
when producing a reversed iterator.

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
#: (2, 1, 0)
````

{% example_playground_link(version = "0.16") %}
foo =
  n: 0
  @next: || self.n += 1
  @next_back: || self.n -= 1

print foo
  .skip 3 # 0, 1, 2
  .reversed()
  .take 3 # 2, 1, 0
  .to_tuple()
#: (2, 1, 0)

{% end %}
#### `@display`

The `@display` metakey defines how an object should be represented when
displaying the object as a string.

````koto
foo = |n|
  data: n
  @display: || 'Foo({self.data})'

foo 42
#: Foo(42)

x = foo -1
"The value of x is '{x}'"
#: The value of x is 'Foo(-1)'
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n
  @display: || 'Foo({self.data})'

print foo 42
#: Foo(42)

x = foo -1
print "The value of x is '{x}'"
#: The value of x is 'Foo(-1)'

{% end %}
#### `@debug`

The `@debug` metakey defines how an object should be represented when
displaying the object in a debug context, e.g. when using [`debug`](#debug-1),
or when the [`?` representation](#representation) is used in an interpolated expression.

````koto
foo = |n|
  data: n
  @display: || 'Foo({self.data})'
  @debug: || '!!{self}!!'

"{foo(123):?}"
#: !!Foo(123)!!
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n
  @display: || 'Foo({self.data})'
  @debug: || '!!{self}!!'

print "{foo(123):?}"
#: !!Foo(123)!!

{% end %}
If `@debug` isn't defined, then `@display` will be used as a fallback.

#### `@type`

The `@type` metakey takes a string which is used when checking a
value's type, e.g. with [type checks](#type-checks) or [`koto.type`][koto-type].

````koto
foo = |n|
  data: n
  @type: "Foo"

let x: Foo = foo 42
koto.type x
#: Foo
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n
  @type: "Foo"

let x: Foo = foo 42
print koto.type x
#: Foo

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
  @type: 'Animal'
  name: name
  speak: || '{self.noise}! My name is {self.name}!'

dog = |name|
  @base: animal name
  @type: 'Dog'
  noise: 'Woof'

cat = |name|
  @base: animal name
  @type: 'Cat'
  noise: 'Meow'

let fido: Dog = dog 'Fido'
fido.speak()
#: Woof! My name is Fido!

let smudge: Cat = cat 'Smudge'
smudge.speak()
#: Meow! My name is Smudge!

# Type checks will refer to base class @type entries when needed
let an_animal: Animal = if true then fido else smudge
an_animal.name
#: Fido
````

{% example_playground_link(version = "0.16") %}
animal = |name|
  @type: 'Animal'
  name: name
  speak: || '{self.noise}! My name is {self.name}!'

dog = |name|
  @base: animal name
  @type: 'Dog'
  noise: 'Woof'

cat = |name|
  @base: animal name
  @type: 'Cat'
  noise: 'Meow'

let fido: Dog = dog 'Fido'
print fido.speak()
#: Woof! My name is Fido!

let smudge: Cat = cat 'Smudge'
print smudge.speak()
#: Meow! My name is Smudge!

# Type checks will refer to base class @type entries when needed
let an_animal: Animal = if true then fido else smudge
print an_animal.name
#: Fido

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
#: Hello!

print x.get_info()
#: -1 is negative

print map.keys(x).to_tuple()
#: ('data')
````

{% example_playground_link(version = "0.16") %}
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
#: Hello!

print x.get_info()
#: -1 is negative

print map.keys(x).to_tuple()
#: ('data')

{% end %}
### Sharing Metamaps

Metamaps can be shared between objects by using
[`Map.with_meta`][map-with_meta], which helps to avoid inefficient
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
  # Declare the object's type
  @type: 'Foo'

  # Override the + operator
  @+: |other| foo self.data + other.data

  # Define how the object should be displayed
  @display: || "Foo({self.data})"

(foo 10) + (foo 20)
#: Foo(30)
````

{% example_playground_link(version = "0.16") %}
# Create an empty map for global values
global = {}

# Define a function that makes a Foo object
foo = |data|
  # Make a new map that contains `data`,
  # and then attach a shared copy of the metamap from foo_meta.
  {data}.with_meta global.foo_meta

# Define some metakeys in foo_meta
global.foo_meta =
  # Declare the object's type
  @type: 'Foo'

  # Override the + operator
  @+: |other| foo self.data + other.data

  # Define how the object should be displayed
  @display: || "Foo({self.data})"

print (foo 10) + (foo 20)
#: Foo(30)

{% end %}
## Error Handling

Errors can be *thrown* in the Koto runtime, which then cause the runtime to stop
execution.

A *try* / *catch* expression can be used to catch any errors thrown while inside
the `try` block, allowing execution to continue.

An optional `finally` block can be used for cleanup actions that need to
performed whether or not an error was caught.

````koto
x = [1, 2, 3]
try
  # Accessing an invalid index will throw an error
  print x[100]
catch error
  print "Caught an error: '{error}'"
finally
  print "...and finally"
#: Caught an error: 'index out of bounds - index: 100, size: 3'
#: ...and finally
````

{% example_playground_link(version = "0.16") %}
x = [1, 2, 3]
try
  # Accessing an invalid index will throw an error
  print x[100]
catch error
  print "Caught an error: '{error}'"
finally
  print "...and finally"
#: Caught an error: 'index out of bounds - index: 100, size: 3'
#: ...and finally

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
#: Caught an error: '!Error!'
````

{% example_playground_link(version = "0.16") %}
f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: '{error}'"
#: Caught an error: '!Error!'

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
#: Throwing a String
````

{% example_playground_link(version = "0.16") %}
f = || throw 'Throwing a String'

try
  f()
catch n: Number
  print 'An error occurred: {n}'
catch error: String
  print error
catch other
  print 'Some other error occurred: {other}'
#: Throwing a String

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
#: 3

abs -42
#: 42
````

{% example_playground_link(version = "0.16") %}
from list import last
from number import abs

x = [1, 2, 3]
print last x
#: 3

print abs -42
#: 42

{% end %}
Multiple values from a single module can be imported at the same time.

````koto
from tuple import contains, first, last

x = 'a', 'b', 'c'
first x
#: a
last x
#: c
contains x, 'b'
#: true
````

{% example_playground_link(version = "0.16") %}
from tuple import contains, first, last

x = 'a', 'b', 'c'
print first x
#: a
print last x
#: c
print contains x, 'b'
#: true

{% end %}
Imported values can be renamed using `as` for clarity or to avoid conflicts.

````koto
from list import first as list_first
from tuple import first as tuple_first
list_first [1, 2]
#: 1
tuple_first (3, 2, 1)
#: 3
````

{% example_playground_link(version = "0.16") %}
from list import first as list_first
from tuple import first as tuple_first
print list_first [1, 2]
#: 1
print tuple_first (3, 2, 1)
#: 3

{% end %}
You can also use `*` to import all of a module's exported values at once (known as a *wildcard import*) .

````koto
from number import *

abs -1
#: 1
sqrt 25
#: 5.0
````

{% example_playground_link(version = "0.16") %}
from number import *

print abs -1
#: 1
print sqrt 25
#: 5.0

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

# Here, say_hello gets exported, making it available to other modules
export say_hello = |name| '{hello}, {name}!'

##################
#   other.koto   #
##################

from my_module import say_hello

say_hello 'Koto'
#: 'Hello, Koto!'
````

{% example_playground_link(version = "0.16") %}
##################
# my_module.koto #
##################

# hello is a local variable, and isn't exported
hello = 'Hello'

# Here, say_hello gets exported, making it available to other modules
export say_hello = |name| '{hello}, {name}!'

##################
#   other.koto   #
##################

from my_module import say_hello

say_hello 'Koto'
#: 'Hello, Koto!'

{% end %}
To add a [type check](#type-checks) to an exported assignment, use a `let` expression:

````koto
export let foo: Number = -1
````

{% example_playground_link(version = "0.16") %}
export let foo: Number = -1

{% end %}
`export` also accepts maps, or any other iterable value that yields a series of key/value pairs.
This is convenient when exporting a lot of values, or generating exports programatically.

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

# Any iterable value that yields key/value pairs can be used with export
export (1..=3).each |i| 'generated_{i}', i
````

{% example_playground_link(version = "0.16") %}
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

# Any iterable value that yields key/value pairs can be used with export
export (1..=3).each |i| 'generated_{i}', i

{% end %}
Once a value has been exported, it becomes available anywhere in the module.

````koto
get_x = ||
  # x hasn't been created yet. When the function is called, the runtime
  # will check the exports map for a matching value.
  x

export x = 123

get_x()
#: 123

# A function that exports `y` with the given value
export_y = |value|
  export y = value

# y hasn't been exported yet, so attempting to access it now throws an error.
try
  y
catch _
  'y not found'
#: y not found

# Calling export_y adds y to the exports map
export_y 42
y
#: 42
````

{% example_playground_link(version = "0.16") %}
get_x = ||
  # x hasn't been created yet. When the function is called, the runtime
  # will check the exports map for a matching value.
  x

export x = 123

print get_x()
#: 123

# A function that exports `y` with the given value
export_y = |value|
  export y = value

# y hasn't been exported yet, so attempting to access it now throws an error.
print try
  y
catch _
  'y not found'
#: y not found

# Calling export_y adds y to the exports map
export_y 42
print y
#: 42

{% end %}
Assigning a new value locally to a previously exported variable won't change
the exported value. If you need to update the exported value,
then it needs to be re-exported.

````koto
export x = 99

# Reassigning a new value to x locally doesn't affect the previously exported value
x = 123
#: 123

# x has a local value of 123, but the exported value of x is still 99.
export x = -1
# x now has an exported and local value of -1
x
#: -1
````

{% example_playground_link(version = "0.16") %}
export x = 99

# Reassigning a new value to x locally doesn't affect the previously exported value
print x = 123
#: 123

# x has a local value of 123, but the exported value of x is still 99.
export x = -1
# x now has an exported and local value of -1
print x
#: -1

{% end %}
### `@main`

A module can export a `@main` function, which will be called after the module has been compiled and successfully initialized.

The use of `export` is optional when assigning to module metakeys like `@main`.

````koto
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

# Equivalent to `export @main = ...`
@main = || print '`my_module` initialized'

##################
#   other.koto   #
##################

from my_module import say_hello
#: `my_module` initialized

say_hello 'Koto'
#: 'Hello, Koto!'
````

{% example_playground_link(version = "0.16") %}
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

# Equivalent to `export @main = ...`
@main = || print '`my_module` initialized'

##################
#   other.koto   #
##################

from my_module import say_hello
#: `my_module` initialized

say_hello 'Koto'
#: 'Hello, Koto!'

{% end %}
### Module Paths

When looking for a module, `import` will look for a `.koto` file with a matching
name, or for a folder with a matching name that contains a `main.koto` file.

For example, when the expression `import foo` is evaluated,
then the runtime will look for a `foo.koto` file in the same location as the current script,
and if one isn't found then the runtime will look for `foo/main.koto`.

## Testing

Koto includes a simple testing framework that allows you to automatically check that your code is behaving as you would expect.

### Assertions

The core library includes a collection of *assertion* functions which
throw errors if a given condition isn't met.

The assertion functions are found in the [`test` module](../core/test),
and are included by default in the [prelude](#prelude).

````koto
try
  assert 1 + 1 == 2
  print 'The assertion passed'
catch error
  print 'The assertion failed'
#: The assertion passed

try
  assert_eq 'hello', 'goodbye'
  print 'The assertion passed'
catch error
  print 'The assertion failed'
#: The assertion failed
````

{% example_playground_link(version = "0.16") %}
try
  assert 1 + 1 == 2
  print 'The assertion passed'
catch error
  print 'The assertion failed'
#: The assertion passed

try
  assert_eq 'hello', 'goodbye'
  print 'The assertion passed'
catch error
  print 'The assertion failed'
#: The assertion failed

{% end %}
### Module Tests

Tests can be added to a module by exporting `@test` functions. A test function is considered to have failed if it throws an error (e.g. from an assertion).

If Koto is configured to run tests, then the tests will be run after a module has been successfully initialized.

After all tests have run successully, then the runtime will call the module's `@main` function if it's defined.

The CLI doesn't enable tests by default when running scripts, but they can be enabled [via a flag][cli-tests].

````koto
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

@main = || print '`my_module` initialized'

@test say_hello = ||
  print 'Running @test say_hello'
  assert_eq say_hello('Test'), 'Hello, Test!'

##################
#   other.koto   #
##################

from my_module import say_hello
#: Running @test say_hello
#: `my_module` initialized
````

{% example_playground_link(version = "0.16") %}
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

@main = || print '`my_module` initialized'

@test say_hello = ||
  print 'Running @test say_hello'
  assert_eq say_hello('Test'), 'Hello, Test!'

##################
#   other.koto   #
##################

from my_module import say_hello
#: Running @test say_hello
#: `my_module` initialized

{% end %}
`@pre_test` and `@post_test` functions can be implemented alongside tests
for setup and cleanup operations.
`@pre_test` will be run before each `@test`, and `@post_test` will be run after.

````koto
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

@main = || print '`my_module` initialized'

@pre_test = ||
  print 'In @pre_test'

@post_test = ||
  print 'In @post_test'

@test say_hello_1 = ||
  print 'Running @test say_hello_1'
  assert_eq say_hello('One'), 'Hello, One!'

@test say_hello_2 = ||
  print 'Running @test say_hello_2'
  assert_eq say_hello('Two'), 'Hello, Two!'

##################
#   other.koto   #
##################

from my_module import say_hello
#: In @pre_test
#: Running @test say_hello_1
#: In @post_test
#: In @pre_test
#: Running @test say_hello_2
#: In @post_test
#: `my_module` initialized
````

{% example_playground_link(version = "0.16") %}
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, {name}!'

@main = || print '`my_module` initialized'

@pre_test = ||
  print 'In @pre_test'

@post_test = ||
  print 'In @post_test'

@test say_hello_1 = ||
  print 'Running @test say_hello_1'
  assert_eq say_hello('One'), 'Hello, One!'

@test say_hello_2 = ||
  print 'Running @test say_hello_2'
  assert_eq say_hello('Two'), 'Hello, Two!'

##################
#   other.koto   #
##################

from my_module import say_hello
#: In @pre_test
#: Running @test say_hello_1
#: In @post_test
#: In @pre_test
#: Running @test say_hello_2
#: In @post_test
#: `my_module` initialized

{% end %}
### Running Tests Manually

Tests can be run manually by calling [`test.run_tests`][test-run_tests]
with a map that contains `@test` functions.

````koto
my_tests =
  @test add: || assert_eq 1 + 1, 2
  @test subtract: || assert_eq 1 - 1, 0

test.run_tests my_tests
````

{% example_playground_link(version = "0.16") %}
my_tests =
  @test add: || assert_eq 1 + 1, 2
  @test subtract: || assert_eq 1 - 1, 0

test.run_tests my_tests

{% end %}
---

You've made it to the end of the guide! If you spotted any mistakes, or noticed any sections that were less clear than you would have liked,
then please open an [issue][issues] or create a [PR][prs].

For further reading, take a look at docs for the [core library][core], the [extra libs][extra-libs], or how Koto can be integrated into Rust applications in the [Rust API docs][rust-api].

[operation-order]: https://en.wikipedia.org/wiki/Order_of_operations#Conventional_order
[compound-assignment]: https://en.wikipedia.org/wiki/Augmented_assignment
[immutable]: https://en.wikipedia.org/wiki/Immutable_object
[utf-8]: https://en.wikipedia.org/wiki/UTF-8
[ascii]: https://en.wikipedia.org/wiki/ASCII
[chars]: ../core/string#chars
[associated]: https://en.wikipedia.org/wiki/Associative_array
[object-wiki]: https://en.wikipedia.org/wiki/Object_(computer_science)
[map-insert]: ../core/map#insert
[map-get]: ../core/map#get
[core]: ../core
[cli]: ../cli
[next]: ../core/iterator#next
[iterator]: ../core/iterator
[once]: ../core/iterator#once
[repeat]: ../core/iterator#repeat
[lazy]: https://en.wikipedia.org/wiki/Lazy_evaluation
[to_list]: ../core/iterator#to-list
[to_tuple]: ../core/iterator#to-tuple
[to_string]: ../core/iterator#to-string
[to_map]: ../core/iterator#to-map
[iterator-count]: ../core/iterator#count
[iterator-sum]: ../core/iterator#sum
[koto-type]: ../core/koto#type
[optional-type]: https://en.wikipedia.org/wiki/Option_type
[rust-format-options]: https://doc.rust-lang.org/std/fmt/#formatting-parameters
[variadic]: https://en.wikipedia.org/wiki/Variadic_function
[koto-unimplemented]: ../core/koto#unimplemented
[total-order]: https://en.wikipedia.org/wiki/Total_order
[iterator-reversed]: ../core/iterator#reversed
[map-with_meta]: ../core/map#with-meta
[cli-tests]: ../cli#running-tests
[test-run_tests]: ../core/test#run-tests
[issues]: https://github.com/koto-lang/koto/issues
[prs]: https://github.com/koto-lang/koto/pulls
[extra-libs]: ../libs
[rust-api]: ../api