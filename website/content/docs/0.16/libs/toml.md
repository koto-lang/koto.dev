+++
title = "toml"
slug = "toml"
+++

# toml

[TOML](https://toml.io) support for Koto.

## from_string

````kototype
|String| -> Any
````

Deserializes a string containing TOML data, returning a structured Koto value.

### Example

````koto
data = r"
string = 'O_o'

[nested]
number = -1.2

[[entries]]
foo = 'bar'

[[entries]]
foo = 'baz'
"

result = toml.from_string data
result.string
#: O_o
result.nested.number
#: -1.2
result.entries[0].foo
#: bar
result.entries[1].foo
#: baz
````

## to_string

````kototype
|Any| -> String
````

Returns a string containing the input value serialized as TOML data.

### Example

````koto
data =
  string: '>_>'
  nested:
    number: 99
  entries: (
    {foo: 'bar'},
    {foo: 'baz'},
  )

toml.to_string data
#: string = ">_>"
check!
#: [nested]
#: number = 99
check!
#: [[entries]]
#: foo = "bar"
check!
#: [[entries]]
#: foo = "baz"
check!
````