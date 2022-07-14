+++
title = "list"
slug = "list"
+++

# list

## clear

````kototype
|List| -> List
````

Clears the list by removing all of its elements, and returns the cleared list.

### Example

````koto
x = [1, 2, 3]
x.clear()
# -> []
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 3]
print x.clear()
# -> []

{% end %}
## contains

````kototype
|List, Value| -> Bool
````

Returns `true` if the list contains a value that matches the input value.

Matching is performed with the `==` equality operator.

### Example

````koto
[1, 'hello', (99, -1)].contains 'hello'
# -> true
````

{% example_playground_link() %}
play.clear_output()

print [1, 'hello', (99, -1)].contains 'hello'
# -> true

{% end %}
## copy

````kototype
|List| -> List
````

Makes a unique copy of the list data.

Note that this only copies the first level of data, so nested containers
will share their data with their counterparts in the copy. To make a copy where
any nested containers are also unique, use [`list.deep_copy`](#deep-copy).

### Example

````koto
x = [1, 2, 'hello']
y = x
y[0] = 'abc' # x and y share the same internal list data
x
# -> ['abc', 2, 'hello']

z = x.copy()
z[1] = -1 # z is a copy of x, so has unique internal data
x # x remains unchanged after the modificaton of z
# -> ['abc', 2, 'hello']
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 'hello']
y = x
y[0] = 'abc' # x and y share the same internal list data
print x
# -> ['abc', 2, 'hello']

z = x.copy()
z[1] = -1 # z is a copy of x, so has unique internal data
print x # x remains unchanged after the modificaton of z
# -> ['abc', 2, 'hello']

{% end %}
### See also

