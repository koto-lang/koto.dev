+++
title = "Language Guide"
template = "docs-guide.html"
insert_anchor_links = "heading"
+++

# The Koto Language Guide

This guide contains a tour of the Koto language, giving an overview of its features.

You're encouraged to play around with the examples to get a feel for the language.
The small icon below each example will load the example into the
[Koto Playground](https://koto.dev/play) where you can run the code and see what
happens when you make changes.

The guide can be read from start to finish, with later sections building on
concepts introduced in earlier sections.


## Getting Started

### Installing Koto

Installing the Koto command-line interface (CLI) currently requires the 
[Rust](https://rust-lang.org) toolchain, 
see [rustup.sh](https://rustup.sh) for installation instructions.

With Rust available on your system, the `koto` command can be installed with
`cargo install koto_cli`.

### REPL

Running `koto` without arguments will start the Koto 
[REPL](https://en.wikipedia.org/wiki/Read–eval–print_loop), where Koto
expressions can be entered and evaluated interactively. 

````lua
> koto
Welcome to Koto v0.11.0
» 1 + 1
➝ 2

» 'hello!'
➝ hello!
````

This guide, along with the [core library reference](../../core), 
can be read in the REPL using the `help` command. 

````lua
> koto
Welcome to Koto v0.11.0
» help bool

  Booleans
  ========

  Booleans are declared with the `true` and `false` keywords, 
  and combined using the `and` and `or` operators.

  |
  |  true and false
  |  # ➝ false
  ...
````

## Language Basics

### Koto Programs

Koto programs contain a series of expressions that are evaluated by Koto's runtime.

For example, this program asks for the user's name and then offers them a
friendly greeting.

````koto
print 'Please enter your name:'
name = io.stdin().read_line()
print "Hi there, $name!"
````

{% example_playground_link() %}
print 'Please enter your name:'
name = io.stdin().read_line()
print "Hi there, $name!"

{% end %}
Try placing the above example in a file named `hello.koto`, and then running 
`koto hello.koto`.

### Comments

Single-line comments start with a `#`. 

````koto
# This is a comment, everything until the end of the line is ignored.
````

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
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
Parentheses can be used to group expressions.

````koto
(1 + 2) * (3 + 4)
# -> 21
````

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
print 1 + 1 == 2
# -> true

print 99 != 100
# -> true

{% end %}
### Null

The `null` keyword is used to declare a value of type `Null`,
which represents the absence of a value.

````koto
null
# -> null
````

{% example_playground_link() %}
print null
# -> null

{% end %}
#### Truthiness

When `null` is encountered in a boolean context, it evaluates as `false`.

Every value except for `false` and `null` evaluates as `true`.

````koto
not null
# -> true

null or 42
# -> 42
````

{% example_playground_link() %}
print not null
# -> true

print null or 42
# -> 42

{% end %}
### Assigning Values

Values are assigned with `=`, and can be freely reassigned.

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

{% example_playground_link() %}
# Assign the value `42` to `x`
x = 42
print x
# -> 42

# Replace the existing value of `x` 
x = true
print x
# -> true

{% end %}
Arithmetic assignment operators are available, e.g. `x *= y` is shorthand for 
`x = x * y`.

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

{% example_playground_link() %}
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

The `debug` expression allows you to quickly display a value while working on a program, along with the expression as a string and its line number.

````koto
x = 10 + 20
debug x / 10
# -> [2] x / 10: 3.0
````

{% example_playground_link() %}
x = 10 + 20
debug x / 10
# -> [2] x / 10: 3.0

{% end %}
The result of a `debug` expression is the value that gets displayed, which can
be useful when you want to quickly inspect the result of an expression.

````koto
x = debug 2 + 2
# -> [1] 2 + 2: 4
x
# -> 4
````

{% example_playground_link() %}
x = debug 2 + 2
# -> [1] 2 + 2: 4
print x
# -> 4

{% end %}


## Strings

Strings can be declared using `'` or `"` quotes. 

````koto
'Hello, World!'
# -> Hello, World!

"Welcome to Koto 👋"
# -> Welcome to Koto 👋
````

{% example_playground_link() %}
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

{% example_playground_link() %}
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
print 'abcdef'[3]
# -> d
print '👋🥳😆'[1]
# -> 🥳

{% end %}
### String Interpolation

Assigned values can be included in a String by prefixing them with `$`.

````koto
xyz = 123
'The value of xyz is $xyz'
# -> The value of xyz is 123
````

{% example_playground_link() %}
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
print '2 plus 3 is ${2 + 3}.'
# -> 2 plus 3 is 5.

{% end %}
### String Escape codes

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
print '\$\'\"'
# -> $'"
print 'Hi \u{1F44B}'
# -> Hi 👋

{% end %}
### Single or double quotes

Whether you use `'` or `"` for your strings doesn't make a difference, except that you can use the other quote character freely in the string without having to escape it with `\`.

````koto
print "This string contains 'single quotes'."
# -> This string contains 'single quotes'.

print 'This string has to escape its \'single quotes\'.'
# -> This string has to escape its 'single quotes'.
````

{% example_playground_link() %}
print "This string contains 'single quotes'."
# -> This string contains 'single quotes'.

print 'This string has to escape its \'single quotes\'.'
# -> This string has to escape its 'single quotes'.

{% end %}


## Functions

Functions are values, and are created using a pair of `|` vertical bars, with the function arguments listed between the start and end `|`. 

The *body* of the function follows, with the result of the body used as the function's result.

````koto
hi = || 'Hello!'
add = |x, y| x + y
````

{% example_playground_link() %}
hi = || 'Hello!'
add = |x, y| x + y

{% end %}
Functions are called with arguments contained in `()` parentheses.

````koto
hi = || 'Hello!'
hi()
# -> Hello!

add = |x, y| x + y
add(50, 5)
# -> 55
````

{% example_playground_link() %}
hi = || 'Hello!'
print hi()
# -> Hello!

add = |x, y| x + y
print add(50, 5)
# -> 55

{% end %}
When calling a function with arguments, the parentheses are optional.

````koto
square = |x| x * x
square 8
# -> 64

pow = |x, y| x.pow y
pow 2, 3
# -> 8
````

{% example_playground_link() %}
square = |x| x * x
print square 8
# -> 64

pow = |x, y| x.pow y
print pow 2, 3
# -> 8

{% end %}
### Return

A function's body can be an indented block, with the final expression in the body used as the function's result.

````koto
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
f 2, 3, 4
# -> 234
````

{% example_playground_link() %}
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
print f 2, 3, 4
# -> 234

{% end %}
The `return` keyword can be used to exit the function early with a result.

````koto
f = |n|
  return 42
  # This expression won't be reached
  n * n
f -1
# -> 42
f 10
# -> 42
````

{% example_playground_link() %}
f = |n|
  return 42
  # This expression won't be reached
  n * n
print f -1
# -> 42
print f 10
# -> 42

{% end %}
### Function Piping

When passing the result of a function into another function, it can become a bit
hard to read, especially when a chain of functions is involved.

Using parentheses can help to disambiguate the expression for the reader, but an
alternative is available in *function piping*, where the `>>` operator can be
used to pass the result of one function to another, working from left to right.

````koto
add = |x, y| x + y
multiply = |x, y| x * y
square = |x| x * x

# Chained function calls can be a bit hard to follow
x = multiply 2, square add 1, 3
x
# -> 32

# Parentheses don't help all that much...
x = multiply(2, square(add(1, 3)))
x
# -> 32

# Piping allows for a left-to-right flow of results
x = add(1, 3) >> square >> multiply 2
x
# -> 32

# Call chains can also be broken across lines 
x = 
  add 1, 3
  >> square 
  >> multiply 2
x
# -> 32
````

{% example_playground_link() %}
add = |x, y| x + y
multiply = |x, y| x * y
square = |x| x * x

# Chained function calls can be a bit hard to follow
x = multiply 2, square add 1, 3
print x
# -> 32

# Parentheses don't help all that much...
x = multiply(2, square(add(1, 3)))
print x
# -> 32

# Piping allows for a left-to-right flow of results
x = add(1, 3) >> square >> multiply 2
print x
# -> 32

# Call chains can also be broken across lines 
x = 
  add 1, 3
  >> square 
  >> multiply 2
print x
# -> 32

{% end %}


## Lists

Lists are declared with `[]` braces and can contain any values.

Entries in a List can be accessed by index (starting from `0`) by using `[]`
braces.

````koto
x = ['a', 99, true]
x[0]
# -> a
x[1]
# -> 99

x[2] = false
x[2]
# -> false

y = [['a', 'b', 'c'], ['x', 'y', 'z']]
y[0][1] 
# -> b
y[1][2] 
# -> z
````

{% example_playground_link() %}
x = ['a', 99, true]
print x[0]
# -> a
print x[1]
# -> 99

x[2] = false
print x[2]
# -> false

y = [['a', 'b', 'c'], ['x', 'y', 'z']]
print y[0][1] 
# -> b
print y[1][2] 
# -> z

{% end %}
Once a List has been created, its data is shared between instances of the List.

````koto
x = [10, 20, 30]
y = x
y[1] = 99
x # x and y share the same data
# -> [10, 99, 30]
````

{% example_playground_link() %}
x = [10, 20, 30]
y = x
y[1] = 99
print x # x and y share the same data
# -> [10, 99, 30]

{% end %}


## Tuples

Tuples are like Lists, except they have a fixed size and their entries can't be replaced. 

Tuples are declared with `()` parentheses containing expressions separated by commas. 

````koto
x = (-1, 'abc', true)
x[1]
# -> abc
````

{% example_playground_link() %}
x = (-1, 'abc', true)
print x[1]
# -> abc

{% end %}
In simple expressions the `()` parentheses are optional.

````koto
1, 2, 3
# -> (1, 2, 3)

x = 'a', 10
# -> ('a', 10)
y = 'b', 20
# -> ('b', 20)
x, y
# -> (('a', 10), ('b', 20))
````

{% example_playground_link() %}
print 1, 2, 3
# -> (1, 2, 3)

print x = 'a', 10
# -> ('a', 10)
print y = 'b', 20
# -> ('b', 20)
print x, y
# -> (('a', 10), ('b', 20))

{% end %}
To create an empty Tuple, or a Tuple with a single entry, use a trailing `,` inside the parentheses.

````koto
# An empty pair of parentheses resolves to Null
() 
# -> null

# A single value in parentheses simply provides the value
(1) 
# -> 1

# A comma inside parentheses creates a Tuple 
(,) 
# -> ()
(1,)
# -> (1)
````

{% example_playground_link() %}
# An empty pair of parentheses resolves to Null
print () 
# -> null

# A single value in parentheses simply provides the value
print (1) 
# -> 1

# A comma inside parentheses creates a Tuple 
print (,) 
# -> ()
print (1,)
# -> (1)

{% end %}
Although Tuples have a fixed structure, mutable values in a Tuple (e.g. Lists and Maps) can still be modified.

````koto
# A Tuple containing two Lists
x = ([1, 2, 3], [4, 5, 6])

# Modify the second List in the Tuple
x[1][0] = 99
x
# -> ([1, 2, 3], [99, 5, 6])
````

{% example_playground_link() %}
# A Tuple containing two Lists
x = ([1, 2, 3], [4, 5, 6])

# Modify the second List in the Tuple
x[1][0] = 99
print x
# -> ([1, 2, 3], [99, 5, 6])

{% end %}


## Maps

Maps are Koto's associative containers, containing a series of key/value entries.

They can be declared with `{}` braces (known as *inline syntax*), or by using indented blocks (known as *block syntax*).

With braces:

````koto
m = {apples: 42, oranges: 99, lemons: 63}
m.oranges
# -> 99
````

{% example_playground_link() %}
m = {apples: 42, oranges: 99, lemons: 63}
print m.oranges
# -> 99

{% end %}
...and as an indented block:

````koto
m = 
  apples: 42
  oranges: 99
  lemons: 63
m.apples
# -> 42
````

{% example_playground_link() %}
m = 
  apples: 42
  oranges: 99
  lemons: 63
print m.apples
# -> 42

{% end %}
Nested maps can be declared with additional indentation:

````koto
m =
  hello:
    world: 99
    everybody: 123
    to:
      you: -1
m.hello.world
# -> 99
m.hello.to.you
# -> -1
````

{% example_playground_link() %}
m =
  hello:
    world: 99
    everybody: 123
    to:
      you: -1
print m.hello.world
# -> 99
print m.hello.to.you
# -> -1

{% end %}
### Shorthand Values

When using inline syntax, if there's a value available that matches a key's name, then declaring the value is optional.

When using inline syntax, declaring a value for a key is optional. The runtime will look for a value that matches the key's name, and then copy it into the map.

````koto
bar = 'hi!'
m = {foo: 42, bar, baz: -1}
m.bar
# -> hi!
````

{% example_playground_link() %}
bar = 'hi!'
m = {foo: 42, bar, baz: -1}
print m.bar
# -> hi!

{% end %}
### Data Sharing

Once a map has been created, any additional instances of the map share the same data.

````koto
a = {foo: 99, bar: -1}
a.foo
# -> 99
z = a
z.foo = 'Hi!'
a.foo
# -> Hi!
````

{% example_playground_link() %}
a = {foo: 99, bar: -1}
print a.foo
# -> 99
z = a
z.foo = 'Hi!'
print a.foo
# -> Hi!

{% end %}
### Maps and Functions

Any value type can be stored in Maps, including Functions.

````koto
m = 
  hello: |name| 'Hello, $name!'
  bye: |name| 'Bye, $name!'

m.hello 'World'
# -> Hello, World!
m.bye 'Friend'
# -> Bye, Friend!
````

{% example_playground_link() %}
m = 
  hello: |name| 'Hello, $name!'
  bye: |name| 'Bye, $name!'

print m.hello 'World'
# -> Hello, World!
print m.bye 'Friend'
# -> Bye, Friend!

{% end %}
`self` is a special identifier that refers to the instance of the map that the function is contained in. 

````koto
m = 
  name: 'World'
  hello: || 'Hello, ${self.name}!'

m.hello()
# -> Hello, World!

m.name = 'Friend'
m.hello()
# -> Hello, Friend!
````

{% example_playground_link() %}
m = 
  name: 'World'
  hello: || 'Hello, ${self.name}!'

print m.hello()
# -> Hello, World!

m.name = 'Friend'
print m.hello()
# -> Hello, Friend!

{% end %}


## Core Library

Koto includes a [Core Library](../../core) of useful functions and values organized into `Map`s known as *modules*. 

````koto
string.size 'hello'
# -> 5

list.first [99, -1, 3]
# -> 99
````

{% example_playground_link() %}
print string.size 'hello'
# -> 5

print list.first [99, -1, 3]
# -> 99

{% end %}
Values in Koto automatically have their corresponding core library modules available via `.` access.

````koto
'xyz'.size()
# -> 3

['abc', 123].first()
# -> abc

(11 / 2).round()
# -> 6

{apples: 42, pears: 99}.contains_key 'apples'
# -> true
````

{% example_playground_link() %}
print 'xyz'.size()
# -> 3

print ['abc', 123].first()
# -> abc

print (11 / 2).round()
# -> 6

print {apples: 42, pears: 99}.contains_key 'apples'
# -> true

{% end %}


## Iterators

The entries of a container can be accessed in order via an Iterator,
created with the `.iter()` function.

The iterator yields values via `.next()`, until the end of the sequence is
reached and `null` is returned.

````koto
i = [10, 20].iter()
i.next()
# -> 10
i.next()
# -> 20
i.next()
# -> null
````

{% example_playground_link() %}
i = [10, 20].iter()
print i.next()
# -> 10
print i.next()
# -> 20
print i.next()
# -> null

{% end %}
Iterators can be *adapted* using adaptors from the
[`iterator` module](../../core/iterator).
Iterator adaptors will accept any iterable value (which includes all containers),
so it's not necessary to call `.iter()` first.

````koto
x = [1, 2, 3, 4, 5].keep |n| n > 3
x.next()
# -> 4
x.next()
# -> 5
x.next()
# -> null
````

{% example_playground_link() %}
x = [1, 2, 3, 4, 5].keep |n| n > 3
print x.next()
# -> 4
print x.next()
# -> 5
print x.next()
# -> null

{% end %}
Iterators can be also be *consumed* using functions like
`.to_list()` and `.to_tuple()`.

````koto
[1, 2, 3]
  .each |n| n * 2
  .to_tuple()
# -> (2, 4, 6)

(11, 22, 33, 44)
  .keep |n| n % 2 == 0
  .to_list()
# -> [22, 44]
````

{% example_playground_link() %}
print [1, 2, 3]
  .each |n| n * 2
  .to_tuple()
# -> (2, 4, 6)

print (11, 22, 33, 44)
  .keep |n| n % 2 == 0
  .to_list()
# -> [22, 44]

{% end %}


## Value Unpacking

Multiple values can be assigned at once by separating the names with commas.

````koto
a, b = 10, 20
a, b
# -> (10, 20)
````

{% example_playground_link() %}
a, b = 10, 20
print a, b
# -> (10, 20)

{% end %}
If there's a single value on the right-hand side of the assignment, 
then it gets *unpacked* into the assignment targets.

````koto
my_tuple = 1, 2
x, y = my_tuple
y, x
# -> (2, 1)
````

{% example_playground_link() %}
my_tuple = 1, 2
x, y = my_tuple
print y, x
# -> (2, 1)

{% end %}
If there aren't enough values to unpack, then `null` is assigned to the extra
assignment targets.

````koto
a, b, c = [-1, -2]
a, b, c
# -> (-1, -2, null)
````

{% example_playground_link() %}
a, b, c = [-1, -2]
print a, b, c
# -> (-1, -2, null)

{% end %}
Unpacking works with any iterable value, including adapted iterators.

````koto
a, b, c = 1..10
a, b, c
# -> (1, 2, 3)

a, b, c = (1..10).each |x| x * 10
a, b, c
# -> (10, 20, 30)
````

{% example_playground_link() %}
a, b, c = 1..10
print a, b, c
# -> (1, 2, 3)

a, b, c = (1..10).each |x| x * 10
print a, b, c
# -> (10, 20, 30)

{% end %}


## Conditional Expressions

### if

`if` expressions come in two flavours; single-line:

````koto
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
# -> odd
````

{% example_playground_link() %}
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
# -> odd

{% end %}
...and multi-line:

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

{% example_playground_link() %}
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
x, x
# -> (3, 3)

foo = if x > 0
  y = x * 10
  y + 3
else 
  y = x * 100
  y * y

foo, foo 
# -> (33, 33)
````

{% example_playground_link() %}
x = if 1 + 1 == 2 then 3 else -1
print x, x
# -> (3, 3)

foo = if x > 0
  y = x * 10
  y + 3
else 
  y = x * 100
  y * y

print foo, foo 
# -> (33, 33)

{% end %}
### switch

`switch` expressions can be used as more minimal alternative to `if`/`else if`/`else` 
cascades.

````koto
fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

fib 7
# -> 13
````

{% example_playground_link() %}
fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

print fib 7
# -> 13

{% end %}
### match

`match` expressions can be used to match a value against a series of patterns, 
with the matched pattern causing a specific branch of code to be executed.

Patterns can be literals or identifiers. An identifier will accept any value, so they're often used with `if` conditions to refine the match.

````koto
match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: $x'
  x if x < 50 then 'less than 50: $x'
  x then 'other: $x'
# -> less than 50: 42
````

{% example_playground_link() %}
print match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: $x'
  x if x < 50 then 'less than 50: $x'
  x then 'other: $x'
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

{% example_playground_link() %}
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
List and Tuple entries can be matched against, with `...` available for capturing the 
rest of the list.

````koto
match ['a', 'b', 'c'].extend [1, 2, 3]
  ['a', 'b'] then "A list containing 'a' and 'b'"
  [1, ...] then "Starts with '1'"
  [..., 'y', last] then "Ends with 'y' followed by '$last'"
  ['a', x, others...] then
    "Starts with 'a', followed by '$x', then ${others.size()} others"
  unmatched then "other: $unmatched"
# -> Starts with 'a', followed by 'b', then 4 others
````

{% example_playground_link() %}
print match ['a', 'b', 'c'].extend [1, 2, 3]
  ['a', 'b'] then "A list containing 'a' and 'b'"
  [1, ...] then "Starts with '1'"
  [..., 'y', last] then "Ends with 'y' followed by '$last'"
  ['a', x, others...] then
    "Starts with 'a', followed by '$x', then ${others.size()} others"
  unmatched then "other: $unmatched"
# -> Starts with 'a', followed by 'b', then 4 others

{% end %}


## Loops

### for

`for` loops can be used to iterate over any iterable value.

````koto
for n in [10, 20, 30]
  print n
# -> 10
# -> 20
# -> 30
````

{% example_playground_link() %}
for n in [10, 20, 30]
  print n
# -> 10
# -> 20
# -> 30

{% end %}
### break

Loops can be stopped early with `break`.

````koto
x = for n in (11, 22, 33, 44, 55)
  if n > 30 
    break n
x
# -> 33
````

{% example_playground_link() %}
x = for n in (11, 22, 33, 44, 55)
  if n > 30 
    break n
print x
# -> 33

{% end %}
### continue

`continue` can be used to skip ahead to the next iteration of the loop.

````koto
for n in (-2, -1, 1, 2)
  if n < 0
    continue
  print n
# -> 1
# -> 2
````

{% example_playground_link() %}
for n in (-2, -1, 1, 2)
  if n < 0
    continue
  print n
# -> 1
# -> 2

{% end %}
### while

`while` loops continue to repeat *while* a condition is true.

````koto
x = 0
while x < 5
  x += 1
x
# -> 5
````

{% example_playground_link() %}
x = 0
while x < 5
  x += 1
print x
# -> 5

{% end %}
### until

`until` loops continue to repeat *until* a condition is true.

````koto
z = [1, 2, 3]
until z.is_empty()
  print z.pop()
# -> 3
# -> 2
# -> 1
````

{% example_playground_link() %}
z = [1, 2, 3]
until z.is_empty()
  print z.pop()
# -> 3
# -> 2
# -> 1

{% end %}
### loop

`loop` creates a loop that will repeat indefinitely.

````koto
x = 0
y = loop
  x += 1
  if x > 4
    break x
y
# -> 5
````

{% example_playground_link() %}
x = 0
y = loop
  x += 1
  if x > 4
    break x
print y
# -> 5

{% end %}


## Ranges

Ranges of integers can be created with `..` or `..=`.

`..` creates a *non-inclusive* range, which defines a range from the start 
*up to but not including* the end of the range.

````koto
r = 10..20
# -> 10..20
r.start()
# -> 10
r.end()
# -> 20
r.contains 20
# -> false
````

{% example_playground_link() %}
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
r = 100..=200
# -> 100..=200
r.contains 200
# -> true
````

{% example_playground_link() %}
print r = 100..=200
# -> 100..=200
print r.contains 200
# -> true

{% end %}
Ranges are iterable, so can be used in for loops, and with the `iterator` module.

````koto
for x in 1..=3
  print x
# -> 1
# -> 2
# -> 3

(0..5).to_list()
# -> [0, 1, 2, 3, 4]
````

{% example_playground_link() %}
for x in 1..=3
  print x
# -> 1
# -> 2
# -> 3

print (0..5).to_list()
# -> [0, 1, 2, 3, 4]

{% end %}


## Advanced Functions

### Optional Arguments

When calling a function, any missing arguments will be replaced by `null`.

````koto
f = |a, b, c|
  print (a, b, c)

f 1
# -> (1, null, null)
f 1, 2
# -> (1, 2, null)
f 1, 2, 3
# -> (1, 2, 3)
````

{% example_playground_link() %}
f = |a, b, c|
  print (a, b, c)

f 1
# -> (1, null, null)
f 1, 2
# -> (1, 2, null)
f 1, 2, 3
# -> (1, 2, 3)

{% end %}
In simple cases the function can check for missing arguments by using `or`.

````koto
f = |a, b, c|
  print (a or -1, b or -2, c or -3)

f 1
# -> (1, -2, -3)
````

{% example_playground_link() %}
f = |a, b, c|
  print (a or -1, b or -2, c or -3)

f 1
# -> (1, -2, -3)

{% end %}
`or` will reject `false`, so if `false` might be a valid input then a
more-verbose direct comparison against `null` can be used instead.

````koto
f = |a| print if a == null then -1 else a

f()
# -> -1
f false
# -> false
````

{% example_playground_link() %}
f = |a| print if a == null then -1 else a

f()
# -> -1
f false
# -> false

{% end %}
### Variadic Functions

A function can accept any number of arguments by adding `...` to the last argument. 
Any additional arguments will be collected into a Tuple which will be assigned to the last argument.

````koto
f = |a, b, others...|
  print "a: $a, b: $b, others: $others"

f 1, 2, 3, 4, 5
# -> a: 1, b: 2, others: (3, 4, 5)
````

{% example_playground_link() %}
f = |a, b, others...|
  print "a: $a, b: $b, others: $others"

f 1, 2, 3, 4, 5
# -> a: 1, b: 2, others: (3, 4, 5)

{% end %}
### Argument Unpacking

Functions that expect List or Tuple arguments can *unpack* their values directly in the argument declaration.

````koto
# A function that sums a List of three values
f = |[a, b, c]| a + b + c

x = [100, 10, 1]
f x
# -> 111
````

{% example_playground_link() %}
# A function that sums a List of three values
f = |[a, b, c]| a + b + c

x = [100, 10, 1]
print f x
# -> 111

{% end %}
In the above example, if anything other than a List with three values is used as
an argument, then an error will be thrown. 

Unpacked values can contain nested unpacked values.

````koto
# A function that takes a Tuple of Lists
# and sums their entries
f = |([a, b], [c, d, e])| 
  a + b + c + d + e
x = ([1, 2], [3, 4, 5])
f x
# -> 15
````

{% example_playground_link() %}
# A function that takes a Tuple of Lists
# and sums their entries
f = |([a, b], [c, d, e])| 
  a + b + c + d + e
x = ([1, 2], [3, 4, 5])
print f x
# -> 15

{% end %}
Ellipses can be used to unpack any number of elements at the start or end of a List or Tuple. 

````koto
f = |(..., last)| last * last
x = (1, 2, 3, 4)
f x
# -> 16
````

{% example_playground_link() %}
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

{% example_playground_link() %}
f = |(first, others...)| first * others.sum()
x = (10, 1, 2, 3)
print f x
# -> 60

{% end %}
As a performance consideration, when assigning elements this way from a List, a new list will be created with copies of the elements. Unpacking elements from a Tuple is cheaper because the underlying data is shared between sub-tuples.

### Ignoring Arguments

The wildcard `_` can be used as a placeholder for arguments that the function ignores. 

````koto
# A function that takes a List,
# and sums its first and third values 
f = |[a, _, c]| a + c

f [100, 10, 1]
# -> 101
````

{% example_playground_link() %}
# A function that takes a List,
# and sums its first and third values 
f = |[a, _, c]| a + c

print f [100, 10, 1]
# -> 101

{% end %}
If you would like to keep the name of the ignored value as a reminder, 
then `_` can be used as a prefix for an identifier (Identifiers starting with 
`_` can be written to but can't be accessed).

````koto
my_map = {foo_a: 1, bar_a: 2, foo_b: 3, bar_b: 4}
my_map
  .keep |(key, _value)| key.starts_with 'foo'
  .to_tuple()
# -> (('foo_a', 1), ('foo_b', 3))
````

{% example_playground_link() %}
my_map = {foo_a: 1, bar_a: 2, foo_b: 3, bar_b: 4}
print my_map
  .keep |(key, _value)| key.starts_with 'foo'
  .to_tuple()
# -> (('foo_a', 1), ('foo_b', 3))

{% end %}
### Captured Values

If a value is accessed in a function that wasn't assigned locally, 
then the value is copied into the function (or *captured*) when it's created. 

````koto
x = 1

# x is assigned outside the function,
# so it gets captured when it's created.
f = |n| n + x 

# Reassigning x here doesn't modify the value 
# of x that was captured when f was created.
x = 100

f 2
# -> 3
````

{% example_playground_link() %}
x = 1

# x is assigned outside the function,
# so it gets captured when it's created.
f = |n| n + x 

# Reassigning x here doesn't modify the value 
# of x that was captured when f was created.
x = 100

print f 2
# -> 3

{% end %}


## Generators

Custom iterators can be made with *generator functions*, which are any functions that contain a `yield` expression. 

The iterator is paused each time `yield` is encountered, waiting for the caller to continue execution.

````koto
my_first_generator = ||
  yield 1
  yield 2
  yield 3

x = my_first_generator()
x.next()
# -> 1
x.next()
# -> 2
x.next()
# -> 3
x.next()
# -> null
````

{% example_playground_link() %}
my_first_generator = ||
  yield 1
  yield 2
  yield 3

x = my_first_generator()
print x.next()
# -> 1
print x.next()
# -> 2
print x.next()
# -> 3
print x.next()
# -> null

{% end %}
Generator functions can have arguments like any other function, and calling them creates an iterator that has access to the `iterator` core library module.

````koto
my_generator = |x|
  for y in 1..=3
    yield x + y 

my_generator(0).to_list()
# -> [1, 2, 3]
my_generator(10).to_tuple()
# -> (11, 12, 13)
````

{% example_playground_link() %}
my_generator = |x|
  for y in 1..=3
    yield x + y 

print my_generator(0).to_list()
# -> [1, 2, 3]
print my_generator(10).to_tuple()
# -> (11, 12, 13)

{% end %}
A generator that takes an iterator as an argument is known as an *iterator adaptor*. 

Inserting an adaptor into the `iterator` module makes it available in any iterator chain.

````koto
# Make an iterator adaptor that yields 
# every other value from the adapted iterator
iterator.every_other = ||
  n = 0
  loop
    # self is the iterator being adapted
    match self.next()
      # Exit when there are no more values 
      # produced by the iterator
      null then 
        return
      # If n is even, then yield a value
      value if n % 2 == 0 then 
        yield value
    n += 1

1..10
  .each |n| n * 10
  .every_other()
  .to_list()
# -> [10, 30, 50, 70, 90]
````

{% example_playground_link() %}
# Make an iterator adaptor that yields 
# every other value from the adapted iterator
iterator.every_other = ||
  n = 0
  loop
    # self is the iterator being adapted
    match self.next()
      # Exit when there are no more values 
      # produced by the iterator
      null then 
        return
      # If n is even, then yield a value
      value if n % 2 == 0 then 
        yield value
    n += 1

print 1..10
  .each |n| n * 10
  .every_other()
  .to_list()
# -> [10, 30, 50, 70, 90]

{% end %}


## Objects and Meta Maps

The behaviour of values in Koto can be customized by making an *Object*.

An Object is created by making a map that contains at least one key with a `@` 
prefix, known as a *metakey*.

Metakeys go in to the object's *metamap*. When the runtime encounters the object
while performing operations, the object's metamap will be checked for an entry 
corresponding to the operation.

In the following example, addition and subtraction are overridden for a custom 
`Foo` object.

````koto
# foo is a function that makes Foo values
foo = |n|
  data: n

  # Overloading the addition operator
  @+: |other|
    # A new Foo is made using the result 
    # of adding the two data values together
    foo self.data + other.data

  # Overloading the subtraction operator
  @-: |other|
    foo self.data - other.data

  # Overloading the multiply-assignment operator
  @*=: |other|
    self.data *= other.data
    self

foo_a = foo 10
foo_b = foo 20

(foo_a + foo_b).data
# -> 30
(foo_a - foo_b).data
# -> -10
foo_a *= foo_b
foo_a.data
# -> 200
````

{% example_playground_link() %}
# foo is a function that makes Foo values
foo = |n|
  data: n

  # Overloading the addition operator
  @+: |other|
    # A new Foo is made using the result 
    # of adding the two data values together
    foo self.data + other.data

  # Overloading the subtraction operator
  @-: |other|
    foo self.data - other.data

  # Overloading the multiply-assignment operator
  @*=: |other|
    self.data *= other.data
    self

foo_a = foo 10
foo_b = foo 20

print (foo_a + foo_b).data
# -> 30
print (foo_a - foo_b).data
# -> -10
foo_a *= foo_b
print foo_a.data
# -> 200

{% end %}
### Meta Operators

All the binary arithmetic operators can be overloaded following this pattern.

The following meta functions and values can also be defined:

#### `@negate`

The `@negate` meta key overloads the unary negation operator.

````koto
foo = |n|
  data: n
  @negate: || foo -self.data

x = -foo(100)
x.data
# -> -100
````

{% example_playground_link() %}
foo = |n|
  data: n
  @negate: || foo -self.data

x = -foo(100)
print x.data
# -> -100

{% end %}
#### `@not`

The `@not` meta key overloads the unary `not` operator.

````koto
foo = |n|
  data: n
  @not: || self.data == 0

not (foo 10)
# -> false
````

{% example_playground_link() %}
foo = |n|
  data: n
  @not: || self.data == 0

print not (foo 10)
# -> false

{% end %}
#### `@[]`

The `@[]` meta key defines how indexing the value with `[]` should behave.

````koto
foo = |n|
  data: n
  @[]: |index| self.data + index

(foo 10)[7]
# -> 17
````

{% example_playground_link() %}
foo = |n|
  data: n
  @[]: |index| self.data + index

print (foo 10)[7]
# -> 17

{% end %}
#### `@||`

The `@||` meta key defines how the value should behave when called as a
function.

````koto
foo = |n|
  data: n
  @||: || 
    self.data *= 2
    self.data

x = foo 2
x()
# -> 4
x()
# -> 8
````

{% example_playground_link() %}
foo = |n|
  data: n
  @||: || 
    self.data *= 2
    self.data

x = foo 2
print x()
# -> 4
print x()
# -> 8

{% end %}
#### `@next`

The `@next` meta key allows for values to behave as iterators.

Whenever the runtime needs to produce an iterator from a value, it will first 
check the value for an implementation of `@next`.

The `@next` function will be called repeatedly during iteration, 
with the returned value being used as the iterator output. 
When the returned value is `null` then the iterator will stop producing output. 

````koto
no_next =
  foo: 42
  bar: 99

no_next.to_tuple()
# -> (('foo', 42), ('bar', 99))

with_next = 
  start: 10
  end: 15
  @next: || 
    if self.start < self.end
      result = self.start
      self.start += 1
      result
    else 
      null
  
with_next.to_tuple()
# -> (10, 11, 12, 13, 14)
````

{% example_playground_link() %}
no_next =
  foo: 42
  bar: 99

print no_next.to_tuple()
# -> (('foo', 42), ('bar', 99))

with_next = 
  start: 10
  end: 15
  @next: || 
    if self.start < self.end
      result = self.start
      self.start += 1
      result
    else 
      null
  
print with_next.to_tuple()
# -> (10, 11, 12, 13, 14)

{% end %}
#### `@next_back`

The `@next_back` meta key is used by
[`iterator.reversed`](../../core/iterator/#reversed) when producing a reversed
iterator. 

An implementation of `@next_back` is only looked for if `@next` is also 
implemented.

````koto
iter =
  foo: 0
  @next: || self.foo += 1
  @next_back: || self.foo -= 1

iter
  .skip 3
  .reversed()
  .take 3
  .to_tuple()
# -> (2, 1, 0)
````

{% example_playground_link() %}
iter =
  foo: 0
  @next: || self.foo += 1
  @next_back: || self.foo -= 1

print iter
  .skip 3
  .reversed()
  .take 3
  .to_tuple()
# -> (2, 1, 0)

{% end %}
#### `@iterator`

The `@iterator` meta key defines how iterators should be created when the value 
is used in an iterable context. 
The function returns an iterable value that is then used during iterator 
operations.

````koto
foo = |n|
  @iterator: || 
    yield n + 1
    yield n + 2
    yield n + 3

(foo 0).to_tuple()
# -> (1, 2, 3)

(foo 100).to_list()
# -> [101, 102, 103]
````

{% example_playground_link() %}
foo = |n|
  @iterator: || 
    yield n + 1
    yield n + 2
    yield n + 3

print (foo 0).to_tuple()
# -> (1, 2, 3)

print (foo 100).to_list()
# -> [101, 102, 103]

{% end %}
Note that this key will be ignored if the value also implements `@next`, 
which implies that the value is *already* an iterator. 

#### `@display`

The `@display` meta key defines how the value should be represented when
displaying the value with functions like [`io.print`](../../core/io/#print) 
or [`string.format`](../../core/string/#format).

````koto
foo = |n|
  data: n
  @display: || 'Foo: {}'.format self.data

foo 42
# -> Foo: 42

x = foo -1
"The value of x is '$x'"
# -> The value of x is 'Foo: -1'
````

{% example_playground_link() %}
foo = |n|
  data: n
  @display: || 'Foo: {}'.format self.data

print foo 42
# -> Foo: 42

x = foo -1
print "The value of x is '$x'"
# -> The value of x is 'Foo: -1'

{% end %}
#### `@type`

The `@type` meta key takes a String as a value which is used when checking the
value's type, e.g. with [`koto.type`](../../core/koto/#type)

````koto
foo = |n|
  data: n
  @type: "Foo"

koto.type (foo 42)
# -> Foo
````

{% example_playground_link() %}
foo = |n|
  data: n
  @type: "Foo"

print koto.type (foo 42)
# -> Foo

{% end %}
#### `@base`

Objects can inherit the entries of another value by declaring it as the object's
*base value* using the `@base` metakey.

In the following example, two kinds of animals are created that share the
`speak` function from their base value.

````koto
animal = |name|
  name: name
  speak: || '${self.noise}! My name is ${self.name}!'

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

{% example_playground_link() %}
animal = |name|
  name: name
  speak: || '${self.noise}! My name is ${self.name}!'

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

Named meta entries can be inserted into the map, which will be accessible via
`.` access while not being listed as one of the map's main entries.

````koto
foo = |n|
  data: n
  @meta hello: "Hello!"
  @meta get_info: || 
    info = match self.data 
      0 then "zero"
      n if n < 0 then "negative"
      else "positive"
    "${self.data} is $info"

x = foo -1
x.hello
# -> Hello!

print x.get_info()
# -> -1 is negative

print map.keys(x).to_tuple()
# -> ('data')
````

{% example_playground_link() %}
foo = |n|
  data: n
  @meta hello: "Hello!"
  @meta get_info: || 
    info = match self.data 
      0 then "zero"
      n if n < 0 then "negative"
      else "positive"
    "${self.data} is $info"

x = foo -1
print x.hello
# -> Hello!

print x.get_info()
# -> -1 is negative

print map.keys(x).to_tuple()
# -> ('data')

{% end %}
### Sharing Meta Maps

If you're creating lots of values, then it will likely be more efficient to create a single value with the meta logic, and then share it between values using [`Map.with_meta_map`](../../core/map/#with-meta-map).

````koto
# Create an empty map for global values 
globals = {}

# Define a function that makes a Foo
foo = |data|
  # Make a map that contains `data`, 
  # along with the meta map from foo_meta
  {data}.with_meta_map globals.foo_meta

# Define some meta behaviour in foo_meta
globals.foo_meta =
  # Override the + operator
  @+: |other| foo self.data + other.data

  # Define how the value should be displayed 
  @display: || "Foo (${self.data})"

(foo 10) + (foo 20)
# -> Foo (30)
````

{% example_playground_link() %}
# Create an empty map for global values 
globals = {}

# Define a function that makes a Foo
foo = |data|
  # Make a map that contains `data`, 
  # along with the meta map from foo_meta
  {data}.with_meta_map globals.foo_meta

# Define some meta behaviour in foo_meta
globals.foo_meta =
  # Override the + operator
  @+: |other| foo self.data + other.data

  # Define how the value should be displayed 
  @display: || "Foo (${self.data})"

print (foo 10) + (foo 20)
# -> Foo (30)

{% end %}


## Errors

When an error is thrown by the Koto runtime, usually execution stops and the error is displayed in the console. 

A `try` / `catch` expression (with an optional `finally` block) can be used to catch any errors thrown by the Koto runtime, allowing execution to continue.

````koto
x = [1, 2, 3]
try
  # Do something that will throw an error 
  print x[100]
catch error 
  print "Caught an error"
finally
  print "...and finally"
# -> Caught an error
# -> ...and finally
````

{% example_playground_link() %}
x = [1, 2, 3]
try
  # Do something that will throw an error 
  print x[100]
catch error 
  print "Caught an error"
finally
  print "...and finally"
# -> Caught an error
# -> ...and finally

{% end %}
`throw` can be used to throw an error from within a Koto script.

````koto
f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: '$error'"
# -> Caught an error: '!Error!'
````

{% example_playground_link() %}
f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: '$error'"
# -> Caught an error: '!Error!'

{% end %}
`throw` can be used with a String or a Map that implements `@display`.

## Testing

### Assertions

A collection of [assertion functions](../../core/test) are available. 

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

{% example_playground_link() %}
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

Tests can be organized in a Map by defining `@test` functions. 

The tests can then be run with [`test.run_tests`](../../core/test#run-tests).

````koto
basic_tests = 
  @test add: || assert_eq 1 + 1, 2 
  @test subtract: || assert_eq 1 - 1, 0 

test.run_tests basic_tests
````

{% example_playground_link() %}
basic_tests = 
  @test add: || assert_eq 1 + 1, 2 
  @test subtract: || assert_eq 1 - 1, 0 

test.run_tests basic_tests

{% end %}
`@pre_test` and `@post_test` functions can be used to define shared setup and cleanup steps.

````koto
make_x = |n|
  data: n
  @+: |other| make_x self.data + other.data
  @-: |other| make_x self.data - other.data

x_tests =
  @pre_test: || 
    self.x1 = make_x 100
    self.x2 = make_x 200

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
# -> Testing subtraction
# -> About to fail
# -> A test failed
````

{% example_playground_link() %}
make_x = |n|
  data: n
  @+: |other| make_x self.data + other.data
  @-: |other| make_x self.data - other.data

x_tests =
  @pre_test: || 
    self.x1 = make_x 100
    self.x2 = make_x 200

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
# -> Testing subtraction
# -> About to fail
# -> A test failed

{% end %}


## Modules

### `import`

Module items can be brought into the current scope using `import`.

````koto
from list import last
from number import abs

x = [1, 2, 3]
last x
# -> 3

abs -42
# -> 42
````

{% example_playground_link() %}
from list import last
from number import abs

x = [1, 2, 3]
print last x
# -> 3

print abs -42
# -> 42

{% end %}
Multiple items from a single module can be imported using `from`.

````koto
from tuple import first, last, size
x = 'a', 'b', 'c'
first x
# -> a
last x
# -> c
size x
# -> 3
````

{% example_playground_link() %}
from tuple import first, last, size
x = 'a', 'b', 'c'
print first x
# -> a
print last x
# -> c
print size x
# -> 3

{% end %}
Imported items can be assigned to alternative names.

````koto
list_size = from list import size
tuple_size = from tuple import size
list_size [1, 2]
# -> 2
tuple_size (3, 2, 1)
# -> 3
````

{% example_playground_link() %}
list_size = from list import size
tuple_size = from tuple import size
print list_size [1, 2]
# -> 2
print tuple_size (3, 2, 1)
# -> 3

{% end %}
### `export`

`export` expressions are used to add values to a module's *exports map*.

Single values can be assigned to and exported at the same time:

````koto
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, $name!'

##################
##################

from my_module import say_hello

say_hello 'Koto'
# -> 'Hello, Koto!' 
````

{% example_playground_link() %}
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, $name!'

##################
##################

from my_module import say_hello

say_hello 'Koto'
# -> 'Hello, Koto!' 

{% end %}
When exporting multiple values, it can be convenient to use map syntax:

````koto

##################
# my_module.koto #
##################

a, b, c = 1, 2, 3

# Inline maps allow for shorthand syntax
export { a, b, c, foo: 42 }

# Map blocks can also be used with export
export 
  bar: 99
  baz: 'baz'
````

{% example_playground_link() %}

##################
# my_module.koto #
##################

a, b, c = 1, 2, 3

# Inline maps allow for shorthand syntax
export { a, b, c, foo: 42 }

# Map blocks can also be used with export
export 
  bar: 99
  baz: 'baz'

{% end %}
### `@tests` and `@main`

A module can export a `@tests` Map containing `@test` functions, which will be 
run after the module has been compiled and initialized.

Additionally, a module can export a `@main` function. 
The `@main` function will be called after the module has been compiled and
initialized, and after exported `@tests` have been successfully run.

Note that because meta entries can't be assigned locally, 
the use of `export` is optional when adding entries to the module's Meta Map.

````koto
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, $name!'

@main = || # Equivalent to export @main =
  print 'Successfully initialized `my_module`'

@tests =
  @test hello_world: ||
    print 'Testing...'
    assert_eq (say_hello 'World'), 'Hello, World!'

##################
##################

from my_module import say_hello
# -> Testing...
# -> Successfully initialized `my_module`

say_hello 'Koto'
# -> 'Hello, Koto!' 
````

{% example_playground_link() %}
##################
# my_module.koto #
##################

export say_hello = |name| 'Hello, $name!'

@main = || # Equivalent to export @main =
  print 'Successfully initialized `my_module`'

@tests =
  @test hello_world: ||
    print 'Testing...'
    assert_eq (say_hello 'World'), 'Hello, World!'

##################
##################

from my_module import say_hello
# -> Testing...
# -> Successfully initialized `my_module`

say_hello 'Koto'
# -> 'Hello, Koto!' 

{% end %}
### Module Paths

By default `import` will look for a `.koto` file
with a matching name, or for a folder with a matching name containing a
`main.koto` file.

e.g. If an `import foo` expression is encountered by the runtime, 
then a `foo.koto` file will be looked for in the same location as the current
script, and if not found then `foo/main.koto` will be checked for.

## Prelude

The prelude is a collection of items that are automatically available in a Koto
script without needing to be imported.

The core library modules are automatically available in the prelude, 
along with the following functions from the core library.

* [`io.print`](../../core/io#print)
* [`koto.type`](../../core/koto#type)
* [`test.assert`](../../core/test#assert)
* [`test.assert_eq`](../../core/test#assert-eq)
* [`test.assert_ne`](../../core/test#assert-ne)
* [`test.assert_near`](../../core/test#assert-near)