+++
title = "number"
slug = "number"
+++

# number

## abs

````kototype
|Number| -> Number
````

Returns the absolute value of the number.

### Example

````koto
-1.abs()
# -> 1

1.abs()
# -> 1
````

{% example_playground_link() %}
print -1.abs()
# -> 1

print 1.abs()
# -> 1

{% end %}
## acos

````kototype
|Number| -> Float
````

Returns the arc cosine of the number. `acos` is the inverse function of `cos`.

### Example

````koto
from number import pi

assert_near 0.acos(), pi / 2
assert_eq 1.acos(), 0
````

{% example_playground_link() %}
from number import pi

assert_near 0.acos(), pi / 2
assert_eq 1.acos(), 0

{% end %}
## acosh

````kototype
|Number| -> Float
````

Returns the inverse hyperbolic cosine of the number.

### Example

````koto
assert 0.acosh().is_nan()
assert_eq 1.acosh(), 0
assert_near 2.acosh(), 1.3169578969248166
````

{% example_playground_link() %}
assert 0.acosh().is_nan()
assert_eq 1.acosh(), 0
assert_near 2.acosh(), 1.3169578969248166

{% end %}
## and

````kototype
|Integer, Integer| -> Integer
````

Returns the bitwise combination of two integers, where a `1` in both input
positions produces a `1` in corresponding output positions.

### Example

````koto
0b1010.and 0b1100
# 0b1000
# -> 8
````

{% example_playground_link() %}
print 0b1010.and 0b1100
# 0b1000
# -> 8

{% end %}
## asin

````kototype
|Number| -> Float
````

Returns the arc sine of the number. `asin` is the inverse function of `sin`.

### Example

````koto
from number import pi

assert_eq 0.asin(), 0
assert_near 1.asin(), pi / 2
````

{% example_playground_link() %}
from number import pi

assert_eq 0.asin(), 0
assert_near 1.asin(), pi / 2

{% end %}
## asinh

````kototype
|Number| -> Float
````

Returns the inverse hyperbolic sine of the number.

### Example

````koto
assert_eq 0.asinh(), 0
assert_near 1.asinh(), 0.8813735870195429
````

{% example_playground_link() %}
assert_eq 0.asinh(), 0
assert_near 1.asinh(), 0.8813735870195429

{% end %}
## atan

````kototype
|Number| -> Float
````

Returns the arc tangent of the number. `atan` is the inverse function of `tan`.

### Example

````koto
from number import pi

assert_eq 0.atan(), 0
assert_near 1.atan(), pi / 4
````

{% example_playground_link() %}
from number import pi

assert_eq 0.atan(), 0
assert_near 1.atan(), pi / 4

{% end %}
## atanh

````kototype
|Number| -> Float
````

Returns the inverse hyperbolic tangent of the number.

### Example

````koto
-1.atanh()
# -> -inf

0.atanh()
# -> 0.0

1.atanh()
# -> inf
````

{% example_playground_link() %}
print -1.atanh()
# -> -inf

print 0.atanh()
# -> 0.0

print 1.atanh()
# -> inf

{% end %}
## atan2

````kototype
|Number, Number| -> Float
````

Returns the arc tangent of `y` and `x` in radians, using the signs of `y` and
`x` to determine the correct quadrant.

### Example

````koto
from number import pi

x, y = 1, 1

assert_near y.atan2(x), pi / 4
assert_near y.atan2(-x), pi - pi / 4
````

{% example_playground_link() %}
from number import pi

x, y = 1, 1

assert_near y.atan2(x), pi / 4
assert_near y.atan2(-x), pi - pi / 4

{% end %}
## ceil

````kototype
|Number| -> Integer
````

Returns the integer that's greater than or equal to the input.

### Example

````koto
0.5.ceil()
# -> 1

2.ceil()
# -> 2

-0.5.ceil()
# -> 0
````

{% example_playground_link() %}
print 0.5.ceil()
# -> 1

print 2.ceil()
# -> 2

