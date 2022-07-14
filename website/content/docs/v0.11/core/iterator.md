+++
title = "iterator"
slug = "iterator"
+++

# iterator

## all

````kototype
|Iterable, |Value| -> Bool| -> Bool
````

Checks the Iterable's values against a test Function.
The Function should return `true` or `false`, and then `all` returns `true`
if all values pass the test.

`all` stops running as soon as it finds a failing test, and then `false` is
returned.

### Example

````koto
(1..9).all |x| x > 0
# -> true

('', '', 'foo').all string.is_empty
# -> false

[10, 20, 30]
  .each |x| x / 10
  .all |x| x < 10
# -> true
````

{% example_playground_link() %}
play.clear_output()

print (1..9).all |x| x > 0
# -> true

print ('', '', 'foo').all string.is_empty
# -> false

print [10, 20, 30]
  .each |x| x / 10
  .all |x| x < 10
# -> true

{% end %}
## any

````kototype
|Iterable, |Value| -> Bool| -> Bool
````

Checks the Iterable's values against a test Function.
The Function should return `true` or `false`, and then `any` returns `true`
if any of the values pass the test.

`any` stops running as soon as it finds a passing test.

### Example

````koto
(1..9).any |x| x == 5
# -> true

('', '', 'foo').any string.is_empty
# -> true

[10, 20, 30]
  .each |x| x / 10
  .any |x| x == 2
# -> true
````

{% example_playground_link() %}
play.clear_output()

print (1..9).any |x| x == 5
# -> true

print ('', '', 'foo').any string.is_empty
# -> true

print [10, 20, 30]
  .each |x| x / 10
  .any |x| x == 2
# -> true

{% end %}
## chain

````kototype
|Iterable, Iterable| -> Iterator
````

`chain` returns an iterator that iterates over the output of the first iterator,
followed by the output of the second iterator.

### Example

````koto
[1, 2].chain([3, 4, 5]).to_tuple()
# -> (1, 2, 3, 4, 5)
````

{% example_playground_link() %}
play.clear_output()

print [1, 2].chain([3, 4, 5]).to_tuple()
# -> (1, 2, 3, 4, 5)

{% end %}
## chunks

````kototype
|Iterable, Number| -> Iterator
````

Returns an iterator that splits up the input data into chunks of size `N`,
where each chunk is provided as an iterator over the chunk's elements.
The final chunk may have fewer than `N` elements.

Note that the input value should be an iterable value that has a defined range,
e.g. a List or a String (i.e. not an adapted iterator or a generator).

### Example

````koto
(1..=10)
  .chunks 3
  .each |chunk| chunk.to_list()
  .to_list()
# -> [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10]]
````

{% example_playground_link() %}
play.clear_output()

print (1..=10)
  .chunks 3
  .each |chunk| chunk.to_list()
  .to_list()
# -> [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10]]

{% end %}
## consume

````kototype
|Iterable| -> Null
````

Consumes the output of the iterator.

````kototype
|Iterable, Function| -> Null
````

Consumes the output of the iterator, calling the provided function with each
iterator output value.

### Example

````koto
result = []
(1..=10)
  .keep |n| n % 2 == 0
  .each |n| result.push n
  .consume()
result
# -> [2, 4, 6, 8, 10]

# Alternatively, calling consume with a function is equivalent to having an
# `each` / `consume` chain
result = []
(1..=10)
  .keep |n| n % 2 == 1
  .consume |n| result.push n
result
# -> [1, 3, 5, 7, 9]
````

{% example_playground_link() %}
play.clear_output()

result = []
(1..=10)
  .keep |n| n % 2 == 0
  .each |n| result.push n
  .consume()
print result
# -> [2, 4, 6, 8, 10]

# Alternatively, calling consume with a function is equivalent to having an
# `each` / `consume` chain
result = []
(1..=10)
  .keep |n| n % 2 == 1
  .consume |n| result.push n
print result
# -> [1, 3, 5, 7, 9]

