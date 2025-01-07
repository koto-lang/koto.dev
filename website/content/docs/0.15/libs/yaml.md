+++
title = "yaml"
slug = "yaml"
+++

# yaml

[YAML](https://yaml.org) support for Koto.

## from_string

````kototype
|String| -> Any
````

Deserializes a string containing YAML data, returning a structured Koto value.

### Example

````koto
data = r'
string: O_o

nested:
  number: -1.2

entries:
- foo: bar
- foo: baz
'

result = yaml.from_string data
result.string
# -> O_o
result.nested.number
# -> -1.2
result.entries[0].foo
# -> bar
result.entries[1].foo
# -> baz
````

## to_string

````kototype
|Any| -> String
````

Returns a string containing the input value serialized as YAML data.

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

yaml.to_string data
# -> string: '>_>'
# -> nested:
# ->   number: 99
# -> entries:
# -> - foo: bar
# -> - foo: baz
# -> 
````