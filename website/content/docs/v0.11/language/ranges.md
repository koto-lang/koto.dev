+++
title = "Ranges"
slug = "ranges"
weight = 12
+++

# Ranges

Ranges of integers can be created with `..` or `..=`.

`..` creates a *non-inclusive* range, which defines a range from the start 
*up to but not including* the end of the range.

````koto
r = 10..20
# -> 10..20
r.start()
# -> 10
r.end()
# -> 20
r.contains 20
# -> false
````

{% example_playground_link() %}
play.clear_output()

print r = 10..20
# -> 10..20
print r.start()
# -> 10
print r.end()
# -> 20
print r.contains 20
# -> false

{% end %}
`..=` creates an *inclusive* range, which includes the end of the range.

````koto
r = 100..=200
# -> 100..201
r.contains 200
# -> true
````

{% example_playground_link() %}
play.clear_output()

print r = 100..=200
# -> 100..201
print r.contains 200
# -> true

{% end %}
Ranges are iterable, so can be used in for loops, and with the `iterator` module.

````koto
for x in 1..=3
  print x
# -> 1
# -> 2
# -> 3

(0..5).to_list()
# -> [0, 1, 2, 3, 4]
````

{% example_playground_link() %}
play.clear_output()

for x in 1..=3
  print x
# -> 1
# -> 2
# -> 3

print (0..5).to_list()
# -> [0, 1, 2, 3, 4]

{% end %}
