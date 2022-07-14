+++
title = "Functions"
slug = "functions"
weight = 3
+++

# Functions

Functions are called with arguments contained in `()` parentheses.

Functions are created using `||` (with arguments are declared between the start and end `|`), 
followed by the function's body. 

The result of the function's body is the function's result.

````koto
hi = || "Hello!"
hi()
# -> Hello!

add = |x, y| x + y
add(50, 5)
# -> 55
````

{% example_playground_link() %}
play.clear_output()

hi = || "Hello!"
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
play.clear_output()

square = |x| x * x
print square 8
# -> 64

pow = |x, y| x.pow y
print pow 2, 3
# -> 8

{% end %}
The function's body can be an indented block, with the last expression used as
the function's result.

````koto
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
f 2, 3, 4
# -> 234
````

{% example_playground_link() %}
play.clear_output()

f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
print f 2, 3, 4
# -> 234

{% end %}
