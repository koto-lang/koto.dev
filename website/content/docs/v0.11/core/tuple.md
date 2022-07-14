+++
title = "tuple"
slug = "tuple"
+++

# tuple

## contains

````kototype
|Tuple, Value| -> Bool
````

Returns `true` if the tuple contains a value that matches the input value.

Matching is performed with the `==` equality operator.

### Example

````koto
(1, "hello", [99, -1]).contains "hello"
# -> true

("goodbye", 123).contains "hello"
# -> false
````

{% example_playground_link() %}
play.clear_output()

print (1, "hello", [99, -1]).contains "hello"
# -> true

print ("goodbye", 123).contains "hello"
# -> false

{% end %}
## deep_copy

## first

````kototype
|Tuple| -> Value
````

Returns the first value in the tuple, or Null if the tuple is empty.

### Example

````koto
x = 99, -1, 42
x.first()
# -> 99

(,).first()
# -> null
````

{% example_playground_link() %}
play.clear_output()

x = 99, -1, 42
print x.first()
# -> 99

print (,).first()
# -> null

{% end %}
## get

````kototype
|Tuple, Number| -> Value
````

````kototype
|Tuple, Number, Value| -> Value
````

Gets the Nth value in the tuple.
If the tuple doesn't contain a value at that position then the provided default
value is returned. If no default value is provided then Null is returned.

### Example

````koto
x = 99, -1, 42

x.get 1
# -> -1

x.get -1
# -> null

x.get 5, "abc"
# -> abc
````

{% example_playground_link() %}
play.clear_output()

x = 99, -1, 42

print x.get 1
# -> -1

print x.get -1
# -> null

print x.get 5, "abc"
# -> abc

{% end %}
## last

````kototype
|Tuple| -> Value
````

Returns the last value in the tuple, or Null if the tuple is empty.

### Example

````koto
x = 99, -1, 42
x.last()
# -> 42

(,).last()
# -> null
````

{% example_playground_link() %}
play.clear_output()

x = 99, -1, 42
print x.last()
# -> 42

print (,).last()
# -> null

{% end %}
## size

````kototype
|Tuple| -> Number
````

Returns the number of values contained in the tuple.

### Example

````koto
x = (10, 20, 30, 40, 50)
x.size()
# -> 5
````

{% example_playground_link() %}
play.clear_output()

x = (10, 20, 30, 40, 50)
print x.size()
# -> 5

{% end %}
## sort_copy

````kototype
|Tuple| -> Tuple
````

Returns a sorted copy of the tuple.

### Example

````koto
x = (1, -1, 99, 42)
y = x.sort_copy()
y
# -> (-1, 1, 42, 99)

x # x remains untouched
# -> (1, -1, 99, 42)
````

{% example_playground_link() %}
play.clear_output()

x = (1, -1, 99, 42)
y = x.sort_copy()
print y
# -> (-1, 1, 42, 99)

print x # x remains untouched
# -> (1, -1, 99, 42)

{% end %}
## to_list

````kototype
|Tuple| -> List
````

Returns a copy of the tuple's data as a list.

### Example

````koto
(1, 2, 3).to_list()
# -> [1, 2, 3]
````

{% example_playground_link() %}
play.clear_output()

print (1, 2, 3).to_list()
# -> [1, 2, 3]

{% end %}
