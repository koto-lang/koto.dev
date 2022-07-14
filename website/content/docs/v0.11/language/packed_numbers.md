+++
title = "Packed Numbers"
slug = "packed_numbers"
weight = 14
+++

# Packed Numbers

Koto includes two types that combine several numbers into a single value; 
`Num2` and `Num4`.

Element-wise arithmetic operations between `Num2`s and `Num4`s are available,
while operations with a `Number` apply the operation to each element.

## Num2

A `Num2` in Koto is a packed pair of 64bit floating-point numbers,
which can be useful when dealing with operations that require pairs of numbers,
like 2D coordinates.

````koto
x = make_num2 1, 2
y = make_num2 3, 4
x + y
# -> num2(4, 6)

x[0] + y[0]
# -> 4.0

x + 10
# -> num2(11, 12)
````

{% example_playground_link() %}
play.clear_output()

x = make_num2 1, 2
y = make_num2 3, 4
print x + y
# -> num2(4, 6)

print x[0] + y[0]
# -> 4.0

print x + 10
# -> num2(11, 12)

{% end %}
## Num4

A `Num4` in Koto is a packed group of 32bit floating-point numbers,
which can be useful when working with operations that require 3 or 4 values,
like 3D coordinates, or RGB/RGBA colour values.

````koto
x = make_num4 1, 2, 3, 4
y = make_num4 5, 6, 7, 8

x[2]
# -> 3.0

x + y
# -> num4(6, 8, 10, 12)

x * 0.5
# -> num4(0.5, 1, 1.5, 2)
````

{% example_playground_link() %}
play.clear_output()

x = make_num4 1, 2, 3, 4
y = make_num4 5, 6, 7, 8

print x[2]
# -> 3.0

print x + y
# -> num4(6, 8, 10, 12)

print x * 0.5
# -> num4(0.5, 1, 1.5, 2)

{% end %}
