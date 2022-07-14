+++
title = "Maps"
slug = "maps"
weight = 7
+++

# Maps

Maps are Koto's associative containers, containing a series of key/value entries.

They can be declared with `{}` braces, or without using indented blocks.

With braces:

````koto
m = {apples: 42, oranges: 99, lemons: 63}
m.oranges
# -> 99
````

{% example_playground_link() %}
play.clear_output()

m = {apples: 42, oranges: 99, lemons: 63}
print m.oranges
# -> 99

{% end %}
...and as an indented block:

````koto
m = 
  apples: 42
  oranges: 99
  lemons: 63
m.apples
# -> 42
````

{% example_playground_link() %}
play.clear_output()

m = 
  apples: 42
  oranges: 99
  lemons: 63
print m.apples
# -> 42

{% end %}
Nested Maps can be declared with additional indentation:

````koto
m =
  hello:
    world: 99
    everybody: 123
    to:
      you: -1
m.hello.world
# -> 99
m.hello.to.you
# -> -1
````

{% example_playground_link() %}
play.clear_output()

m =
  hello:
    world: 99
    everybody: 123
    to:
      you: -1
print m.hello.world
# -> 99
print m.hello.to.you
# -> -1

{% end %}
Once a Map has been created, its data is shared between instances of the Map.

````koto
a = {foo: 99, bar: -1}
z = a
z.foo = 'Hi!'
a.foo
# -> Hi!
````

{% example_playground_link() %}
play.clear_output()

a = {foo: 99, bar: -1}
z = a
z.foo = 'Hi!'
print a.foo
# -> Hi!

{% end %}
Any value type can be stored in Maps, including Functions.

````koto
m = 
  hello: |name| 'Hello, $name!'
  bye: |name| 'Bye, $name!'

m.hello 'World'
# -> Hello, World!
m.bye 'Friend'
# -> Bye, Friend!
````

{% example_playground_link() %}
play.clear_output()

m = 
  hello: |name| 'Hello, $name!'
  bye: |name| 'Bye, $name!'

print m.hello 'World'
# -> Hello, World!
print m.bye 'Friend'
# -> Bye, Friend!

{% end %}
When the first argument in a Map's function is `self`,
then `self` will automatically be assigned as an instance of the Map that the Function's contained in.

````koto
m = 
  name: 'World'
  hello: |self| 'Hello, ${self.name}!'

m.hello()
# -> Hello, World!

m.name = 'Friend'
m.hello()
# -> Hello, Friend!
````

{% example_playground_link() %}
play.clear_output()

m = 
  name: 'World'
  hello: |self| 'Hello, ${self.name}!'

print m.hello()
# -> Hello, World!

m.name = 'Friend'
print m.hello()
# -> Hello, Friend!

{% end %}