print -0.5.ceil()
# -> 0

{% end %}
### See Also

* [`number.floor`](#floor)
* [`number.round`](#round)
* [`number.to_int`](#to-int)

## clamp

````kototype
|Number, Number, Number| -> Number
````

Returns the first number restricted to the range defined by the second and third
numbers.

### Example

````koto
0.clamp 1, 2
# -> 1

1.5.clamp 1, 2
# -> 1.5

3.0.clamp 1, 2
# -> 2
````

{% example_playground_link() %}
print 0.clamp 1, 2
# -> 1

print 1.5.clamp 1, 2
# -> 1.5

print 3.0.clamp 1, 2
# -> 2

{% end %}
## cos

````kototype
|Number| -> Float
````

Returns the cosine of the number.

### Example

````koto
0.cos()
# -> 1.0

number.pi.cos()
# -> -1.0
````

{% example_playground_link() %}
print 0.cos()
# -> 1.0

print number.pi.cos()
# -> -1.0

{% end %}
## cosh

````kototype
|Number| -> Float
````

Returns the hyperbolic cosine of the number.

### Example

````koto
assert_eq 0.cosh(), 1
assert_near 1.cosh(), 1.5430806348152437
````

{% example_playground_link() %}
assert_eq 0.cosh(), 1
assert_near 1.cosh(), 1.5430806348152437

{% end %}
## degrees

````kototype
|Number| -> Float
````

Converts radians into degrees.

### Example

````koto
from number import pi, tau

pi.degrees()
# -> 180.0

tau.degrees()
# -> 360.0
````

{% example_playground_link() %}
from number import pi, tau

print pi.degrees()
# -> 180.0

print tau.degrees()
# -> 360.0

{% end %}
## e

````kototype
Float
````

Provides the `e` constant.

## exp

````kototype
|Number| -> Float
````

Returns the result of applying the exponential function,
equivalent to calling `e.pow x`.

### Example

````koto
assert_eq 0.exp(), 1
assert_eq 1.exp(), number.e
````

{% example_playground_link() %}
assert_eq 0.exp(), 1
assert_eq 1.exp(), number.e

{% end %}
## exp2

````kototype
|Number| -> Float
````

Returns the result of applying the base-2 exponential function,
equivalent to calling `2.pow x`.

### Example

````koto
1.exp2()
# -> 2.0

3.exp2()
# -> 8.0
````

{% example_playground_link() %}
print 1.exp2()
# -> 2.0

print 3.exp2()
# -> 8.0

{% end %}
## flip_bits

````kototype
|Integer| -> Integer
````

Returns the input with its bits 'flipped', i.e. `1` => `0`, and `0` => `1`.

### Example

````koto
1.flip_bits()
# -> -2
````

{% example_playground_link() %}
print 1.flip_bits()
# -> -2

{% end %}
## floor

````kototype
|Number| -> Integer
````

Returns the integer that's less than or equal to the input.

### Example

````koto
0.5.floor()
# -> 0

2.floor()
# -> 2

-0.5.floor()
# -> -1
````

{% example_playground_link() %}
print 0.5.floor()
# -> 0

print 2.floor()
# -> 2

print -0.5.floor()
# -> -1

{% end %}
### See Also

* [`number.ceil`](#ceil)
* [`number.round`](#round)
* [`number.to_int`](#to-int)

## infinity

````kototype
Float
````

Provides the `∞` constant.

## is_nan

````kototype
|Number| -> Bool
````

Returns true if the number is `NaN`.

### Example

````koto
1.is_nan()
# -> false

(0 / 0).is_nan()
# -> true
````

{% example_playground_link() %}
print 1.is_nan()
# -> false

print (0 / 0).is_nan()
# -> true

{% end %}
## lerp

````kototype
|a: Number, b: Number, t: Number| -> Float
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
a, b = 1, 2

a.lerp b, 0
# -> 1
a.lerp b, 0.5
# -> 1.5
a.lerp b, 1
# -> 2

a.lerp b, -0.5
# -> 0.5
a.lerp b, 1.5
# -> 2.5
````

{% example_playground_link() %}
a, b = 1, 2

print a.lerp b, 0
# -> 1
print a.lerp b, 0.5
# -> 1.5
print a.lerp b, 1
# -> 2

print a.lerp b, -0.5
# -> 0.5
print a.lerp b, 1.5
# -> 2.5

{% end %}
## ln

````kototype
|Number| -> Float
````

Returns the natural logarithm of the number.

### Example

````koto
1.ln()
# -> 0.0

number.e.ln()
# -> 1.0
````

{% example_playground_link() %}
print 1.ln()
# -> 0.0

print number.e.ln()
# -> 1.0

{% end %}
## log2

````kototype
|Number| -> Float
````

Returns the base-2 logarithm of the number.

### Example

````koto
2.log2()
# -> 1.0

4.log2()
# -> 2.0
````

{% example_playground_link() %}
print 2.log2()
# -> 1.0

print 4.log2()
# -> 2.0

{% end %}
## log10

````kototype
|Number| -> Float
````

Returns the base-10 logarithm of the number.

### Example

````koto
10.log10()
# -> 1.0

100.log10()
# -> 2.0
````

{% example_playground_link() %}
print 10.log10()
# -> 1.0

print 100.log10()
# -> 2.0

{% end %}
## max

````kototype
|Number, Number| -> Number
````

Returns the larger of the two numbers.

### Example

````koto
1.max 2
# -> 2

4.5.max 3
# -> 4.5
````

{% example_playground_link() %}
print 1.max 2
# -> 2

print 4.5.max 3
# -> 4.5

{% end %}
## min

````kototype
|Number, Number| -> Number
````

Returns the smaller of the two numbers.

### Example

````koto
1.min 2
# -> 1

4.5.min 3
# -> 3
````

{% example_playground_link() %}
print 1.min 2
# -> 1

print 4.5.min 3
# -> 3

{% end %}
## nan

````kototype
Float
````

Provides the `NaN` (Not a Number) constant.

## negative_infinity

````kototype
Float
````

Provides the `-∞` constant.

## or

````kototype
|Integer, Integer| -> Integer
````

Returns the bitwise combination of two integers, where a `1` in either input
positions produces a `1` in corresponding output positions.

### Example

````koto
0b1010.or 0b1100
# 0b1110
# -> 14
````

{% example_playground_link() %}
print 0b1010.or 0b1100
# 0b1110
# -> 14

{% end %}
## pi

````kototype
Float
````

Provides the `π` constant.

## pi_2

````kototype
Float
````

Provides the `π` constant divided by `2`.

## pi_4

````kototype
Float
````

Provides the `π` constant divided by `4`.

## pow

````kototype
|Number, Number| -> Number
````

Returns the result of raising the first number to the power of the second.

### Example

````koto
2.pow 3
# -> 8
````

{% example_playground_link() %}
print 2.pow 3
# -> 8

{% end %}
## radians

````kototype
|Number| -> Float
````

Converts degrees into radians.

### Example

````koto
from number import pi

assert_near 90.radians(), pi / 2
assert_near 360.radians(), pi * 2
````

{% example_playground_link() %}
from number import pi

assert_near 90.radians(), pi / 2
assert_near 360.radians(), pi * 2

{% end %}
## recip

````kototype
|Number| -> Float
````

Returns the reciprocal of the number, i.e. `1 / x`.

### Example

````koto
2.recip()
# -> 0.5
````

{% example_playground_link() %}
print 2.recip()
# -> 0.5

{% end %}
## round

````kototype
|Number| -> Integer
````

Returns the nearest integer to the input number.
Half-way values round away from zero.

### Example

````koto
0.5.round()
# -> 1

2.round()
# -> 2

-0.5.round()
# -> -1
````

{% example_playground_link() %}
print 0.5.round()
# -> 1

print 2.round()
# -> 2

print -0.5.round()
# -> -1

{% end %}
### See Also

* [`number.ceil`](#ceil)
* [`number.floor`](#floor)
* [`number.to_int`](#to-int)

## shift_left

````kototype
|Integer, Integer| -> Integer
````

Returns the result of shifting the bits of the first number to the left by the
amount specified by the second number.

### Note

The shift amount must be greater than or equal to `0`.

### Example

````koto
0b1010.shift_left 2
# 0b101000
# -> 40
````

{% example_playground_link() %}
print 0b1010.shift_left 2
# 0b101000
# -> 40

{% end %}
## shift_right

````kototype
|Integer, Integer| -> Integer
````

Returns the result of shifting the bits of the first number to the right by the
amount specified by the second number.

### Note

The shift amount must be greater than or equal to `0`.

### Example

````koto
0b1010.shift_right 2
# 0b0010
# -> 2
````

{% example_playground_link() %}
print 0b1010.shift_right 2
# 0b0010
# -> 2

{% end %}
## sin

````kototype
|Number| -> Float
````

Returns the sine of the number.

### Example

````koto
from number import pi

(pi * 0.5).sin()
# -> 1.0

(pi * 1.5).sin()
# -> -1.0
````

{% example_playground_link() %}
from number import pi

print (pi * 0.5).sin()
# -> 1.0

print (pi * 1.5).sin()
# -> -1.0

{% end %}
## sinh

````kototype
|Number| -> Float
````

Returns the hyperbolic sine of the number.

### Example

````koto
assert_eq 0.sinh(), 0
assert_near 1.sinh(), 1.1752011936438014
````

{% example_playground_link() %}
assert_eq 0.sinh(), 0
assert_near 1.sinh(), 1.1752011936438014

{% end %}
## sqrt

````kototype
|Number| -> Float
````

Returns the square root of the number.

### Example

````koto
64.sqrt()
# -> 8.0
````

{% example_playground_link() %}
print 64.sqrt()
# -> 8.0

{% end %}
## tan

````kototype
|Number| -> Float
````

Returns the tangent of the number.

### Example

````koto
assert_eq 0.tan(), 0
assert_near 1.tan(), 1.557407724654902
````

{% example_playground_link() %}
assert_eq 0.tan(), 0
assert_near 1.tan(), 1.557407724654902

{% end %}
## tanh

````kototype
|Number| -> Float
````

Returns the hyperbolic tangent of the number.

### Example

````koto
assert_near 1.tanh(), 1.sinh() / 1.cosh()
````

{% example_playground_link() %}
assert_near 1.tanh(), 1.sinh() / 1.cosh()

{% end %}
## tau

````kototype
Float
````

Provides the `τ` constant, equivalent to `2π`.

## to_float

````kototype
|Number| -> Float
````

Returns the number as a `Float`.

### Example

````koto
1.to_float()
# -> 1.0
````

{% example_playground_link() %}
print 1.to_float()
# -> 1.0

{% end %}
## to_int

````kototype
|Number| -> Integer
````

Converts a Number into an integer by removing its fractional part.

This is often called `trunc` in other languages.

### Example

````koto
2.9.to_int()
# -> 2

1.5.to_int()
# -> 1

-0.5.to_int()
# -> 0

-1.9.to_int()
# -> -1
````

{% example_playground_link() %}
print 2.9.to_int()
# -> 2

print 1.5.to_int()
# -> 1

print -0.5.to_int()
# -> 0

print -1.9.to_int()
# -> -1

{% end %}
### See Also

* [`number.ceil`](#ceil)
* [`number.floor`](#floor)
* [`number.round`](#round)

## xor

````kototype
|Integer, Integer| -> Integer
````

Returns the bitwise combination of two integers,
where a `1` in one (and only one) of the input positions
produces a `1` in corresponding output positions.

### Example

````koto
0b1010.xor 0b1100
# 0b0110
# -> 6
````

{% example_playground_link() %}
print 0b1010.xor 0b1100
# 0b0110
# -> 6

{% end %}