{% end %}
## copy

````kototype
|Iterator| -> Iterator
````

Returns an iterator that shares the same iterable data, but with a unique
iteration position (which is part of an iterator's shared state by default).

### Note

If the iterator is a generator then some effort will be made to make the
generator's copy provide the same output as the original, however this isn't
guaranteeed to be successful. Specifically, the value stack of the copied VM
will be scanned for iterators, and each iterator will have a copy made.
Iterators that may be used in other ways by the generator (e.g. stored in
containers or function captures) won't be copied and will still have shared
state.

### Example

````koto
x = (1..=10).iter()
y = x # y shares the same iteration position as x.
z = x.copy() # z shares the same iteration data (the range 1..=10),
             # but has a unique iteration position.

x.next()
# -> 1
x.next()
# -> 2
y.next() # y shares x's iteration position.
# -> 3
z.next() # z's iteration hasn't been impacted by the advancing of x and y.
# -> 1
````

{% example_playground_link() %}
play.clear_output()

x = (1..=10).iter()
y = x # y shares the same iteration position as x.
z = x.copy() # z shares the same iteration data (the range 1..=10),
             # but has a unique iteration position.

print x.next()
# -> 1
print x.next()
# -> 2
print y.next() # y shares x's iteration position.
# -> 3
print z.next() # z's iteration hasn't been impacted by the advancing of x and y.
# -> 1

{% end %}
## count

````kototype
|Iterable| -> Number
````

Counts the number of items yielded from the iterator.

### Example

````koto
(5..15).count()
# -> 10

(0..100)
  .keep |x| x % 2 == 0
  .count()
# -> 50
````

{% example_playground_link() %}
play.clear_output()

print (5..15).count()
# -> 10

print (0..100)
  .keep |x| x % 2 == 0
  .count()
# -> 50

{% end %}
## cycle

````kototype
|Iterable| -> Iterator
````

Takes an Iterable and returns a new iterator that endlessly repeats the output
of the iterable.

### Example

````koto
(1, 2, 3)
  .cycle()
  .take 10
  .to_list()
# -> [1, 2, 3, 1, 2, 3, 1, 2, 3, 1]
````

{% example_playground_link() %}
play.clear_output()

print (1, 2, 3)
  .cycle()
  .take 10
  .to_list()
# -> [1, 2, 3, 1, 2, 3, 1, 2, 3, 1]

{% end %}
## each

````kototype
|Iterable, |Value| -> Value| -> Iterator
````

Takes an Iterable and a Function, and returns a new iterator that provides the
result of calling the function with each value in the iterable.

### Example

````koto
(2, 3, 4)
  .each |x| x * 2
  .to_list()
# -> [4, 6, 8]
````

{% example_playground_link() %}
play.clear_output()

print (2, 3, 4)
  .each |x| x * 2
  .to_list()
# -> [4, 6, 8]

{% end %}
## enumerate

````kototype
|Iterable| -> Iterator
````

Returns an iterator that provides each value along with an associated index.

### Example

````koto
('a', 'b', 'c').enumerate().to_list()
# -> [(0, 'a'), (1, 'b'), (2, 'c')]
````

{% example_playground_link() %}
play.clear_output()

print ('a', 'b', 'c').enumerate().to_list()
# -> [(0, 'a'), (1, 'b'), (2, 'c')]

{% end %}
## find

````kototype
|Iterable, |Value| -> Bool| -> Value
````

Returns the first value in the iterable that passes the test function.

The function is called for each value in the iterator, and returns either `true`
if the value is a match, or `false` if it's not.

The first matching value will cause iteration to stop.

If no match is found then Null is returned.

### Example

````koto
(10..20).find |x| x > 14 and x < 16
# -> 15

(10..20).find |x| x > 100
# -> null
````

{% example_playground_link() %}
play.clear_output()

print (10..20).find |x| x > 14 and x < 16
# -> 15

print (10..20).find |x| x > 100
# -> null

{% end %}
## flatten

````kototype
|Iterable| -> Value
````

Returns the output of the input iterator, with any nested iterable values
flattened out.

Note that only one level of flattening is performed, so any double-nested
containers will still be present in the output.

### Example

````koto
[(2, 4), [6, 8, (10, 12)]].flatten().to_list()
# -> [2, 4, 6, 8, (10, 12)]
````

{% example_playground_link() %}
play.clear_output()

print [(2, 4), [6, 8, (10, 12)]].flatten().to_list()
# -> [2, 4, 6, 8, (10, 12)]

{% end %}
### See Also

* [`iterator.find`](#find)

## fold

````kototype
|Iterable, Value, |Value, Value| -> Value| -> Value
````

Returns the result of 'folding' the iterator's values into an accumulator
function.

The function takes the accumulated value and the next iterator value,
and then returns the result of folding the value into the accumulator.

The first argument is an initial accumulated value that gets passed to the
function along with the first value from the iterator.

The result is then the final accumulated value.

This operation is also known in other languages as `reduce`, `accumulate`,
`inject`, `fold left`, along with other names.

### Example

````koto
('a', 'b', 'c').fold '', |result, x| result += x + '-'
# -> a-b-c-
````

{% example_playground_link() %}
play.clear_output()

print ('a', 'b', 'c').fold '', |result, x| result += x + '-'
# -> a-b-c-

{% end %}
### See Also

* [`iterator.product`](#product)
* [`iterator.sum`](#sum)

## generate

````kototype
|Function| -> Iterator
````

````kototype
|Number, Function| -> Value
````

Provides an iterator that yields the result of repeatedly calling the provided
function. A number of calls to the function can be provided as the first
argument.

### Example

````koto
state = {x: 0}
f = || state.x += 1
iterator.generate(f).take(5).to_list()
# -> [1, 2, 3, 4, 5]

iterator.generate(3, f).to_tuple()
# -> (6, 7, 8)
````

{% example_playground_link() %}
play.clear_output()

state = {x: 0}
f = || state.x += 1
print iterator.generate(f).take(5).to_list()
# -> [1, 2, 3, 4, 5]

print iterator.generate(3, f).to_tuple()
# -> (6, 7, 8)

{% end %}
### See Also

* [`iterator.repeat`](#repeat)

## intersperse

````kototype
|Iterable, Value| -> Iterator
````

Returns an iterator that yields a copy of the provided value between each
adjacent pair of output values.

````kototype
|Iterable, || -> Value| -> Iterator
````

Returns an iterator that yields the result of calling the provided function
between each adjacent pair of output values.

### Example

````koto
('a', 'b', 'c').intersperse('-').to_string()
# -> a-b-c

separators = (1, 2, 3).iter()
('a', 'b', 'c')
  .intersperse || separators.next()
  .to_tuple(),
# -> ('a', 1, 'b', 2, 'c')
````

{% example_playground_link() %}
play.clear_output()

print ('a', 'b', 'c').intersperse('-').to_string()
# -> a-b-c

separators = (1, 2, 3).iter()
print ('a', 'b', 'c')
  .intersperse || separators.next()
  .to_tuple(),
# -> ('a', 1, 'b', 2, 'c')

{% end %}
## iter

````kototype
|Iterable| -> Iterator
````

Returns an iterator that yields the provided iterable's values.

Iterable values will be automatically accepted by most iterator operations,
so it's usually not necessary to call `.iter()`, however it can be usefult
sometimes to make a standalone iterator for manual iteration.

Note that calling `.iter` with an `Iterator` will return the iterator without
modification. If a copy of the iterator is needed then use `.copy()`.

### Example

````koto
i = (1..10).iter()
i.skip 5
i.next()
# -> 6
````

{% example_playground_link() %}
play.clear_output()

i = (1..10).iter()
i.skip 5
print i.next()
# -> 6

{% end %}
### See Also

* [`iterator.copy`](#copy)

## keep

````kototype
|Iterable, |Value| -> Bool| -> Iterator
````

Returns an iterator that keeps only the values that pass a test function.

The function is called for each value in the iterator, and returns either `true`
if the value should be kept in the iterator output, or `false` if it should be
discarded.

### Example

````koto
(0..10).keep(|x| x % 2 == 0).to_tuple()
# -> (0, 2, 4, 6, 8)
````

{% example_playground_link() %}
play.clear_output()

print (0..10).keep(|x| x % 2 == 0).to_tuple()
# -> (0, 2, 4, 6, 8)

{% end %}
## last

````kototype
|Iterable| -> Value
````

Consumes the iterator, returning the last yielded value.

### Example

````koto
(1..100).take(5).last()
# -> 5

(0..0).last()
# -> null
````

{% example_playground_link() %}
play.clear_output()

print (1..100).take(5).last()
# -> 5

print (0..0).last()
# -> null

{% end %}
## max

````kototype
|Iterable| -> Value
````

Returns the maximum value found in the iterable.

````kototype
|Iterable, |Value| -> Value| -> Value
````

Returns the maximum value found in the iterable, based on first calling a 'key'
function with the value, and then using the resulting keys for the comparisons.

A `<` 'less than' comparison is performed between each value and the maximum
found so far, until all values in the iterator have been compared.

### Example

````koto
(8, -3, 99, -1).max()
# -> 99
````

{% example_playground_link() %}
play.clear_output()

print (8, -3, 99, -1).max()
# -> 99

{% end %}
### See Also

* [`iterator.min`](#min)
* [`iterator.min_max`](#min-max)

## min

````kototype
|Iterable| -> Value
````

Returns the minimum value found in the iterable.

````kototype
|Iterable, |Value| -> Value| -> Value
````

Returns the minimum value found in the iterable, based on first calling a 'key'
function with the value, and then using the resulting keys for the comparisons.

A `<` 'less than' comparison is performed between each value and the minimum
found so far, until all values in the iterator have been compared.

### Example

````koto
(8, -3, 99, -1).min()
# -> -3
````

{% example_playground_link() %}
play.clear_output()

print (8, -3, 99, -1).min()
# -> -3

{% end %}
### See Also

* [`iterator.max`](#max)
* [`iterator.min_max`](#min-max)

## min_max

````kototype
|Iterable| -> (Value, Value)
````

Returns the minimum and maximum values found in the iterable.

````kototype
|Iterable, |Value| -> Value| -> Value
````

Returns the minimum and maximum values found in the iterable, based on first
calling a 'key' function with the value, and then using the resulting keys for
the comparisons.

A `<` 'less than' comparison is performed between each value and both the
minimum and maximum found so far, until all values in the iterator have been
compared.

### Example

````koto
(8, -3, 99, -1).min_max()
# -> (-3, 99)
````

{% example_playground_link() %}
play.clear_output()

print (8, -3, 99, -1).min_max()
# -> (-3, 99)

{% end %}
### See Also

* [`iterator.max`](#max)
* [`iterator.min`](#min)

## next

````kototype
|Iterator| -> Value
````

Returns the next value from the iterator.

### Example

````koto
x = (1, 2).iter()
x.next()
# -> 1
x.next()
# -> 2
x.next()
# -> null
````

{% example_playground_link() %}
play.clear_output()

x = (1, 2).iter()
print x.next()
# -> 1
print x.next()
# -> 2
print x.next()
# -> null

{% end %}
## position

````kototype
|Iterable, |Value| -> Bool| -> Value
````

Returns the position of the first value in the iterable that passes the test
function.

The function is called for each value in the iterator, and returns either `true`
if the value is a match, or `false` if it's not.

The first matching value will cause iteration to stop, and the number of
steps taken to reach the matched value is returned as the result.

If no match is found then Null is returned.

### Example

````koto
(10..20).position |x| x == 15
# -> 5

(10..20).position |x| x == 99
# -> null
````

{% example_playground_link() %}
play.clear_output()

print (10..20).position |x| x == 15
# -> 5

print (10..20).position |x| x == 99
# -> null

{% end %}
### See Also

* [`iterator.find`](#find)

## product

````kototype
|Iterable| -> Value
````

Returns the result of multiplying each value in the iterable together.

### Example

````koto
(2, 3, 4).product()
# -> 24
````

{% example_playground_link() %}
play.clear_output()

print (2, 3, 4).product()
# -> 24

{% end %}
### See also

* [`iterator.fold`](#fold)
* [`iterator.sum`](#sum)

## repeat

````kototype
|Value| -> Iterator
````

````kototype
|Value, Number| -> Iterator
````

Provides an iterator that repeats the provided value. 
A number of repeats can be optionally provided as the second argument.

### Example

````koto
iterator.repeat(42).take(5).to_list()
# -> [42, 42, 42, 42, 42]

iterator.repeat(-1, 3).to_tuple()
# -> (-1, -1, -1)
````

{% example_playground_link() %}
play.clear_output()

print iterator.repeat(42).take(5).to_list()
# -> [42, 42, 42, 42, 42]

print iterator.repeat(-1, 3).to_tuple()
# -> (-1, -1, -1)

{% end %}
### See Also

* [`iterator.generate`](#generate)

## skip

````kototype
|Iterable, Number| -> Iterator
````

Skips over a number of steps in the iterator.

### Example

````koto
(100..200).skip(50).next()
# -> 150
````

{% example_playground_link() %}
play.clear_output()

print (100..200).skip(50).next()
# -> 150

{% end %}
### See also

* [`iterator.take`](#take)

## sum

````kototype
|Iterable| -> Value
````

Returns the result of adding each value in the iterable together.

### Example

````koto
(2, 3, 4).sum()
# -> 9
````

{% example_playground_link() %}
play.clear_output()

print (2, 3, 4).sum()
# -> 9

{% end %}
### See also

* [`iterator.fold`](#fold)
* [`iterator.product`](#product)

## take

````kototype
|Iterable, Number| -> Iterator
````

Provides an iterator that consumes a number of values from the input before
finishing.

### Example

````koto
(100..200).take(3).to_tuple()
# -> (100, 101, 102)
````

{% example_playground_link() %}
play.clear_output()

print (100..200).take(3).to_tuple()
# -> (100, 101, 102)

{% end %}
### See also

* [`iterator.skip`](#skip)

## to_list

````kototype
|Iterable| -> List
````

Consumes all values coming from the iterator and places them in a list.

### Example

````koto
('a', 42, (-1, -2)).to_list()
# -> ['a', 42, (-1, -2)]
````

{% example_playground_link() %}
play.clear_output()

print ('a', 42, (-1, -2)).to_list()
# -> ['a', 42, (-1, -2)]

{% end %}
### See also

* [`iterator.to_map`](#to-map)
* [`iterator.to_string`](#to-string)
* [`iterator.to_tuple`](#to-tuple)

## to_map

````kototype
|Iterable| -> Map
````

Consumes all values coming from the iterator and places them in a map.

If a value is a tuple, then the first element in the tuple will be inserted as
the key for the map entry, and the second element will be inserted as the value.

If the value is anything other than a tuple, then it will be inserted as the map
key, with Null as the entry's value.

### Example

````koto
('a', 'b', 'c').to_map()
# -> {a: null, b: null, c: null}

('a', 'bbb', 'cc')
  .each |x| x, x.size()
  .to_map()
# -> {a: 1, bbb: 3, cc: 2}
````

{% example_playground_link() %}
play.clear_output()

print ('a', 'b', 'c').to_map()
# -> {a: null, b: null, c: null}

print ('a', 'bbb', 'cc')
  .each |x| x, x.size()
  .to_map()
# -> {a: 1, bbb: 3, cc: 2}

{% end %}
### See also

* [`iterator.to_list`](#to-list)
* [`iterator.to_string`](#to-string)
* [`iterator.to_tuple`](#to-tuple)

## to_num2

````kototype
|Iterable| -> Num2
````

Consumes up to 2 values from the iterator and places them in a Num2.

### Example

````koto
[1].to_num2()
# -> num2(1, 0)
(1..10).keep(|n| n % 2 == 0).to_num2()
# -> num2(2, 4)
````

{% example_playground_link() %}
play.clear_output()

print [1].to_num2()
# -> num2(1, 0)
print (1..10).keep(|n| n % 2 == 0).to_num2()
# -> num2(2, 4)

{% end %}
### See also

* [`iterator.to_num4`](#to-num4)

## to_num4

````kototype
|Iterable| -> Num4
````

Consumes up to 4 values from the iterator and places them in a Num2.

### Example

````koto
[1].to_num4()
# -> num4(1, 0, 0, 0)
(1..10).keep(|n| n % 2 == 0).to_num4()
# -> num4(2, 4, 6, 8)
````

{% example_playground_link() %}
play.clear_output()

print [1].to_num4()
# -> num4(1, 0, 0, 0)
print (1..10).keep(|n| n % 2 == 0).to_num4()
# -> num4(2, 4, 6, 8)

{% end %}
### See also

* [`iterator.to_num4`](#to-num4)

## to_string

````kototype
|Iterable| -> String
````

Consumes all values coming from the iterator and produces a string containing
the formatted values.

### Example

````koto
('x', 'y', 'z').to_string()
# -> xyz

(1, 2, 3).intersperse('-').to_string()
# -> 1-2-3
````

{% example_playground_link() %}
play.clear_output()

print ('x', 'y', 'z').to_string()
# -> xyz

print (1, 2, 3).intersperse('-').to_string()
# -> 1-2-3

{% end %}
### See also

* [`iterator.to_list`](#to-list)
* [`iterator.to_map`](#to-map)
* [`iterator.to_tuple`](#to-tuple)

## to_tuple

````kototype
|Iterable| -> Tuple
````

Consumes all values coming from the iterator and places them in a tuple.

### Example

````koto
('a', 42, (-1, -2)).to_list()
# -> ['a', 42, (-1, -2)]
````

{% example_playground_link() %}
play.clear_output()

print ('a', 42, (-1, -2)).to_list()
# -> ['a', 42, (-1, -2)]

{% end %}
### See also

* [`iterator.to_list`](#to-list)
* [`iterator.to_map`](#to-map)
* [`iterator.to_string`](#to-string)

## windows

````kototype
|Iterable, Number| -> Iterator
````

Returns an iterator that splits up the input data into overlapping windows of
size `N`, where each window is provided as an iterator over the chunk's
elements.

If the input has fewer than `N` elements then no windows will be produced.

Note that the input value should be an iterable value that has a defined range,
e.g. a List or a String (i.e. not an adapted iterator or a generator).

### Example

````koto
(1..=5).windows(3).each(iterator.to_list).to_list(),
# -> [[1, 2, 3], [2, 3, 4], [3, 4, 5]]
````

{% example_playground_link() %}
play.clear_output()

print (1..=5).windows(3).each(iterator.to_list).to_list(),
# -> [[1, 2, 3], [2, 3, 4], [3, 4, 5]]

{% end %}
## zip

````kototype
|Iterable, Iterable| -> Iterator
````

Combines the values in two iterables into an iterator that provides
corresponding pairs of values, one at a time from each input iterable.

### Example

````koto
(1, 2, 3).zip(('a', 'b', 'c')).to_list()
# -> [(1, 'a'), (2, 'b'), (3, 'c')]
````

{% example_playground_link() %}
play.clear_output()

print (1, 2, 3).zip(('a', 'b', 'c')).to_list()
# -> [(1, 'a'), (2, 'b'), (3, 'c')]

{% end %}
