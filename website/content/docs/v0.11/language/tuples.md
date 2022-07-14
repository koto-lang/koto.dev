+++
title = "Tuples"
slug = "tuples"
weight = 5
+++

# Tuples

Tuples are declared with `()` parentheses. 

````koto
x = (-1, 'abc', true)
x[1]
# -> abc
````

{% example_playground_link() %}
play.clear_output()

x = (-1, 'abc', true)
print x[1]
# -> abc

{% end %}
To create an empty Tuple or a Tuple with a single entry, use a trailing `,` inside the parentheses.

````koto
()
# -> null
(1)
# -> 1

(,)
# -> ()
(1,)
# -> (1)
````

{% example_playground_link() %}
play.clear_output()

print ()
# -> null
print (1)
# -> 1

print (,)
# -> ()
print (1,)
# -> (1)

{% end %}
In simple expressions the `()` parentheses are optional.

````koto
1, 2, 3
# -> (1, 2, 3)

x = "a", 10
y = "b", 20
x, y
# -> (('a', 10), ('b', 20))
````

{% example_playground_link() %}
play.clear_output()

print 1, 2, 3
# -> (1, 2, 3)

x = "a", 10
y = "b", 20
print x, y
# -> (('a', 10), ('b', 20))

{% end %}
Tuples behave like Lists with a fixed size and with entries that can't be replaced, 
however Lists and Maps that are contained in a Tuple can be modified due to
their interior mutability.

````koto
x = ([1, 2, 3], [4, 5, 6])
x[1][0] = 99
x[1]
# -> [99, 5, 6]
````

{% example_playground_link() %}
play.clear_output()

x = ([1, 2, 3], [4, 5, 6])
x[1][0] = 99
print x[1]
# -> [99, 5, 6]

{% end %}
