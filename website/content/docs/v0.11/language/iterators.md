+++
title = "Iterators"
slug = "iterators"
weight = 9
+++

# Iterators

The entries of a container can be accessed in order via an Iterator,
created with the `.iter()` function.

The iterator yields values via `.next()`, until the end of the sequence is
reached and `null` is returned.

````koto
i = [10, 20].iter()
i.next()
# -> 10
i.next()
# -> 20
i.next()
# -> null
````

{% example_playground_link() %}
play.clear_output()

i = [10, 20].iter()
print i.next()
# -> 10
print i.next()
# -> 20
print i.next()
# -> null

{% end %}
Iterators can be *adapted* using adaptors from the
[`iterator` module](../../core/iterator).
Iterator adaptors will accept any iterable value (which includes all containers),
so it's not necessary to call `.iter()` first.

````koto
x = [1, 2, 3, 4, 5].keep |n| n > 3
x.next()
# -> 4
x.next()
# -> 5
x.next()
# -> null
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 3, 4, 5].keep |n| n > 3
print x.next()
# -> 4
print x.next()
# -> 5
print x.next()
# -> null

{% end %}
Iterators can be also be *consumed* using functions like
`.to_list()` and `.to_tuple()`.

````koto
[1, 2, 3]
  .each |n| n * 2
  .to_tuple()
# -> (2, 4, 6)

(11, 22, 33, 44)
  .keep |n| n % 2 == 0
  .to_list()
# -> [22, 44]
````

{% example_playground_link() %}
play.clear_output()

print [1, 2, 3]
  .each |n| n * 2
  .to_tuple()
# -> (2, 4, 6)

print (11, 22, 33, 44)
  .keep |n| n % 2 == 0
  .to_list()
# -> [22, 44]

{% end %}
