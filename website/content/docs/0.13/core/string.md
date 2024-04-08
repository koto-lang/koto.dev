+++
title = "string"
slug = "string"
+++

# string

## bytes

````kototype
|String| -> Iterator
````

Returns an iterator that yields a series of integers representing the bytes
contained in the string data.

### Example

````koto
'Hëy!'.bytes().to_tuple()
# -> (72, 195, 171, 121, 33)
````

{% example_playground_link(version = "0.13") %}
print 'Hëy!'.bytes().to_tuple()
# -> (72, 195, 171, 121, 33)

{% end %}
### See Also

* [`string.from_bytes`](#from-bytes)

## chars

````kototype
|String| -> Iterator
````

Returns an iterator that yields the string's characters as strings.

A 'character' in Koto is defined as being a 
[unicode grapheme cluster](https://www.unicode.org/glossary/#grapheme_cluster).

### Note

Note that this is the default iteration behaviour for a string, so calling
`'hello'.chars()` is equivalent to calling `iterator.iter('hello')`.

### Example

````koto
'Héllø! 👋'.chars().to_tuple()
# -> ('H', 'é', 'l', 'l', 'ø', '!', ' ', '👋')
````

{% example_playground_link(version = "0.13") %}
print 'Héllø! 👋'.chars().to_tuple()
# -> ('H', 'é', 'l', 'l', 'ø', '!', ' ', '👋')

{% end %}
### See Also

* [`string.char-indices`](#char-indices)

## char_indices

````kototype
|String| -> Iterator
````

Returns an iterator that yields the indices of each 
[grapheme cluster](https://www.unicode.org/glossary/#grapheme_cluster) in the string.

Each cluster is represented as a range, which can then be used to extract the
cluster from the string via indexing.

### Example

````koto
'Hi 👋'.char_indices().to_tuple()
# -> (0..1, 1..2, 2..3, 3..7)
````

{% example_playground_link(version = "0.13") %}
print 'Hi 👋'.char_indices().to_tuple()
# -> (0..1, 1..2, 2..3, 3..7)

{% end %}
### See Also

* [`string.chars`](#chars)

## contains

````kototype
|String, String| -> Bool
````

Returns `true` if the second provided string is a sub-string of the first.

### Example

````koto
'xyz'.contains 'abc'
# -> false

'xyz'.contains 'yz'
# -> true

'xyz'.contains 'xyz'
# -> true

'xyz'.contains ''
# -> true
````

{% example_playground_link(version = "0.13") %}
print 'xyz'.contains 'abc'
# -> false

print 'xyz'.contains 'yz'
# -> true

print 'xyz'.contains 'xyz'
# -> true

print 'xyz'.contains ''
# -> true

{% end %}
## ends_with

````kototype
|String, String| -> Bool
````

Returns `true` if the first string ends with the second string.

### Example

````koto
'abcdef'.ends_with 'def'
# -> true

'xyz'.ends_with 'abc'
# -> false
````

{% example_playground_link(version = "0.13") %}
print 'abcdef'.ends_with 'def'
# -> true

print 'xyz'.ends_with 'abc'
# -> false

{% end %}
## escape

````kototype
|String| -> String
````

Returns the string with characters replaced with escape codes.

For example, newlines get replaced with `\n`, tabs get replaced with `\t`.

### Example

````koto
'👋'.escape()
# -> \u{1f44b}
````

{% example_playground_link(version = "0.13") %}
print '👋'.escape()
# -> \u{1f44b}

{% end %}
## is_empty

````kototype
|String| -> Bool
````

Returns `true` if the string contains no characters.

### Example

````koto
'abcdef'.is_empty()
# -> false

''.is_empty()
# -> true
````

{% example_playground_link(version = "0.13") %}
print 'abcdef'.is_empty()
# -> false

print ''.is_empty()
# -> true

{% end %}
## from_bytes

````kototype
|Iterable| -> String
````

Returns a string containing the bytes that are produced by the input iterable.
The iterable output must contain only Numbers in the `0..=255` range.
The resulting sequence of bytes must contain UTF-8 data.

### Example

````koto
string.from_bytes (72, 195, 171, 121, 33)
# -> Hëy!
````

{% example_playground_link(version = "0.13") %}
print string.from_bytes (72, 195, 171, 121, 33)
# -> Hëy!

{% end %}
### See Also

* [`string.bytes`](#bytes)

## lines

````kototype
|String| -> Iterator
````

Returns an iterator that yields the lines contained in the input string.

### Note

Lines end with either `\r\n` or `\n`.

### Example

````koto
'foo\nbar\nbaz'.lines().to_tuple()
# -> ('foo', 'bar', 'baz')

'\n\n\n'.lines().to_tuple()
# -> ('', '', '')
````

{% example_playground_link(version = "0.13") %}
print 'foo\nbar\nbaz'.lines().to_tuple()
# -> ('foo', 'bar', 'baz')

print '\n\n\n'.lines().to_tuple()
# -> ('', '', '')

{% end %}
## replace

````kototype
|String, String, String| -> String
````

Returns a copy of the input string with all occurrences of the match string
replaced with an alternative string.

### Example

````koto
'10101'.replace '0', 'x'
# -> 1x1x1
````

{% example_playground_link(version = "0.13") %}
print '10101'.replace '0', 'x'
# -> 1x1x1

{% end %}
## split

````kototype
|String, String| -> Iterator
````

Returns an iterator that yields strings resulting from splitting the first
string wherever the second string is encountered.

````kototype
|String, |String| -> Bool| -> Iterator
````

Returns an iterator that yields strings resulting from splitting the input
string based on the result of calling a function. The function will be called
for each grapheme in the input string, and splits will occur when the function
returns true.

### Example

````koto
'a,b,c'.split(',').to_tuple()
# -> ('a', 'b', 'c')

'O_O'.split('O').to_tuple()
# -> ('', '_', '')

'x!y?z'.split(|c| c == '!' or c == '?').to_tuple()
# -> ('x', 'y', 'z')
````

{% example_playground_link(version = "0.13") %}
print 'a,b,c'.split(',').to_tuple()
# -> ('a', 'b', 'c')

print 'O_O'.split('O').to_tuple()
# -> ('', '_', '')

print 'x!y?z'.split(|c| c == '!' or c == '?').to_tuple()
# -> ('x', 'y', 'z')

{% end %}
## starts_with

````kototype
|String, String| -> Bool
````

Returns `true` if the first string starts with the second string.

### Example

````koto
'abcdef'.starts_with 'abc'
# -> true

'xyz'.starts_with 'abc'
# -> false
````

{% example_playground_link(version = "0.13") %}
print 'abcdef'.starts_with 'abc'
# -> true

print 'xyz'.starts_with 'abc'
# -> false

{% end %}
## to_lowercase

````kototype
|String| -> String
````

Returns a lowercase version of the input string.

### Example

````koto
'HÉLLÖ'.to_lowercase()
# -> héllö

'O_o'.to_lowercase()
# -> o_o
````

{% example_playground_link(version = "0.13") %}
print 'HÉLLÖ'.to_lowercase()
# -> héllö

print 'O_o'.to_lowercase()
# -> o_o

{% end %}
## to_number

````kototype
|String| -> Number
````

Returns the string converted into a number.

* `0x`, `0o`, and `0b` prefixes will cause the parsing to treat the input as
  containing a hexadecimal, octal, or binary number respectively.
* Otherwise the number is assumed to be base 10, and the presence of a decimal
  point will produce a float instead of an integer.

If a number can't be produced then `Null` is returned.

````kototype
|String, Integer| -> Integer
````

Returns the string converted into an integer given the specified base.

The base must be in the range `2..=36`, otherwise an error will be thrown.

If the string contains non-numerical digits then `Null` is returned.

### Example

````koto
'123'.to_number()
# -> 123

'-8.9'.to_number()
# -> -8.9

'0x7f'.to_number()
# -> 127

'0b10101'.to_number()
# -> 21

'2N9C'.to_number(36)
# -> 123456
````

{% example_playground_link(version = "0.13") %}
print '123'.to_number()
# -> 123

print '-8.9'.to_number()
# -> -8.9

print '0x7f'.to_number()
# -> 127

print '0b10101'.to_number()
# -> 21

print '2N9C'.to_number(36)
# -> 123456

{% end %}
## to_uppercase

````kototype
|String| -> String
````

Returns an uppercase version of the input string.

### Example

````koto
'héllö'.to_uppercase()
# -> HÉLLÖ

'O_o'.to_uppercase()
# -> O_O
````

{% example_playground_link(version = "0.13") %}
print 'héllö'.to_uppercase()
# -> HÉLLÖ

print 'O_o'.to_uppercase()
# -> O_O

{% end %}
## trim

````kototype
|String| -> String
````

Returns the string with whitespace at the start and end of the string trimmed.

### Example

````koto
'   x    '.trim()
# -> x

'     >'.trim()
# -> >
````

{% example_playground_link(version = "0.13") %}
print '   x    '.trim()
# -> x

print '     >'.trim()
# -> >

{% end %}
