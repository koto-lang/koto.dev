+++
title = "Lists"
slug = "lists"
weight = 4
+++

# Lists

Lists are declared with `[]` braces and can contain any value types.

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
play.clear_output()

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
play.clear_output()

x = [10, 20, 30]
y = x
y[1] = 99
print x # x and y share the same data
# -> [10, 99, 30]

{% end %}