* [`list.deep_copy`](#deep-copy)

## deep_copy

````kototype
|List| -> List
````

Makes a unique *deep* copy of the list data.

This makes a unique copy of the list data, and then recursively makes deep
copies of any nested containers in the list.

If only the first level of data needs to be made unique, then use
[`list.copy`](#copy).

### Example

````koto
x = [[1, 2], [3, [4, 5]]]
y = x.deep_copy()
y[1][1] = 99
x # a deep copy has been made, so x is unaffected by the assignment to y
# -> [[1, 2], [3, [4, 5]]]
````

{% example_playground_link() %}
play.clear_output()

x = [[1, 2], [3, [4, 5]]]
y = x.deep_copy()
y[1][1] = 99
print x # a deep copy has been made, so x is unaffected by the assignment to y
# -> [[1, 2], [3, [4, 5]]]

{% end %}
### See also

* [`list.copy`](#copy)

## extend

````kototype
|List, Iterable| -> List
````

Extends the list with the output of the iterator, and returns the list.

### Example

````koto
x = [1, 2, 3]
x.extend 'abc'
# -> [1, 2, 3, 'a', 'b', 'c']
x.last()
# -> c
x.extend [10, 20, 30]
# -> [1, 2, 3, 'a', 'b', 'c', 10, 20, 30]
x.last()
# -> 30
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 3]
print x.extend 'abc'
# -> [1, 2, 3, 'a', 'b', 'c']
print x.last()
# -> c
print x.extend [10, 20, 30]
# -> [1, 2, 3, 'a', 'b', 'c', 10, 20, 30]
print x.last()
# -> 30

{% end %}
### See also

* [`list.push`](#push)

## fill

````kototype
|List, Value| -> List
````

Fills the list with copies of the provided value, and returns the list.

### Example

````koto
x = [1, 2, 3]
x.fill 99
# -> [99, 99, 99]
x
# -> [99, 99, 99]
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 3]
print x.fill 99
# -> [99, 99, 99]
print x
# -> [99, 99, 99]

{% end %}
## first

````kototype
|List| -> Value
````

Returns the first value in the list, or Null if the list is empty.

### Example

````koto
[99, -1, 42].first()
# -> 99

[].first()
# -> null
````

{% example_playground_link() %}
play.clear_output()

print [99, -1, 42].first()
# -> 99

print [].first()
# -> null

{% end %}
### See also

* [`list.get`](#get)
* [`list.last`](#last)

## get

````kototype
|List, Number| -> Value
````

````kototype
|List, Number, Value| -> Value
````

Gets the Nth value in the list.
If the list doesn't contain a value at that position then the provided default
value is returned. If no default value is provided then Null is returned.

### Example

````koto
x = [99, -1, 42]

x.get 1
# -> -1

x.get -1
# -> null

x.get 5, 123
# -> 123
````

{% example_playground_link() %}
play.clear_output()

x = [99, -1, 42]

print x.get 1
# -> -1

print x.get -1
# -> null

print x.get 5, 123
# -> 123

{% end %}
### See also

* [`list.first`](#first)
* [`list.last`](#last)

## insert

````kototype
|List, Number, Value| -> List
````

Inserts the value into the Nth position in the list, and returns the list.

An error is thrown if the position is negative or greater than the size of the
list.

### Example

````koto
x = [99, -1, 42]
x.insert 2, 'hello'
# -> [99, -1, 'hello', 42]
x
# -> [99, -1, 'hello', 42]
````

{% example_playground_link() %}
play.clear_output()

x = [99, -1, 42]
print x.insert 2, 'hello'
# -> [99, -1, 'hello', 42]
print x
# -> [99, -1, 'hello', 42]

{% end %}
### See also

* [`list.remove`](#remove)

## is_empty

````kototype
|List| -> Bool
````

Returns `true` if the list has a size of zero, and `false` otherwise.

### Example

````koto
[].is_empty()
# -> true

[1, 2, 3].is_empty()
# -> false
````

{% example_playground_link() %}
play.clear_output()

print [].is_empty()
# -> true

print [1, 2, 3].is_empty()
# -> false

{% end %}
## last

````kototype
|List| -> Value
````

Returns the last value in the list, or Null if the list is empty.

### Example

````koto
[99, -1, 42].last()
# -> 42

[].last()
# -> null
````

{% example_playground_link() %}
play.clear_output()

print [99, -1, 42].last()
# -> 42

print [].last()
# -> null

{% end %}
### See also

* [`list.first`](#first)
* [`list.get`](#get)

## pop

````kototype
|List| -> Value
````

Removes the last value from the list and returns it.

If the list is empty then Null is returned.

### Example

````koto
x = [99, -1, 42]
x.pop()
# -> 42

x
# -> [99, -1]

[].pop()
# -> null
````

{% example_playground_link() %}
play.clear_output()

x = [99, -1, 42]
print x.pop()
# -> 42

print x
# -> [99, -1]

print [].pop()
# -> null

{% end %}
### See also

* [`list.push`](#push)

## push

````kototype
|List, Value| -> Value
````

Adds the value to the end of the list, and returns the list.

### Example

````koto
x = [99, -1]
x.push 'hello'
# -> [99, -1, 'hello']
x
# -> [99, -1, 'hello']
````

{% example_playground_link() %}
play.clear_output()

x = [99, -1]
print x.push 'hello'
# -> [99, -1, 'hello']
print x
# -> [99, -1, 'hello']

{% end %}
### See also

* [`list.pop`](#pop)

## remove

````kototype
|List, Number| -> Value
````

Removes the value at the given position from the list and returns it.

Throws an error if the position isn't a valid index in the list.

### Example

````koto
[99, -1, 42].remove 1
# [99, 42]
````

{% example_playground_link() %}
play.clear_output()

[99, -1, 42].remove 1
# [99, 42]

{% end %}
### See also

* [`list.insert`](#insert)

## resize

````kototype
|List, Number| -> List
````

````kototype
|List, Number, Value| -> List
````

Grows or shrinks the list to the specified size, and returns the list.
If the new size is larger, then copies of the provided value (or Null if no
value is provided) are used to fill the new space.

### Example

````koto
x = [1, 2]
x.resize 4, 'x'
# -> [1, 2, 'x', 'x']

x.resize 3
# -> [1, 2, 'x']

x.resize 4
# -> [1, 2, 'x', null]
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2]
print x.resize 4, 'x'
# -> [1, 2, 'x', 'x']

print x.resize 3
# -> [1, 2, 'x']

print x.resize 4
# -> [1, 2, 'x', null]

{% end %}
## resize_with

````kototype
|List, Number, || -> Value| -> List
````

Grows or shrinks the list to the specified size, and returns the list.
If the new size is larger, then the provided function will be called repeatedly
to fill the remaining space, with the result of the function being added to the
end of the list.

### Example

````koto
new_entries = (5, 6, 7, 8).iter()
x = [1, 2]
x.resize_with 4, || new_entries.next()
# -> [1, 2, 5, 6]

x.resize_with 2, || new_entries.next()
# -> [1, 2]
````

{% example_playground_link() %}
play.clear_output()

new_entries = (5, 6, 7, 8).iter()
x = [1, 2]
print x.resize_with 4, || new_entries.next()
# -> [1, 2, 5, 6]

print x.resize_with 2, || new_entries.next()
# -> [1, 2]

{% end %}
## retain

````kototype
|List, Value| -> List
````

Retains matching values in the list (discarding values that don't match), and
returns the list.

If the test value is a function, then the function will be called with each of
the list's values, and if the function returns `true` then the value will be
retained, otherwise if the function returns `false` then the value will be
discarded.

If the test value is not a function, then the list's values will be compared
using the `==` equality operator, and then retained if they match.

### Example

````koto
x = (1..10).to_list()
x.retain |n| n < 5
# -> [1, 2, 3, 4]
x
# -> [1, 2, 3, 4]

x = [1, 3, 8, 3, 9, -1]
x.retain 3
# -> [3, 3]
x
# -> [3, 3]
````

{% example_playground_link() %}
play.clear_output()

x = (1..10).to_list()
print x.retain |n| n < 5
# -> [1, 2, 3, 4]
print x
# -> [1, 2, 3, 4]

x = [1, 3, 8, 3, 9, -1]
print x.retain 3
# -> [3, 3]
print x
# -> [3, 3]

{% end %}
## reverse

````kototype
|List| -> List
````

Reverses the order of the list's contents, and returns the list.

### Example

````koto
x = ['hello', -1, 99, 'world']
x.reverse()
# -> ['world', 99, -1, 'hello']
x
# -> ['world', 99, -1, 'hello']
````

{% example_playground_link() %}
play.clear_output()

x = ['hello', -1, 99, 'world']
print x.reverse()
# -> ['world', 99, -1, 'hello']
print x
# -> ['world', 99, -1, 'hello']

{% end %}
## size

````kototype
|List| -> Number
````

Returns the number of values contained in the list.

### Example

````koto
x = (1..=100).to_list()
x.size()
# -> 100

[].size()
# -> 0
````

{% example_playground_link() %}
play.clear_output()

x = (1..=100).to_list()
print x.size()
# -> 100

print [].size()
# -> 0

{% end %}
## sort

````kototype
|List| -> List
````

Sorts the list in place, and returns the list.

````kototype
|List, |Value| -> Value| -> List
````

Sorts the list in place, based on the output of calling a 'key' function for
each value, and returns the list. The function result is cached, so it's only
called once per value.

### Example

````koto
x = [1, -1, 99, 42]
x.sort()
# -> [-1, 1, 42, 99]
x
# -> [-1, 1, 42, 99]

x = ['bb', 'ccc', 'a']
x.sort string.size
# -> ['a', 'bb', 'ccc']
x
# -> ['a', 'bb', 'ccc']

x = [2, 1, 3]
x.sort |n| -n
# -> [3, 2, 1]
x
# -> [3, 2, 1]
````

{% example_playground_link() %}
play.clear_output()

x = [1, -1, 99, 42]
print x.sort()
# -> [-1, 1, 42, 99]
print x
# -> [-1, 1, 42, 99]

x = ['bb', 'ccc', 'a']
print x.sort string.size
# -> ['a', 'bb', 'ccc']
print x
# -> ['a', 'bb', 'ccc']

x = [2, 1, 3]
print x.sort |n| -n
# -> [3, 2, 1]
print x
# -> [3, 2, 1]

{% end %}
## swap

````kototype
|List, List| -> Null
````

Swaps the contents of the two input lists.

### Example

````koto
x = [1, 2, 3]
y = [7, 8, 9]
x.swap y

x
# -> [7, 8, 9]

y
# -> [1, 2, 3]
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 3]
y = [7, 8, 9]
x.swap y

print x
# -> [7, 8, 9]

print y
# -> [1, 2, 3]

{% end %}
## to_tuple

````kototype
|List| -> Tuple
````

Returns a copy of the list data as a tuple.

### Example

````koto
[1, 2, 3].to_tuple()
# -> (1, 2, 3)
````

{% example_playground_link() %}
play.clear_output()

print [1, 2, 3].to_tuple()
# -> (1, 2, 3)

{% end %}
## transform

````kototype
|List, |Value| -> Value| -> List
````

Transforms the list data by replacing each value with the result of calling the
provided function, and then returns the list.

### Example

````koto
x = ['aaa', 'bb', 'c']
x.transform string.size
# -> [3, 2, 1]
x
# -> [3, 2, 1]

x.transform |n| '{}'.format n
# -> ['3', '2', '1']
x
# -> ['3', '2', '1']
````

{% example_playground_link() %}
play.clear_output()

x = ['aaa', 'bb', 'c']
print x.transform string.size
# -> [3, 2, 1]
print x
# -> [3, 2, 1]

print x.transform |n| '{}'.format n
# -> ['3', '2', '1']
print x
# -> ['3', '2', '1']

{% end %}
## with_size

````kototype
|Number, Value| -> List
````

Returns a list containing `N` copies of a value.

### Example

````koto
list.with_size 5, '!'
# -> ['!', '!', '!', '!', '!']
````

{% example_playground_link() %}
play.clear_output()

print list.with_size 5, '!'
# -> ['!', '!', '!', '!', '!']

{% end %}
