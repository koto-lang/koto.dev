+++
title = "num4"
slug = "num4"
+++

# num4

## length

````kototype
|Num4| -> Float
````

Returns the length of the vector represented by the Num4's elements.

### Example

````koto
x = make_num4(2, -2, 2, -2)
x.length()
# -> 4.0
````

{% example_playground_link() %}
play.clear_output()

x = make_num4(2, -2, 2, -2)
print x.length()
# -> 4.0

{% end %}
## lerp

````kototype
|a: Num4, b: Num4, t: Number| -> Num4
````

Linearly interpolates between `a` and `b` using the interpolation factor `t`.

The range (`a` -> `b`) corresponds to the value range of (`0` -> `1`) for `t`.

e.g.

* At `t` == `0`, the result is equal to `a`.
* At `t` == `1`, the result is equal to `b`.
* At other values of `t`, the result is a proportional mix of `a` and `b`.
* Values for `t` outside of (`0` -> `1`) will extrapolate from the (`a` -> `b`)
  range.

### Example

````koto
a = make_num4 0, 10, -10, 0
b = make_num4 10, 50, 10, 0

a.lerp b, 0
# -> num4(0, 10, -10, 0)
a.lerp b, 0.5
# -> num4(5, 30, 0, 0)
a.lerp b, 1
# -> num4(10, 50, 10, 0)

a.lerp b, -0.5
# -> num4(-5, -10, -20, 0)
a.lerp b, 1.5
# -> num4(15, 70, 20, 0)
````

{% example_playground_link() %}
play.clear_output()

a = make_num4 0, 10, -10, 0
b = make_num4 10, 50, 10, 0

print a.lerp b, 0
# -> num4(0, 10, -10, 0)
print a.lerp b, 0.5
# -> num4(5, 30, 0, 0)
print a.lerp b, 1
# -> num4(10, 50, 10, 0)

print a.lerp b, -0.5
# -> num4(-5, -10, -20, 0)
print a.lerp b, 1.5
# -> num4(15, 70, 20, 0)

{% end %}
## make_num4

````kototype
|Number| -> Num4
````

````kototype
|Number, Number| -> Num4
````

````kototype
|Number, Number, Number| -> Num4
````

````kototype
|Number, Number, Number, Number| -> Num4
````

````kototype
|Num2| -> Num4
````

````kototype
|Num4| -> Num4
````

````kototype
|Iterable| -> Num4
````

Makes a Num4 from the provided values.

### Example

````koto
make_num4 1
# -> num4(1, 1, 1, 1)

make_num4 3, 4
# -> num4(3, 4, 0, 0)

make_num4 5, 6, 7, 8
# -> num4(5, 6, 7, 8)

make_num4 [11, 12, 13, 14]
# -> num4(11, 12, 13, 14)
````

{% example_playground_link() %}
play.clear_output()

print make_num4 1
# -> num4(1, 1, 1, 1)

print make_num4 3, 4
# -> num4(3, 4, 0, 0)

print make_num4 5, 6, 7, 8
# -> num4(5, 6, 7, 8)

print make_num4 [11, 12, 13, 14]
# -> num4(11, 12, 13, 14)

{% end %}
## max

````kototype
|Num4| -> Float
````

Returns the value of the largest element in the Num4.

### Example

````koto
x = make_num4(10, 20, -50, -10)
x.max()
# -> 20.0
````

{% example_playground_link() %}
play.clear_output()

x = make_num4(10, 20, -50, -10)
print x.max()
# -> 20.0

{% end %}
## min

````kototype
|Num4| -> Float
````

Returns the value of the smallest element in the Num4.

### Example

````koto
x = make_num4(10, 20, -50, -10)
x.min()
# -> -50.0
````

{% example_playground_link() %}
play.clear_output()

x = make_num4(10, 20, -50, -10)
print x.min()
# -> -50.0

{% end %}
## normalize

