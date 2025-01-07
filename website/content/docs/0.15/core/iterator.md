+++
title = "iterator"
slug = "iterator"
+++

# iterator

## all

````kototype
|Iterable, test: |Any| -> Bool| -> Bool
````

Checks the Iterable's values against a test function.

The test function should return `true` if the value passes the test, otherwise
it should return `false`. 

`all` will return `true` if *all* values pass the test, otherwise it will return
`false`.

`all` stops running as soon as it finds a value that fails the test.

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

{% example_playground_link(version = "0.15") %}
print (1..9).all |x| x > 0
# -> true

print ('', '', 'foo').all string.is_empty
# -> false

print [10, 20, 30]
  .each |x| x / 10
  .all |x| x < 10
# -> true

{% end %}
### See Also

* [`iterator.any`](#any)

## any

````kototype
|Iterable, test: |Any| -> Bool| -> Bool
````

Checks the Iterable's values against a test function.

The test function should return `true` if the value passes the test, otherwise
it should return `false`. 

`any` will return `true` if *any* of the values pass the test, 
otherwise it will return `false`.

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

{% example_playground_link(version = "0.15") %}
print (1..9).any |x| x == 5
# -> true

print ('', '', 'foo').any string.is_empty
# -> true

print [10, 20, 30]
  .each |x| x / 10
  .any |x| x == 2
# -> true

{% end %}
### See Also

* [`iterator.all`](#all)

## chain

````kototype
|first: Iterable, second: Iterable| -> Iterator
````

`chain` returns an iterator that iterates over the output of the first iterator,
followed by the output of the second iterator.

### Example

````koto
[1, 2]
  .chain 'abc'
  .to_tuple()
# -> (1, 2, 'a', 'b', 'c')
````

{% example_playground_link(version = "0.15") %}
print [1, 2]
  .chain 'abc'
  .to_tuple()
# -> (1, 2, 'a', 'b', 'c')

{% end %}
## chunks

````kototype
|Iterable, size: Number| -> Iterator
````

Returns an iterator that splits up the input data into chunks of size `N`,
where each chunk is provided as a Tuple.
The final chunk may have fewer than `N` elements.

### Example

````koto
1..=10
  .chunks 3
  .to_list()
# -> [(1, 2, 3), (4, 5, 6), (7, 8, 9), (10)]
````

{% example_playground_link(version = "0.15") %}
print 1..=10
  .chunks 3
  .to_list()
# -> [(1, 2, 3), (4, 5, 6), (7, 8, 9), (10)]

{% end %}
## consume

````kototype
|Iterable| -> Null
````

Consumes the output of the iterator.

````kototype
|Iterable, |Any| -> Any| -> Null
````

Consumes the output of the iterator, calling the provided function with each
iterator output value.

### Example

````koto
result = []
1..=10
  .keep |n| n % 2 == 0
  .each |n| result.push n
  .consume()
result
# -> [2, 4, 6, 8, 10]

# Alternatively, calling consume with a function is equivalent to having an
# `each` / `consume` chain
result = []
1..=10
  .keep |n| n % 2 == 1
  .consume |n| result.push n
result
# -> [1, 3, 5, 7, 9]
````

{% example_playground_link(version = "0.15") %}
result = []
1..=10
  .keep |n| n % 2 == 0
  .each |n| result.push n
  .consume()
print result
# -> [2, 4, 6, 8, 10]

# Alternatively, calling consume with a function is equivalent to having an
# `each` / `consume` chain
result = []
1..=10
  .keep |n| n % 2 == 1
  .consume |n| result.push n
print result
# -> [1, 3, 5, 7, 9]

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

0..100
  .keep |x| x % 2 == 0
  .count()
# -> 50
````

{% example_playground_link(version = "0.15") %}
print (5..15).count()
# -> 10

print 0..100
  .keep |x| x % 2 == 0
  .count()
# -> 50

{% end %}
## cycle

````kototype
|Iterable| -> Iterator
````

Takes an Iterable and returns a new iterator that endlessly repeats the
iterable's output.

The iterable's output gets cached, which may result in a large amount of memory
being used if the cycle has a long length.

### Example

````koto
(1, 2, 3)
  .cycle()
  .take 10
  .to_list()
# -> [1, 2, 3, 1, 2, 3, 1, 2, 3, 1]
````

{% example_playground_link(version = "0.15") %}
print (1, 2, 3)
  .cycle()
  .take 10
  .to_list()
# -> [1, 2, 3, 1, 2, 3, 1, 2, 3, 1]

{% end %}
## each

````kototype
|Iterable, function: |Any| -> Any| -> Iterator
````

Creates a new iterator that yields the result of calling the provided `function` 
with each value from the input iterator.

### Example

````koto
(2, 3, 4)
  .each |x| x * 2
  .to_list()
# -> [4, 6, 8]
````

{% example_playground_link(version = "0.15") %}
print (2, 3, 4)
  .each |x| x * 2
  .to_list()
# -> [4, 6, 8]

{% end %}
## enumerate

````kototype
|Iterable| -> Iterator
````

Creates an iterator that yields each value along with an associated index.

### Example

````koto
('a', 'b', 'c').enumerate().to_list()
# -> [(0, 'a'), (1, 'b'), (2, 'c')]
````

{% example_playground_link(version = "0.15") %}
print ('a', 'b', 'c').enumerate().to_list()
# -> [(0, 'a'), (1, 'b'), (2, 'c')]

{% end %}
## find

````kototype
|Iterable, test: |Any| -> Bool| -> Any?
````

Returns the first value in the iterable that passes the test function.

The function is called for each value in the iterator, and should return either
`true` if the value is a match, or `false` if it's not.

The first matching value will cause iteration to stop.

If no match is found then `null` is returned.

### Example

````koto
(10..20).find |x| x > 14 and x < 16
# -> 15

(10..20).find |x| x > 100
# -> null
````

{% example_playground_link(version = "0.15") %}
print (10..20).find |x| x > 14 and x < 16
# -> 15

print (10..20).find |x| x > 100
# -> null

{% end %}
## flatten

````kototype
|Iterable| -> Iterator
````

Returns the output of the input iterator, with any nested iterable values
flattened out.

Note that only one level of flattening is performed, so any double-nested
containers will still be present in the output.

### Example

````koto
[(2, 4), [6, 8, (10, 12)]]
  .flatten()
  .to_list()
# -> [2, 4, 6, 8, (10, 12)]
````

{% example_playground_link(version = "0.15") %}
print [(2, 4), [6, 8, (10, 12)]]
  .flatten()
  .to_list()
# -> [2, 4, 6, 8, (10, 12)]

{% end %}
### See Also

* [`iterator.find`](#find)

## fold

````kototype
|
  input: Iterable, 
  initial_value: Any, 
  accumulator: |accumulated: Any, next: Any| -> Any
| -> Any
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
('a', 'b', 'c')
  .fold [], |result, x| 
    result.push x
    result.push '-'
# -> ['a', '-', 'b', '-', 'c', '-']
````

{% example_playground_link(version = "0.15") %}
print ('a', 'b', 'c')
  .fold [], |result, x| 
    result.push x
    result.push '-'
# -> ['a', '-', 'b', '-', 'c', '-']

{% end %}
### See Also

* [`iterator.product`](#product)
* [`iterator.sum`](#sum)

## generate

````kototype
|generator: || -> Any| -> Iterator
````

Creates an iterator that yields the result of repeatedly calling the `generator`
function. 

*Warning*: This version of `generate` will iterate endlessly, so consider using 
an adaptor like [`iterator.take`](#take) to produce an iterator that has an end.

````kototype
|n: Number, generator: || -> Any| -> Any
````

Creates an iterator that yields the result of calling the `generator`
function `n` times.

### Example

````koto
from iterator import generate

state = {x: 0}
f = || state.x += 1

generate(f)
  .take(5)
  .to_list()
# -> [1, 2, 3, 4, 5]

generate(3, f).to_tuple()
# -> (6, 7, 8)
````

{% example_playground_link(version = "0.15") %}
from iterator import generate

state = {x: 0}
f = || state.x += 1

print generate(f)
  .take(5)
  .to_list()
# -> [1, 2, 3, 4, 5]

print generate(3, f).to_tuple()
# -> (6, 7, 8)

{% end %}
### See Also

* [`iterator.repeat`](#repeat)

## intersperse

````kototype
|Iterable, value: Any| -> Iterator
````

Returns an iterator that yields a copy of the provided value between each
adjacent pair of output values.

````kototype
|Iterable, generator: || -> Any| -> Iterator
````

Returns an iterator that yields the result of calling the provided function
between each adjacent pair of output values.

### Example

````koto
('a', 'b', 'c').intersperse('-').to_string()
# -> a-b-c

separators = (1, 2, 3).iter()
('a', 'b', 'c')
  .intersperse || separators.next().get()
  .to_tuple(),
# -> ('a', 1, 'b', 2, 'c')
````

{% example_playground_link(version = "0.15") %}
print ('a', 'b', 'c').intersperse('-').to_string()
# -> a-b-c

separators = (1, 2, 3).iter()
print ('a', 'b', 'c')
  .intersperse || separators.next().get()
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
modification. If a copy of the iterator is needed then see `koto.copy` and
`koto.deep_copy`.

### Example

````koto
i = (1..10).iter()
i.skip 5
i.next().get()
# -> 6
````

{% example_playground_link(version = "0.15") %}
i = (1..10).iter()
i.skip 5
print i.next().get()
# -> 6

{% end %}
### See Also

* [`koto.copy`](./koto#copy)
* [`koto.deep_copy`](./koto#deep-copy)

## keep

````kototype
|Iterable, test: |Any| -> Bool| -> Iterator
````

Returns an iterator that keeps only the values that pass a test function.

The function is called for each value in the iterator, and should return either
`true` if the value should be kept in the iterator output, or `false` if it
should be discarded.

### Example

````koto
0..10
  .keep |x| x % 2 == 0
  .to_tuple()
# -> (0, 2, 4, 6, 8)
````

{% example_playground_link(version = "0.15") %}
print 0..10
  .keep |x| x % 2 == 0
  .to_tuple()
# -> (0, 2, 4, 6, 8)

{% end %}
## last

````kototype
|Iterable| -> Any?
````

Consumes the iterator, returning the last yielded value.

### Example

````koto
(1..100).take(5).last()
# -> 5

(0..0).last()
# -> null
````

{% example_playground_link(version = "0.15") %}
print (1..100).take(5).last()
# -> 5

print (0..0).last()
# -> null

{% end %}
## max

````kototype
|Iterable| -> Any
````

Returns the maximum value found in the iterable.

````kototype
|Iterable, key: |Any| -> Any| -> Any
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

{% example_playground_link(version = "0.15") %}
print (8, -3, 99, -1).max()
# -> 99

{% end %}
### See Also

* [`iterator.min`](#min)
* [`iterator.min_max`](#min-max)

## min

````kototype
|Iterable| -> Any
````

Returns the minimum value found in the iterable.

````kototype
|Iterable, key: |Any| -> Any| -> Any
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

{% example_playground_link(version = "0.15") %}
print (8, -3, 99, -1).min()
# -> -3

{% end %}
### See Also

* [`iterator.max`](#max)
* [`iterator.min_max`](#min-max)

## min_max

````kototype
|Iterable| -> (Any, Any)
````

Returns the minimum and maximum values found in the iterable.

````kototype
|Iterable, key: |Any| -> Any| -> Any
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

{% example_playground_link(version = "0.15") %}
print (8, -3, 99, -1).min_max()
# -> (-3, 99)

{% end %}
### See Also

* [`iterator.max`](#max)
* [`iterator.min`](#min)

## next

````kototype
|Iterable| -> IteratorOutput?
````

Returns the next value from the iterator wrapped in an 
[`IteratorOutput`](#iteratoroutput), 
or `null` if the iterator has been exhausted.

### Example

````koto
x = (1, null, 'x').iter()
x.next()
# -> IteratorOutput(1)
x.next()
# -> IteratorOutput(null)
x.next()
# -> IteratorOutput(x)
x.next()
# -> null

# Call .get() to access the value from an IteratorOutput
'abc'.next().get()
# -> a
````

{% example_playground_link(version = "0.15") %}
x = (1, null, 'x').iter()
print x.next()
# -> IteratorOutput(1)
print x.next()
# -> IteratorOutput(null)
print x.next()
# -> IteratorOutput(x)
print x.next()
# -> null

# Call .get() to access the value from an IteratorOutput
print 'abc'.next().get()
# -> a

{% end %}
### See Also

* [`iterator.next_back`](#next-back)

## next_back

````kototype
|Iterable| -> IteratorOutput?
````

Returns the next value from the end of the iterator wrapped in an 
[`IteratorOutput`](#iteratoroutput), 
or `null` if the iterator has been exhausted.

This only works with iterators that have a defined end, so attempting to call
`next_back` on endless iterators like [`iterator.generate`](#generate) will 
result in an error.

### Example

````koto
x = (1..=5).iter()
x.next_back()
# -> IteratorOutput(5)
x.next_back()
# -> IteratorOutput(4)

# calls to next and next_back can be mixed together
x.next()
# -> IteratorOutput(1)
x.next_back()
# -> IteratorOutput(3)
x.next_back()
# -> IteratorOutput(2)

# 1 has already been produced by the iterator, so it's now exhausted
x.next_back()
# -> null
````

{% example_playground_link(version = "0.15") %}
x = (1..=5).iter()
print x.next_back()
# -> IteratorOutput(5)
print x.next_back()
# -> IteratorOutput(4)

# calls to next and next_back can be mixed together
print x.next()
# -> IteratorOutput(1)
print x.next_back()
# -> IteratorOutput(3)
print x.next_back()
# -> IteratorOutput(2)

# 1 has already been produced by the iterator, so it's now exhausted
print x.next_back()
# -> null

{% end %}
### See Also

* [`iterator.next`](#next)
* [`iterator.reversed`](#reversed)

## once

````kototype
|Any| -> Iterator
````

Returns an iterator that yields the given value a single time.

### Example

````koto
iterator.once(99)
  .chain('abc')
  .to_tuple()
# -> (99, 'a', 'b', 'c')
````

{% example_playground_link(version = "0.15") %}
print iterator.once(99)
  .chain('abc')
  .to_tuple()
# -> (99, 'a', 'b', 'c')

{% end %}
### See Also

* [`iterator.generate`](#generate)
* [`iterator.repeat`](#repeat)

## peekable

````kototype
|Iterable| -> Peekable
````

Wraps the given iterable value in a peekable iterator.

### Peekable.peek

Returns the next value from the iterator without advancing it. 
The peeked value is cached until the iterator is advanced.

#### Example

````koto
x = 'abc'.peekable()
x.peek()
# -> IteratorOutput(a)
x.peek()
# -> IteratorOutput(a)
x.next()
# -> IteratorOutput(a)
x.peek()
# -> IteratorOutput(b)
x.next(), x.next()
# -> (IteratorOutput(b), IteratorOutput(c))
x.peek()
# -> null
````

{% example_playground_link(version = "0.15") %}
x = 'abc'.peekable()
print x.peek()
# -> IteratorOutput(a)
print x.peek()
# -> IteratorOutput(a)
print x.next()
# -> IteratorOutput(a)
print x.peek()
# -> IteratorOutput(b)
print x.next(), x.next()
# -> (IteratorOutput(b), IteratorOutput(c))
print x.peek()
# -> null

{% end %}
#### See Also

* [`iterator.next`](#next)

### Peekable.peek_back

Returns the next value from the end of the iterator without advancing it. 
The peeked value is cached until the iterator is advanced.

#### Example

````koto
x = 'abc'.peekable()
x.peek_back()
# -> IteratorOutput(c)
x.next_back()
# -> IteratorOutput(c)
x.peek()
# -> IteratorOutput(a)
x.peek_back()
# -> IteratorOutput(b)
x.next_back(), x.next_back()
# -> (IteratorOutput(b), IteratorOutput(a))
x.peek_back()
# -> null
````

{% example_playground_link(version = "0.15") %}
x = 'abc'.peekable()
print x.peek_back()
# -> IteratorOutput(c)
print x.next_back()
# -> IteratorOutput(c)
print x.peek()
# -> IteratorOutput(a)
print x.peek_back()
# -> IteratorOutput(b)
print x.next_back(), x.next_back()
# -> (IteratorOutput(b), IteratorOutput(a))
print x.peek_back()
# -> null

{% end %}
#### See Also

* [`iterator.next_back`](#next-back)

## position

````kototype
|Iterable, test: |Any| -> Bool| -> Any
````

Returns the position of the first value in the iterable that passes the test
function.

The function is called for each value in the iterator, and should return either
`true` if the value is a match, or `false` if it's not.

The first matching value will cause iteration to stop, and the number of
steps taken to reach the matched value is returned as the result.

If no match is found then `null` is returned.

### Example

````koto
(10..20).position |x| x == 15
# -> 5

(10..20).position |x| x == 99
# -> null
````

{% example_playground_link(version = "0.15") %}
print (10..20).position |x| x == 15
# -> 5

print (10..20).position |x| x == 99
# -> null

{% end %}
### See Also

* [`iterator.find`](#find)

## product

````kototype
|Iterable| -> Any
````

Returns the result of multiplying each value in the iterable together, using `1`
as the initial value.

### Example

````koto
# Find the product of a sequence of numbers
(2, 3, 4).product()
# -> 24

my_type = |n|
  n: n
  @*: |other| my_type self.n * other.n
  @display: || 'my_type({self.n})'

# Find the sum of a sequence of my_type values
(my_type(2), my_type(3)).product my_type(10)
# -> my_type(60)
````

{% example_playground_link(version = "0.15") %}
# Find the product of a sequence of numbers
print (2, 3, 4).product()
# -> 24

my_type = |n|
  n: n
  @*: |other| my_type self.n * other.n
  @display: || 'my_type({self.n})'

# Find the sum of a sequence of my_type values
print (my_type(2), my_type(3)).product my_type(10)
# -> my_type(60)

{% end %}
### See also

* [`iterator.fold`](#fold)
* [`iterator.sum`](#sum)

## repeat

````kototype
|value: Any| -> Iterator
````

Creates an iterator that endlessly yields the provided `value`.

*Warning*: This version of `repeat` will iterate endlessly, so consider using 
an adaptor like [`iterator.take`](#take) to produce an iterator with an end.

````kototype
|value: Any, n: Number| -> Iterator
````

Creates an iterator that yields `n` repeats of the provided `value`.

### Example

````koto
from iterator import repeat

repeat(42).take(5).to_list()
# -> [42, 42, 42, 42, 42]

repeat('x', 3).to_tuple()
# -> ('x', 'x', 'x')
````

{% example_playground_link(version = "0.15") %}
from iterator import repeat

print repeat(42).take(5).to_list()
# -> [42, 42, 42, 42, 42]

print repeat('x', 3).to_tuple()
# -> ('x', 'x', 'x')

{% end %}
### See Also

* [`iterator.generate`](#generate)
* [`iterator.once`](#once)

## reversed

````kototype
|Iterator| -> Iterator
````

Reverses the order of the iterator's output.

This only works with iterators that have a defined end, so attempting to reverse
endless iterators like [`iterator.generate`](#generate) will result in an error.

### Example

````koto
'Héllö'.reversed().to_tuple()
# -> ('ö', 'l', 'l', 'é', 'H')

(1..=10).reversed().skip(5).to_tuple()
# -> (5, 4, 3, 2, 1)
````

{% example_playground_link(version = "0.15") %}
print 'Héllö'.reversed().to_tuple()
# -> ('ö', 'l', 'l', 'é', 'H')

print (1..=10).reversed().skip(5).to_tuple()
# -> (5, 4, 3, 2, 1)

{% end %}
## skip

````kototype
|Iterable, steps: Number| -> Iterator
````

Skips over a number of steps in the iterator.

### Example

````koto
(100..200).skip(50).next().get()
# -> 150
````

{% example_playground_link(version = "0.15") %}
print (100..200).skip(50).next().get()
# -> 150

{% end %}
### See also

* [`iterator.step`](#step)
* [`iterator.take`](#take)

## step

````kototype
|Iterable, step_size: Number| -> Iterator
````

Steps over the iterable's output by the provided step size.

### Example

````koto
(0..10).step(3).to_tuple()
# -> (0, 3, 6, 9)

'Héllö'.step(2).to_string()
# -> Hlö
````

{% example_playground_link(version = "0.15") %}
print (0..10).step(3).to_tuple()
# -> (0, 3, 6, 9)

print 'Héllö'.step(2).to_string()
# -> Hlö

{% end %}
### See also

* [`iterator.skip`](#skip)

## sum

````kototype
|Iterable| -> Any
````

Returns the result of adding each value in the iterable together, using `0` as
the initial value.

````kototype
|Iterable, initial_value: Any| -> Any
````

Returns the result of adding each value in the iterable together, using the
provided initial value as the start of the operation.

### Example

````koto
# Find the sum of a sequence of numbers
(2, 3, 4).sum()
# -> 9

my_type = |n|
  n: n
  @+: |other| my_type self.n + other.n
  @display: || 'my_type({self.n})'

# Find the sum of a sequence of my_type values
(my_type(1), my_type(2)).sum my_type(100)
# -> my_type(103)
````

{% example_playground_link(version = "0.15") %}
# Find the sum of a sequence of numbers
print (2, 3, 4).sum()
# -> 9

my_type = |n|
  n: n
  @+: |other| my_type self.n + other.n
  @display: || 'my_type({self.n})'

# Find the sum of a sequence of my_type values
print (my_type(1), my_type(2)).sum my_type(100)
# -> my_type(103)

{% end %}
### See also

* [`iterator.fold`](#fold)
* [`iterator.product`](#product)

## take

````kototype
|Iterable, count: Number| -> Iterator
````

Creates an iterator that yields a number of values from the input before
finishing.

````kototype
|Iterable, test: |Any| -> Bool| -> Iterator
````

Creates an iterator that yields values from the input while they pass a
test function.

The test function should return `true` if the iterator should continue to yield
values, and `false` if the iterator should stop yielding values.

### Example

````koto
(100..200).take(3).to_tuple()
# -> (100, 101, 102)

'hey!'.take(|c| c != '!').to_string()
# -> hey
````

{% example_playground_link(version = "0.15") %}
print (100..200).take(3).to_tuple()
# -> (100, 101, 102)

print 'hey!'.take(|c| c != '!').to_string()
# -> hey

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

{% example_playground_link(version = "0.15") %}
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
key, with `null` as the entry's value.

### Example

````koto
('a', 'b', 'c').to_map()
# -> {a: null, b: null, c: null}

('a', 'bbb', 'cc')
  .each |x| x, size x
  .to_map()
# -> {a: 1, bbb: 3, cc: 2}
````

{% example_playground_link(version = "0.15") %}
print ('a', 'b', 'c').to_map()
# -> {a: null, b: null, c: null}

print ('a', 'bbb', 'cc')
  .each |x| x, size x
  .to_map()
# -> {a: 1, bbb: 3, cc: 2}

{% end %}
### See also

* [`iterator.to_list`](#to-list)
* [`iterator.to_string`](#to-string)
* [`iterator.to_tuple`](#to-tuple)

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

{% example_playground_link(version = "0.15") %}
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
('a', 42, (-1, -2)).to_tuple()
# -> ('a', 42, (-1, -2))
````

{% example_playground_link(version = "0.15") %}
print ('a', 42, (-1, -2)).to_tuple()
# -> ('a', 42, (-1, -2))

{% end %}
### See also

* [`iterator.to_list`](#to-list)
* [`iterator.to_map`](#to-map)
* [`iterator.to_string`](#to-string)

## windows

````kototype
|Iterable, size: Number| -> Iterator
````

Returns an iterator that splits up the input data into overlapping windows of
the specified `size`, where each window is provided as a Tuple.

If the input has fewer elements than the window size, then no windows will be
produced.

### Example

````koto
1..=5
  .windows 3
  .to_list(),
# -> [(1, 2, 3), (2, 3, 4), (3, 4, 5)]
````

{% example_playground_link(version = "0.15") %}
print 1..=5
  .windows 3
  .to_list(),
# -> [(1, 2, 3), (2, 3, 4), (3, 4, 5)]

{% end %}
## zip

````kototype
|first: Iterable, second: Iterable| -> Iterator
````

Combines the values in two iterables into an iterator that yields
corresponding pairs of values, one at a time from each input iterable.

### Example

````koto
(1, 2, 3)
  .zip ('a', 'b', 'c')
  .to_list()
# -> [(1, 'a'), (2, 'b'), (3, 'c')]
````

{% example_playground_link(version = "0.15") %}
print (1, 2, 3)
  .zip ('a', 'b', 'c')
  .to_list()
# -> [(1, 'a'), (2, 'b'), (3, 'c')]

{% end %}
## IteratorOutput

A wrapper for a single item of iterator output.

This exists to allow functions like [`iterator.next`](#next) to return `null` to
indicate that the iterator has been exhausted, 
while also allowing `null` to appear in the iterator's output.

## IteratorOutput.get

````kototype
|IteratorOutput| -> Any
````

Returns the wrapped iterator output value.

### Example

````koto
x = 'abc'.next()
# -> IteratorOutput(a)
x.get()
# -> a
````

{% example_playground_link(version = "0.15") %}
print x = 'abc'.next()
# -> IteratorOutput(a)
print x.get()
# -> a

{% end %}
