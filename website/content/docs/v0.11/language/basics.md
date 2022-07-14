+++
title = "Language Basics"
slug = "basics"
weight = 1
+++

# Language Basics

## Koto Programs

Koto programs contain a series of expressions that are evaluated by Koto's runtime.

For example, this program asks for the user's name and then offers them a
friendly greeting.

````koto
print 'Please enter your name:'
name = io.stdin().read_line()
print "Hi there, $name!"
````

{% example_playground_link() %}
play.clear_output()

print 'Please enter your name:'
name = io.stdin().read_line()
print "Hi there, $name!"

{% end %}
Try placing the above example in a file named `hello.koto`, and then running 
`koto hello.koto`.

## Comments

Single-line comments start with a `#`. 

````koto
# This is a comment, everything until the end of the line is ignored.
````

{% example_playground_link() %}
play.clear_output()

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
play.clear_output()

#- 
This is a 
multi-line 
comment.
-#

{% end %}
## Numbers

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
play.clear_output()

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
play.clear_output()

print (1 + 2) * (3 + 4)
# -> 21

{% end %}
## Booleans

Booleans are declared with the `true` and `false` keywords, and combined using
the `and` and `or` operators.

````koto
true and false
# -> false

true or false
# -> true
````

{% example_playground_link() %}
play.clear_output()

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
play.clear_output()

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
play.clear_output()

print 1 + 1 == 2
# -> true

print 99 != 100
# -> true

{% end %}
## Null

The `null` keyword is used to declare a Null value,
which is used to represent the absence of a value.

````koto
null
# -> null
````

{% example_playground_link() %}
play.clear_output()

print null
# -> null

{% end %}
### Truthiness

When `null` is encountered in a boolean context, it evaluates as `false`.

Every value except for `false` and `null` evaluates as `true`.

````koto
not null
# -> true

null or 42
# -> 42
````

{% example_playground_link() %}
play.clear_output()

print not null
# -> true

print null or 42
# -> 42

{% end %}
## Value Assignments

Values are assigned with `=`, and can be freely reassigned.

````koto
x = 42
x
# -> 42

x = true
x
# -> true
````

{% example_playground_link() %}
play.clear_output()

x = 42
print x
# -> 42

x = true
print x
# -> true

{% end %}
Arithmetic assignment operators are available, e.g. `x *= y` is shorthand for 
`x = x * y`.

````koto
a = 100
a += 11
a
# -> 111
````

{% example_playground_link() %}
play.clear_output()

a = 100
a += 11
print a
# -> 111

{% end %}