````kototype
|Num4| -> Num4
````

Returns a Num4 with the same direction as the input,
with its length normalized to 1.

### Example

````koto
x = make_num4(2, -2, 2, -2)
x.normalize()
# -> num4(0.5, -0.5, 0.5, -0.5)
````

{% example_playground_link() %}
play.clear_output()

x = make_num4(2, -2, 2, -2)
print x.normalize()
# -> num4(0.5, -0.5, 0.5, -0.5)

{% end %}
## product

````kototype
|Num4| -> Float
````

Returns the result of multiplying the Num4's elements together.

### Example

````koto
x = make_num4(10, 20, -50, -10)
x.product()
# -> 100000.0
````

{% example_playground_link() %}
play.clear_output()

x = make_num4(10, 20, -50, -10)
print x.product()
# -> 100000.0

{% end %}
## sum

````kototype
|Num4| -> Float
````

Returns the result of adding the Num4's elements together.

### Example

````koto
x = make_num4(10, 20, 30, 40)
x.sum()
# -> 100.0
````

{% example_playground_link() %}
play.clear_output()

x = make_num4(10, 20, 30, 40)
print x.sum()
# -> 100.0

{% end %}
## with

````kototype
|Num4, index: Number, value: Number| -> Num4
````

Returns a Num4 with the element at `index` replaced with `value`.

### Example

````koto
x = make_num4 10, 20, 30, 40
x.with 0, 99
# -> num4(99, 20, 30, 40)
x.with 3, -1
# -> num4(10, 20, 30, -1)
````

{% example_playground_link() %}
play.clear_output()

x = make_num4 10, 20, 30, 40
print x.with 0, 99
# -> num4(99, 20, 30, 40)
print x.with 3, -1
# -> num4(10, 20, 30, -1)

{% end %}
## r

````kototype
|Num4| -> Float
````

Returns the first element of the Num4.

This can be useful when using a Num4 as a colour value, and want to access its
'red' component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.r()
# -> 10.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.r()
# -> 10.0

{% end %}
## g

````kototype
|Num4| -> Float
````

Returns the second element of the Num4.

This can be useful when using a Num4 as a colour value, and want to access its
'green' component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.g()
# -> 20.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.g()
# -> 20.0

{% end %}
## b

````kototype
|Num4| -> Float
````

Returns the third element of the Num4.

This can be useful when using a Num4 as a colour value, and want to access its
'blue' component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.b()
# -> 30.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.b()
# -> 30.0

{% end %}
## a

````kototype
|Num4| -> Float
````

Returns the fourth element of the Num4.

This can be useful when using a Num4 as a colour value, and want to access its
'alpha' component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.w()
# -> 40.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.w()
# -> 40.0

{% end %}
## x

````kototype
|Num4| -> Float
````

Returns the first element of the Num4.

This can be useful when using a Num4 as a 3D or 4D vector, and want to access
its `x` component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.x()
# -> 10.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.x()
# -> 10.0

{% end %}
## y

````kototype
|Num4| -> Float
````

Returns the second element of the Num4.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.y()
# -> 20.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.y()
# -> 20.0

{% end %}
This can be useful when using a Num4 as a 3D or 4D vector, and want to access
its `y` component.

## z

````kototype
|Num4| -> Float
````

Returns the third element of the Num4.

This can be useful when using a Num4 as a 3D or 4D vector, and want to access
its `z` component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.z()
# -> 30.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.z()
# -> 30.0

{% end %}
## w

````kototype
|Num4| -> Float
````

Returns the fourth element of the Num4.

This can be useful when using a Num4 as a 4D vector, and want to access its `w`
component.

### Example

````koto
n = make_num4 10, 20, 30, 40
n.w()
# -> 40.0
````

{% example_playground_link() %}
play.clear_output()

n = make_num4 10, 20, 30, 40
print n.w()
# -> 40.0

{% end %}
