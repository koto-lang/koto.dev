+++
title = "Generators"
slug = "generators"
weight = 13
+++

# Generators

Custom iterators can be made with `generator functions`, 
which are any functions that contain a `yield` expression. 

````koto
f = ||
  yield 1
  yield 2
  yield 3

x = f()
x.next()
# -> 1
x.next()
# -> 2
x.next()
# -> 3
x.next()
# -> null
````

{% example_playground_link() %}
play.clear_output()

f = ||
  yield 1
  yield 2
  yield 3

x = f()
print x.next()
# -> 1
print x.next()
# -> 2
print x.next()
# -> 3
print x.next()
# -> null

{% end %}
Generator functions can be called with arguments like any other function, 
and their resulting generators have access to the `iterator` module.

````koto
my_generator = |x|
  for y in 1..=3
    yield x + y 

my_generator(0).to_list()
# -> [1, 2, 3]
my_generator(10).to_tuple()
# -> (11, 12, 13)
````

{% example_playground_link() %}
play.clear_output()

my_generator = |x|
  for y in 1..=3
    yield x + y 

print my_generator(0).to_list()
# -> [1, 2, 3]
print my_generator(10).to_tuple()
# -> (11, 12, 13)

{% end %}
A generator that takes an iterator as an argument acts an
iterator adaptor. 

Inserting it into the `iterator` module makes it available
in any iterator chain.

````koto
iterator.every_other = |iter|
  n = 0
  loop
    match iter.next()
      null then 
        return
      value if n % 2 == 0 then 
        yield value
    n += 1

(1..=5)
  .each |n| n * 10
  .every_other()
  .to_list()
# -> [10, 30, 50]
````

{% example_playground_link() %}
play.clear_output()

iterator.every_other = |iter|
  n = 0
  loop
    match iter.next()
      null then 
        return
      value if n % 2 == 0 then 
        yield value
    n += 1

print (1..=5)
  .each |n| n * 10
  .every_other()
  .to_list()
# -> [10, 30, 50]

{% end %}
