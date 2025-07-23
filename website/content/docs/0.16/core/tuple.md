+++
title = "tuple"
slug = "tuple"
+++

# tuple

## contains

````kototype
|Tuple, value: Any| -> Bool
````

Returns `true` if the tuple contains a value that matches the input value.

Matching is performed with the `==` equality operator.

### Example

````koto
(1, "hello", [99, -1]).contains "hello"
#: true

("goodbye", 123).contains "hello"
#: false
````

{% example_playground_link(version = "0.16") %}
print (1, "hello", [99, -1]).contains "hello"
#: true

print ("goodbye", 123).contains "hello"
#: false

{% end %}
## first

````kototype
|Tuple| -> Any?
````

Returns the first value in the tuple, or `null` if the tuple is empty.

### Example

````koto
x = 99, -1, 42
x.first()
#: 99

().first()
#: null
````

{% example_playground_link(version = "0.16") %}
x = 99, -1, 42
print x.first()
#: 99

print ().first()
#: null

{% end %}
## get

````kototype
|Tuple, index: Number| -> Any?
````

````kototype
|Tuple, index: Number, default: Any| -> Any?
````

Gets the Nth value in the tuple.
If the tuple doesn't contain a value at that position then the provided `default`
value is returned. If no default value is provided then `null` is returned.

### Example

````koto
x = 99, -1, 42

x.get 1
#: -1

x.get -1
#: null

x.get 5, "abc"
#: abc
````

{% example_playground_link(version = "0.16") %}
x = 99, -1, 42

print x.get 1
#: -1

print x.get -1
#: null

print x.get 5, "abc"
#: abc

{% end %}
## is_empty

````kototype
|Tuple| -> Bool
````

Returns `true` if the tuple has a size of zero, and `false` otherwise.

### Example

````koto
().is_empty()
#: true

(1, 2, 3).is_empty()
#: false
````

{% example_playground_link(version = "0.16") %}
print ().is_empty()
#: true

print (1, 2, 3).is_empty()
#: false

{% end %}
## last

````kototype
|Tuple| -> Any?
````

Returns the last value in the tuple, or `null` if the tuple is empty.

### Example

````koto
x = 99, -1, 42
x.last()
#: 42

(,).last()
#: null
````

{% example_playground_link(version = "0.16") %}
x = 99, -1, 42
print x.last()
#: 42

print (,).last()
#: null

{% end %}
## sort_copy

````kototype
|Tuple| -> Tuple
````

Returns a sorted copy of the tuple.

````kototype
|List, key: |Any| -> Any| -> List
````

Returns a sorted copy of the tuple, based on the output of calling a `key`
function for each of the tuple's elements.

The key function's result is cached, so it's only called once per value.

### Example

````koto
x = (1, -1, 99, 42)
y = x.sort_copy()
y
#: (-1, 1, 42, 99)

x # x remains untouched
#: (1, -1, 99, 42)

# Sort in reverse order by using a key function
x.sort_copy |n| -n
#: (99, 42, 1, -1)
````

{% example_playground_link(version = "0.16") %}
x = (1, -1, 99, 42)
y = x.sort_copy()
print y
#: (-1, 1, 42, 99)

print x # x remains untouched
#: (1, -1, 99, 42)

# Sort in reverse order by using a key function
print x.sort_copy |n| -n
#: (99, 42, 1, -1)

{% end %}
## to_list

````kototype
|Tuple| -> List
````

Returns a copy of the tuple's data as a list.

### Example

````koto
(1, 2, 3).to_list()
#: [1, 2, 3]
````

{% example_playground_link(version = "0.16") %}
print (1, 2, 3).to_list()
#: [1, 2, 3]

{% end %}
