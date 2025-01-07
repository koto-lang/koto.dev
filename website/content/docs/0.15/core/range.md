+++
title = "range"
slug = "range"
+++

# range

## contains

````kototype
|Range, Number| -> Bool
````

Returns true if the provided number is within the range, and false otherwise.

````kototype
|Range, Range| -> Bool
````

Returns true if the provided range is entirely contained within the range,
and false otherwise.

### Example

````koto
(10..20).contains 15
# -> true

(200..=100).contains 100
# -> true

x = 1..10
x.contains -1
# -> false

(10..20).contains 14..18
# -> true

(100..200).contains 50..250
# -> false
````

{% example_playground_link(version = "0.15") %}
print (10..20).contains 15
# -> true

print (200..=100).contains 100
# -> true

x = 1..10
print x.contains -1
# -> false

print (10..20).contains 14..18
# -> true

print (100..200).contains 50..250
# -> false

{% end %}
## end

````kototype
|Range| -> Number
````

Returns the `end` value of the range.

### Example

````koto
(50..100).end()
# -> 100

(10..0).end()
# -> 0
````

{% example_playground_link(version = "0.15") %}
print (50..100).end()
# -> 100

print (10..0).end()
# -> 0

{% end %}
### See also

* [start](#start)

## expanded

````kototype
|Range, amount: Number| -> Range
````

Returns a copy of the input range which has been 'expanded' in both directions
by the provided `amount`. 

For an ascending range this will mean that `start` will decrease by the provided
amount, while `end` will increase.

Negative amounts will cause the range to shrink rather than grow.

### Example

````koto
(10..20).expanded 5
# -> 5..25

(10..20).expanded -2
# -> 12..18

(5..-5).expanded 5
# -> 10..-10

(5..-5).expanded -5
# -> 0..0

(5..-5).expanded -10
# -> -5..5
````

{% example_playground_link(version = "0.15") %}
print (10..20).expanded 5
# -> 5..25

print (10..20).expanded -2
# -> 12..18

print (5..-5).expanded 5
# -> 10..-10

print (5..-5).expanded -5
# -> 0..0

print (5..-5).expanded -10
# -> -5..5

{% end %}
## intersection

````kototype
|Range, Range| -> Range?
````

Returns a range representing the intersecting region of the two input ranges.

If there is no intersecting region then `null` is returned.

### Example

````koto
(10..20).intersection 5..15
# -> 10..15

(100..200).intersection 250..=150
# -> 150..200

(0..10).intersection 90..99
# -> null
````

{% example_playground_link(version = "0.15") %}
print (10..20).intersection 5..15
# -> 10..15

print (100..200).intersection 250..=150
# -> 150..200

print (0..10).intersection 90..99
# -> null

{% end %}
## is_inclusive

````kototype
|Range| -> Bool
````

Returns true if the range has a defined end which is inclusive.

### Example

````koto
(10..20).is_inclusive()
# -> false

(1..=10).is_inclusive()
# -> true

(100..).is_inclusive()
# -> false
````

{% example_playground_link(version = "0.15") %}
print (10..20).is_inclusive()
# -> false

print (1..=10).is_inclusive()
# -> true

print (100..).is_inclusive()
# -> false

{% end %}
## start

````kototype
|Range| -> Number
````

Returns the `start` value of the range.

### Example

````koto
(50..100).start()
# -> 50

(10..0).start()
# -> 10
````

{% example_playground_link(version = "0.15") %}
print (50..100).start()
# -> 50

print (10..0).start()
# -> 10

{% end %}
### See also

* [end](#end)

## union

````kototype
|Range, Number| -> Range
````

Returns the union of the range and a provided number.

If the number falls outside of the range then the resulting range will be
expanded to include the number.

````kototype
|Range, Range| -> Range
````

Returns the union of two ranges.

The resulting range will encompass all values that are contained in the two
ranges, and any values that lie between them.

### Example

````koto
(0..10).union 5
# -> 0..10

(0..10).union 99
# -> 0..100

a = 10..20
b = 40..50
a.union b
# -> 10..50
````

{% example_playground_link(version = "0.15") %}
print (0..10).union 5
# -> 0..10

print (0..10).union 99
# -> 0..100

a = 10..20
b = 40..50
print a.union b
# -> 10..50

{% end %}
