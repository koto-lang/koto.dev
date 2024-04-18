+++
title = "regex"
slug = "regex"
+++

# regex

[Regular expression](https://en.wikipedia.org/wiki/Regular_expression) utilities for Koto.

## new

````kototype
|String| -> Regex
````

Creates a new [Regex](#regex-1) from the given string.

## Regex

The core regular expression type used by the `regex` module.

The `regex` module is a wrapper for the [`regex`](https://github.com/rust-lang/regex) crate, 
please see [its documentation](https://docs.rs/regex/latest/regex/) for more information, 
including a guide to the [supported syntax](https://docs.rs/regex/latest/regex/#syntax).

## Regex.is_match

````kototype
|Regex, String| -> Bool
````

Returns `true` if the given string matches against the regular expression.

### Example

````koto
r = regex.new r'x\d\d'
r.is_match 'x42'
# -> true
r.is_match 'z99'
# -> false
````

## Regex.find

````kototype
|Regex, String| -> Match
````

If the given string matches the regular expression, then an instance of
[`Match`](#match) is returned that allows the first matched region in the string
to be inspected. If no matches are found then `null` is returned.

### Example

````koto
# Make a regex that will match against any group of 2 or 3 a-z characters
r = regex.new r'[a-z]{2,3}'
found = r.find 'a b xyz jk mno'
found.text(), found.range()
# -> ('xyz', 4..7)

r.find '12345'
# -> null
````

## Regex.find_all

````kototype
|Regex, String| -> Matches 
````

If the given string matches the regular expression, then an instance of
[`Matches`](#matches) is returned that allows all matches in the input to be
inspected. If no matches are found then `null` is returned.

### Example

````koto
# Make a regex that will match against any group of 2 or 3 a-z characters
r = regex.new r'[a-z]{2,3}'
matches = r.find_all('a bc def gh')
for found in matches
  print found.text(), found.range()
# -> ('bc', 2..4)
# -> ('def', 5..8)
# -> ('gh', 9..11)
````

## Regex.captures

````kototype
|Regex, String| -> Map
````

If the given string matches the regular expression, then a map is returned
containing the first match found, along with matches for any capture groups.
If no matches are found then `null` is returned.

Captured groups are entered in the map with their indices as the key, 
and if the group is named then it's also inserted with the name.
The first entry in the map (index 0) contains the entire match, with subsequent
captures starting at index 1.

### Example

````koto
# Make a regex that will match against two words inside <> braces
r = regex.new r'<(?<group_a>\S+)\s+(\S+)>'
captures = r.captures '!!! <Hello, World!> ???'

# Entry 0 contains the complete match
captures.get(0).text()
# -> <Hello, World!>

# Named captured groups use the name as the lookup key
captures.group_a.text()
# -> Hello,

# Groups without names use their group index as the lookup key
captures.get(2).text()
# -> World!

# Named capture groups are also available by index
group_name, capture = captures.get_index(1)
group_name, capture.text()
# -> ('group_a', 'Hello,')
````

## Regex.replace_all

````kototype
|Regex, input: String, replacement: String| -> String
````

Returns a string with each match in the input replaced using rules defined in
the replacement string.

````koto
# Make a regex that will match against two words inside <> braces
r = regex.new r'<(?<a>\S+)\s+(?<b>\S+)>'
r.replace_all '!!! <Replace Me> !!!', '>_>'
# -> !!! >_> !!!

# Capture groups can be referred to in the replacement string
r.replace_all '!!! <AAA BBB> !!!', '[$a$b $a$b]'
# -> !!! [AAABBB AAABBB] !!!
````

## Matches

`Matches` is an iterator that outputs a [`Match`](#match) for each match 
resulting from a call to [`Regex.find_all`](#regex-find-all).

## Match

`Match` is a type produced from calls to search functions like
[`Regex.find`](#regex-find) or [`Regex.captures`](#regex-captures) that provides 
access to the matched region of the input string, 
along with the matched region's indices.

## Match.text

````kototype
|Match| -> String
````

Returns the matched region of the input string.

### Example

````koto
m = regex.new(r'x\d\d').find 'abc def x99 123'
m.text()
# -> x99
````

## Match.range

````kototype
|Match| -> Range
````

Returns the indices of the matched region in the input string.

### Example

````koto
m = regex.new(r'x\d\d').find 'abc def x99 123'
m.range()
# -> 8..11
````