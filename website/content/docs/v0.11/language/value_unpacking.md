+++
title = "Value Unpacking"
slug = "value_unpacking"
weight = 6
+++

# Value Unpacking

Multiple values can be assigned at once by separating the names with commas.

````koto
a, b = 10, 20
a, b
# -> (10, 20)

my_tuple = 1, 2
x, y = my_tuple
y, x
# -> (2, 1)
````

{% example_playground_link() %}
play.clear_output()

a, b = 10, 20
print a, b
# -> (10, 20)

my_tuple = 1, 2
x, y = my_tuple
print y, x
# -> (2, 1)

{% end %}
This works with lists too.

````koto
a, b = [10, 20, 30, 40, 50]
b, a
# -> (20, 10)
````

{% example_playground_link() %}
play.clear_output()

a, b = [10, 20, 30, 40, 50]
print b, a
# -> (20, 10)

{% end %}
