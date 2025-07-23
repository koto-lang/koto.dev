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
#: (72, 195, 171, 121, 33)
````

{% example_playground_link(version = "0.16") %}
print 'Hëy!'.bytes().to_tuple()
#: (72, 195, 171, 121, 33)

{% end %}
### See Also

* [`string.from_bytes`](#from-bytes)

## chars

````kototype
|String| -> Iterator
````

Returns an iterator that yields the string's characters as strings.

A 'character' in Koto is defined as being a
[unicode grapheme cluster][grapheme-cluster].

It's worth noting that is the default iteration behaviour for a string,
so calling `'hello'.chars()` is equivalent to calling `iterator.iter('hello')`.

### Example

````koto
'Héllø! 👋'.chars().to_tuple()
#: ('H', 'é', 'l', 'l', 'ø', '!', ' ', '👋')
````

{% example_playground_link(version = "0.16") %}
print 'Héllø! 👋'.chars().to_tuple()
#: ('H', 'é', 'l', 'l', 'ø', '!', ' ', '👋')

{% end %}
### See Also

* [`string.char_indices`](#char-indices)

## char_indices

````kototype
|String| -> Iterator
````

Returns an iterator that yields the indices of each
[grapheme cluster][grapheme-cluster] in the string.

Each cluster is represented as a range, which can then be used to extract the
cluster from the string via indexing.

### Example

````koto
s = 'Hi 👋'

indices = s.char_indices().to_tuple()
#: (0..1, 1..2, 2..3, 3..7)

s[indices[3]]
#: 👋
````

{% example_playground_link(version = "0.16") %}
s = 'Hi 👋'

print indices = s.char_indices().to_tuple()
#: (0..1, 1..2, 2..3, 3..7)

print s[indices[3]]
#: 👋

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
#: false

'xyz'.contains 'yz'
#: true

'xyz'.contains 'xyz'
#: true

'xyz'.contains ''
#: true
````

{% example_playground_link(version = "0.16") %}
print 'xyz'.contains 'abc'
#: false

print 'xyz'.contains 'yz'
#: true

print 'xyz'.contains 'xyz'
#: true

print 'xyz'.contains ''
#: true

{% end %}
## ends_with

````kototype
|String, String| -> Bool
````

Returns `true` if the first string ends with the second string.

### Example

````koto
'abcdef'.ends_with 'def'
#: true

'xyz'.ends_with 'abc'
#: false
````

{% example_playground_link(version = "0.16") %}
print 'abcdef'.ends_with 'def'
#: true

print 'xyz'.ends_with 'abc'
#: false

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
#: \u{1f44b}
````

{% example_playground_link(version = "0.16") %}
print '👋'.escape()
#: \u{1f44b}

{% end %}
## is_empty

````kototype
|String| -> Bool
````

Returns `true` if the string contains no characters.

### Example

````koto
'abcdef'.is_empty()
#: false

''.is_empty()
#: true
````

{% example_playground_link(version = "0.16") %}
print 'abcdef'.is_empty()
#: false

print ''.is_empty()
#: true

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
#: Hëy!
````

{% example_playground_link(version = "0.16") %}
print string.from_bytes (72, 195, 171, 121, 33)
#: Hëy!

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
#: ('foo', 'bar', 'baz')

'\n\n\n'.lines().to_tuple()
#: ('', '', '')
````

{% example_playground_link(version = "0.16") %}
print 'foo\nbar\nbaz'.lines().to_tuple()
#: ('foo', 'bar', 'baz')

print '\n\n\n'.lines().to_tuple()
#: ('', '', '')

{% end %}
## repeat

````kototype
|String, n: Number| -> String
````

Creates a new string by repeating the input `n` times.

### Example

````koto
'abc'.repeat 3
#: abcabcabc
````

{% example_playground_link(version = "0.16") %}
print 'abc'.repeat 3
#: abcabcabc

{% end %}
## replace

````kototype
|String, match: String, replacement: String| -> String
````

Returns a copy of the input string with all occurrences of the `match` string
replaced with a `replacement` string.

### Example

````koto
'10101'.replace '0', 'x'
#: 1x1x1
````

{% example_playground_link(version = "0.16") %}
print '10101'.replace '0', 'x'
#: 1x1x1

{% end %}
## split

````kototype
|String, match: String| -> Iterator
````

Returns an iterator that yields strings resulting from splitting the first
string wherever the `match` string is encountered.

````kototype
|String, match: |String| -> Bool| -> Iterator
````

Returns an iterator that yields strings resulting from splitting the input
string based on the result of calling a `match` function.

The `match` function will be called for each grapheme in the input string, and
splits will occur when the function returns true.

### Example

````koto
'a,b,c'.split(',').to_tuple()
#: ('a', 'b', 'c')

'O_O'.split('O').to_tuple()
#: ('', '_', '')

'x!y?z'.split(|c| c == '!' or c == '?').to_tuple()
#: ('x', 'y', 'z')
````

{% example_playground_link(version = "0.16") %}
print 'a,b,c'.split(',').to_tuple()
#: ('a', 'b', 'c')

print 'O_O'.split('O').to_tuple()
#: ('', '_', '')

print 'x!y?z'.split(|c| c == '!' or c == '?').to_tuple()
#: ('x', 'y', 'z')

{% end %}
## starts_with

````kototype
|String, match: String| -> Bool
````

Returns `true` if the first string starts with the `match` string.

### Example

````koto
'abcdef'.starts_with 'abc'
#: true

'xyz'.starts_with 'abc'
#: false
````

{% example_playground_link(version = "0.16") %}
print 'abcdef'.starts_with 'abc'
#: true

print 'xyz'.starts_with 'abc'
#: false

{% end %}
## strip_prefix

````kototype
|input: String, prefix: String| -> String?
````

Returns the input string with the given `prefix` removed,
or `null` if the input string doesn't start with `prefix`.

### Example

````koto
'abc: xyz'.strip_prefix 'abc: '
#: xyz

'xxxxx'.strip_prefix 'abc: '
#: null
````

{% example_playground_link(version = "0.16") %}
print 'abc: xyz'.strip_prefix 'abc: '
#: xyz

print 'xxxxx'.strip_prefix 'abc: '
#: null

{% end %}
### See Also

* [`string.strip_suffix`](#strip-suffix)
* [`string.trim_start`](#trim-start)

## strip_suffix

````kototype
|input: String, suffix: String| -> String?
````

Returns the input string with the given `suffix` removed,
or `null` if the input string doesn't end with `suffix`.

### Example

````koto
'abc: xyz'.strip_suffix ' xyz'
#: abc:

'xxxxx'.strip_suffix ' xyz'
#: null
````

{% example_playground_link(version = "0.16") %}
print 'abc: xyz'.strip_suffix ' xyz'
#: abc:

print 'xxxxx'.strip_suffix ' xyz'
#: null

{% end %}
### See Also

* [`string.strip_prefix`](#strip-prefix)
* [`string.trim_end`](#trim-end)

## to_lowercase

````kototype
|String| -> String
````

Returns a lowercase version of the input string.

### Example

````koto
'HÉLLÖ'.to_lowercase()
#: héllö

'O_o'.to_lowercase()
#: o_o
````

{% example_playground_link(version = "0.16") %}
print 'HÉLLÖ'.to_lowercase()
#: héllö

print 'O_o'.to_lowercase()
#: o_o

{% end %}
## to_number

````kototype
|String| -> Number?
````

Returns the string converted into a number.

* `0x`, `0o`, and `0b` prefixes will cause the parsing to treat the input as
  containing a hexadecimal, octal, or binary number respectively.
* Otherwise the number is assumed to be base 10, and the presence of a decimal
  point will produce a float instead of an integer.

If a number can't be produced then `null` is returned.

````kototype
|String, base: Number| -> Number?
````

Returns the string converted into a number given the specified `base`.

The base must be an integer in the range `2..=36`,
otherwise an error will be thrown.

If the string contains non-numerical digits then `null` is returned.

### Example

````koto
'123'.to_number()
#: 123

'-8.9'.to_number()
#: -8.9

'0x7f'.to_number()
#: 127

'0b10101'.to_number()
#: 21

'2N9C'.to_number(36)
#: 123456
````

{% example_playground_link(version = "0.16") %}
print '123'.to_number()
#: 123

print '-8.9'.to_number()
#: -8.9

print '0x7f'.to_number()
#: 127

print '0b10101'.to_number()
#: 21

print '2N9C'.to_number(36)
#: 123456

{% end %}
## to_uppercase

````kototype
|String| -> String
````

Returns an uppercase version of the input string.

### Example

````koto
'héllö'.to_uppercase()
#: HÉLLÖ

'O_o'.to_uppercase()
#: O_O
````

{% example_playground_link(version = "0.16") %}
print 'héllö'.to_uppercase()
#: HÉLLÖ

print 'O_o'.to_uppercase()
#: O_O

{% end %}
## trim

````kototype
|input: String| -> String
````

Returns a string with any whitespace removed from the start and end of the input.

````kototype
|input: String, pattern: String| -> String
````

Returns a string with all occurrences of the pattern removed from the start and end of the input.

### Example

````koto
'   x   '.trim()
#: x

'     !'.trim()
#: !

'----O_o----'.trim '-'
#: O_o

'abcabc!!!abcabc'.trim 'abc'
#: !!!
````

{% example_playground_link(version = "0.16") %}
print '   x   '.trim()
#: x

print '     !'.trim()
#: !

print '----O_o----'.trim '-'
#: O_o

print 'abcabc!!!abcabc'.trim 'abc'
#: !!!

{% end %}
### See Also

* [`string.trim_start`](#trim-start)
* [`string.trim_end`](#trim-end)

## trim_start

````kototype
|input: String| -> String
````

Returns a string with any whitespace removed from the start of the input.

````kototype
|input: String, pattern: String| -> String
````

Returns a string with all occurrences of the pattern removed from the start of the input.

### Example

````koto
trimmed = '   x   '.trim_start()
(trimmed,)
#: ('x   ')

'----O_o----'.trim_start '-'
#: O_o----

'abcabc!!!abcabc'.trim_start 'abc'
#: !!!abcabc
````

{% example_playground_link(version = "0.16") %}
trimmed = '   x   '.trim_start()
print (trimmed,)
#: ('x   ')

print '----O_o----'.trim_start '-'
#: O_o----

print 'abcabc!!!abcabc'.trim_start 'abc'
#: !!!abcabc

{% end %}
### See Also

* [`string.strip_prefix`](#strip-prefix)
* [`string.trim`](#trim)
* [`string.trim_end`](#trim-end)

## trim_end

````kototype
|input: String| -> String
````

Returns a string with any whitespace removed from the end of the input.

````kototype
|input: String, pattern: String| -> String
````

Returns a string with all occurrences of the pattern removed from the end of the input.

### Example

````koto
'   x   '.trim_end()
#:    x

'     !     '.trim_end()
#:      !

'----O_o----'.trim_end '-'
#: ----O_o

'abcabc!!!abcabc'.trim_end 'abc'
#: abcabc!!!
````

{% example_playground_link(version = "0.16") %}
print '   x   '.trim_end()
#:    x

print '     !     '.trim_end()
#:      !

print '----O_o----'.trim_end '-'
#: ----O_o

print 'abcabc!!!abcabc'.trim_end 'abc'
#: abcabc!!!

{% end %}
### See Also

* [`string.strip_suffix`](#strip-suffix)
* [`string.trim`](#trim)
* [`string.trim_start`](#trim-start)

[grapheme-cluster]: https://www.unicode.org/glossary/#grapheme_cluster