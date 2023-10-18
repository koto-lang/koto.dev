+++
title = "string"
slug = "string"
+++

# string

## bytes

````kototype
|String| -> Iterator
````

Returns an iterator that yields a series of Numbers representing the bytes
contained in the string data.

### Example

````koto
'Hëy!'.bytes().to_tuple()
# -> (72, 195, 171, 121, 33)
````

{% example_playground_link() %}
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

### Note

A 'character' in Koto is defined as a grapheme, so `.chars()` iterates over the
string's grapheme clusters.

### Note

Note that this is the default iteration behaviour for a string, so calling
`.chars()` on a string is equivalent to calling `iterator.iter()`.

### Example

````koto
'Héllø! 👋'.chars().to_tuple()
# -> ('H', 'é', 'l', 'l', 'ø', '!', ' ', '👋')
````

{% example_playground_link() %}
print 'Héllø! 👋'.chars().to_tuple()
# -> ('H', 'é', 'l', 'l', 'ø', '!', ' ', '👋')

{% end %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
print '👋'.escape()
# -> \u{1f44b}

{% end %}
## format

````kototype
|String, Value...| -> String
````

Returns a formatted string, with the arguments being assigned to
`{}` placeholders in the format string.

### Formatting Syntax

The syntax for format strings in Koto is similar to
[Rust's formatting syntax](https://doc.rust-lang.org/std/fmt/).

#### Placeholders

* `{}`
  * Takes the next value from the list of arguments, starting with the first.
  * Subsequent `{}` placeholders will take following values.
* `{0}, {1}, {2}, ...`
  * Takes the value at the specified index.
* `{x}, {name}, {id}`
  * Takes values by name from a Map.
    * The Map is expected to be the first argument after the format string.

`{` characters can be included in the output string by escaping them with
another `{`, e.g. `'{{}}'.format()` will output `'{}'`.

#### Formatting modifiers

Modifiers can be provided after a `:` separator in the format string.

##### Minimum width, fill, and alignment

A minimum width can be specified, ensuring that the formatted value takes up at
least that many characters, e.g. `'x{:4}x'.format 'ab'` will output `xab  x`.

The minimum width can be prefixed with an alignment modifier:

* `<` - left-aligned
* `^` - centered
* `>` - right-aligned

e.g. `'x{:>4}x'.format 'ab'` will output `x  abx`.

Values are left-aligned by default, except for numbers which are right-aligned
by default, e.g. `'x{:4}x'.format 1.2` will output `x 1.2x`.

The alignment modifier can be prefixed with a character which will be used to
fill any empty space in the formatted string (the default character being ` `).
e.g. `'{:x^8}'.format 1234` will output `xx1234xx`.

##### Maximum width / Precision

A maximum width can be specified following a `.` character,
e.g. `'{:.2}'.format abcd'` will output `ab`.

For numbers this will define the number of decimal places that should be
displayed.

Combining a maximum width with a minimum width is allowed, with the minimum
coming before the maximum in the format string,
e.g. `'x{:4.2}x'.format 'abcd'` will output `xab  x`.

### Example

````koto
'{}, {}!'.format 'Hello', 'World'
# -> Hello, World!

'{0}-{1}-{0}'.format 99, 'xxx'
# -> 99-xxx-99

'{foo} {bar}'.format {foo: 42, bar: true}
# -> 42 true

'{:.2}'.format 1/3
# -> 0.33

'{:-^8.2}'.format 2/3
# -> --0.67--

'foo = {foo:8.3}'.format {foo: 42}
# -> foo =   42.000
````

{% example_playground_link() %}
print '{}, {}!'.format 'Hello', 'World'
# -> Hello, World!

print '{0}-{1}-{0}'.format 99, 'xxx'
# -> 99-xxx-99

print '{foo} {bar}'.format {foo: 42, bar: true}
# -> 42 true

print '{:.2}'.format 1/3
# -> 0.33

print '{:-^8.2}'.format 2/3
# -> --0.67--

print 'foo = {foo:8.3}'.format {foo: 42}
# -> foo =   42.000

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

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
print '10101'.replace '0', 'x'
# -> 1x1x1

{% end %}
## size

````kototype
|String| -> Number
````

Returns the number of graphemes in the string.

### Note

Equivalent to calling `.chars().count()`.

### Example

````koto
''.size()
# -> 0

'abcdef'.size()
# -> 6

'🥳👋😁'.size()
# -> 3
````

{% example_playground_link() %}
print ''.size()
# -> 0

print 'abcdef'.size()
# -> 6

print '🥳👋😁'.size()
# -> 3

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

{% example_playground_link() %}
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

{% example_playground_link() %}
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

{% example_playground_link() %}
print 'HÉLLÖ'.to_lowercase()
# -> héllö

print 'O_o'.to_lowercase()
# -> o_o

{% end %}
## to_number

````kototype
|String| -> Number
````

Returns the string parsed as a number.

### Example

````koto
'123'.to_number()
# -> 123

'-8.9'.to_number()
# -> -8.9
````

{% example_playground_link() %}
print '123'.to_number()
# -> 123

print '-8.9'.to_number()
# -> -8.9

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

{% example_playground_link() %}
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

{% example_playground_link() %}
print '   x    '.trim()
# -> x

print '     >'.trim()
# -> >

{% end %}
