+++
title = "koto"
slug = "koto"
+++

# koto

A collection of utilities for working with the Koto runtime.

## copy

````kototype
|value: Any| -> Any
````

Makes a copy of the provided value.

### Shared mutable data

For values that have shared mutable data (i.e., `List`, `Map`), unique copies of
the data will be made. Note that this only applies to the first level of data,
so nested containers will still share their data with their counterparts in the
original data. To make a copy where any nested containers are also unique,
use [`koto.deep_copy`](#deep-copy).

### Iterator copies

Copied iterators share the same underlying data as the original, but have a
unique iteration position, which is part of an iterator's shared state by
default.

If the iterator is a generator, some effort will be made to make the generator's
copy produce the same output as the original. However, this isn't guaranteed to
be successful. Specifically, the value stack of the copied virtual machine will
be scanned for iterators, and each iterator will have a copy made. Iterators
that may be used in other ways by the generator (such as being stored in
containers or function captures) won't be copied and will still have shared
state.

### Examples

````koto
# Copying a map
x = {foo: -1, bar: 99}
y = x
y.foo = 42
x.foo
#: 42

z = koto.copy x
z.bar = -1
x.bar # x.bar remains unmodified due to the copy
#: 99
````

{% example_playground_link(version = "0.16") %}
# Copying a map
x = {foo: -1, bar: 99}
y = x
y.foo = 42
print x.foo
#: 42

z = koto.copy x
z.bar = -1
print x.bar # x.bar remains unmodified due to the copy
#: 99

{% end %}

````koto
# Copying a list

x = (1..=10).iter()
y = x # y shares the same iteration position as x.
z = koto.copy x # z shares the same iteration data (the range 1..=10),
                # but has a unique iteration position.

x.next().get()
#: 1
x.next().get()
#: 2
y.next().get() # y shares x's iteration position.
#: 3
z.next().get() # z isn't impacted by the advancing of x and y.
#: 1
````

{% example_playground_link(version = "0.16") %}
# Copying a list

x = (1..=10).iter()
y = x # y shares the same iteration position as x.
z = koto.copy x # z shares the same iteration data (the range 1..=10),
                # but has a unique iteration position.

print x.next().get()
#: 1
print x.next().get()
#: 2
print y.next().get() # y shares x's iteration position.
#: 3
print z.next().get() # z isn't impacted by the advancing of x and y.
#: 1

{% end %}
### See also

* [`koto.deep_copy`](#deep-copy)

## deep_copy

````kototype
|value: Any| -> Any
````

Makes a unique *deep* copy of the value's data.

### Shared mutable data

This makes a unique copy of the value's data, and then recursively makes deep
copies of any nested containers in the value.

If only the first level of data needs to be made unique, then use
[`koto.copy`](#copy).

### Example

````koto
x = [[1, 2], [3, [4, 5]]]
y = koto.deep_copy x
y[1][1] = 99
x # a deep copy has been made, so x is unaffected by the assignment to y
#: [[1, 2], [3, [4, 5]]]
````

{% example_playground_link(version = "0.16") %}
x = [[1, 2], [3, [4, 5]]]
y = koto.deep_copy x
y[1][1] = 99
print x # a deep copy has been made, so x is unaffected by the assignment to y
#: [[1, 2], [3, [4, 5]]]

{% end %}
### See also

* [`koto.copy`](#copy)

## hash

````kototype
|value: Any| -> Number?
````

Returns the value's hash as an integer, or `null` if the value is not hashable.

### Example

````koto
from koto import hash

(hash 'hi') == (hash 'bye')
#: false

# Lists aren't hashable
hash [1, 2]
#: null

# Tuples are hashable if they only contain hashable values
(hash (1, 2)) == null
#: false
````

{% example_playground_link(version = "0.16") %}
from koto import hash

print (hash 'hi') == (hash 'bye')
#: false

# Lists aren't hashable
print hash [1, 2]
#: null

# Tuples are hashable if they only contain hashable values
print (hash (1, 2)) == null
#: false

{% end %}
## load

````kototype
|script: String| -> Chunk
````

Compiles the provided Koto `script` and returns a compiled `Chunk`.

Any compilation errors get thrown.

### Example

````koto
chunk = koto.load '1 + 2'
koto.run chunk
#: 3
````

{% example_playground_link(version = "0.16") %}
chunk = koto.load '1 + 2'
print koto.run chunk
#: 3

{% end %}
### See also

* [`koto.run`](#run)

## run

````kototype
|script: String| -> Any
````

Compiles and runs the provided Koto `script`, and returns the resulting value.

Any compilation or runtime errors get thrown.

````kototype
|Chunk| -> Any
````

Runs the compiled `Chunk`, and returns the resulting value.

Any runtime errors encountered during execution get thrown.

### Example

````koto
koto.run '[1, 2, 3, 4].sum()'
#: 10
````

{% example_playground_link(version = "0.16") %}
print koto.run '[1, 2, 3, 4].sum()'
#: 10

{% end %}
### See also

* [`koto.load`](#load)

## script_dir

````kototype
|| -> String?
````

Returns the path of the directory containing the current script, if available.

## script_path

````kototype
|| -> String?
````

Returns the path of the file containing the current script, if available.

## size

````kototype
|value: Any| -> Number
````

Returns the *size* of a value.

The size of a value is typically defined as the number of elements in a
container, with some notable exceptions:

* For strings, the size is the number of bytes in the string data.
* For ranges, the size is the number of integers in the range.
  * For non-inclusive ranges, this is equivalent to
    `range.end() - range.start()`.
  * For inclusive ranges, this is equivalent to
    `range.end() + 1 - range.start()`.
  * If the range is unbounded then an error will be thrown.
* An error will be thrown if the value doesn't have a defined size.

### Example

````koto
from koto import size

(size [1, 2, 3]), (size ())
#: (3, 0)

(size 'hello'), (size 'héllø'), (size '')
#: (5, 7, 0)

(size 10..20), (size 10..=20), (size 20..0)
#: (10, 11, 20)
````

{% example_playground_link(version = "0.16") %}
from koto import size

print (size [1, 2, 3]), (size ())
#: (3, 0)

print (size 'hello'), (size 'héllø'), (size '')
#: (5, 7, 0)

print (size 10..20), (size 10..=20), (size 20..0)
#: (10, 11, 20)

{% end %}
## type

````kototype
|value: Any| -> String
````

Returns the type of the input value as a String.

### Example

````koto
koto.type true
#: Bool

x = 42
koto.type x
#: Number

foo =
  @type: "Foo"
koto.type foo
#: Foo
````

{% example_playground_link(version = "0.16") %}
print koto.type true
#: Bool

x = 42
print koto.type x
#: Number

foo =
  @type: "Foo"
print koto.type foo
#: Foo

{% end %}
## unimplemented

````kototype
Unimplemented
````

An instance of `Unimplemented`, which should be thrown from [overridden arithmetic operators][guide-arithmetic] when the
operation isn't supported with the given input type.

### Example

````koto
foo = |n|
  data: n
  @type: 'Foo'
  @display: || 'Foo({self.data})'
  @+: |other|
    # Throw an `unimplemented` error if the rhs isn't a Foo
    match type other
      'Foo' then foo self.data + other.data
      else throw koto.unimplemented

bar = |n|
  data: n
  @type: 'Bar'
  @display: || 'Bar({self.data})'
  @r+: |other|

    match type other
      'Foo' or 'Bar' then bar other.data + self.data
      else throw koto.unimplemented

(foo 10) + (foo 20)
#: Foo(30)

(foo 2) + (bar 3)
#: Bar(5)
````

{% example_playground_link(version = "0.16") %}
foo = |n|
  data: n
  @type: 'Foo'
  @display: || 'Foo({self.data})'
  @+: |other|
    # Throw an `unimplemented` error if the rhs isn't a Foo
    match type other
      'Foo' then foo self.data + other.data
      else throw koto.unimplemented

bar = |n|
  data: n
  @type: 'Bar'
  @display: || 'Bar({self.data})'
  @r+: |other|

    match type other
      'Foo' or 'Bar' then bar other.data + self.data
      else throw koto.unimplemented

print (foo 10) + (foo 20)
#: Foo(30)

print (foo 2) + (bar 3)
#: Bar(5)

{% end %}


[guide-arithmetic]: ../../language/#arithmetic-operators