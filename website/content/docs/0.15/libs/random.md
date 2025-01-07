+++
title = "random"
slug = "random"
+++

# random

Utilities for generating random values in Koto.

At the core of the module is the `Rng` type, which is a seedable random
number generator. Each thread has access to a generator with a randomly
selected seed, or unique generators can be created with [`random.generator`](#generator).

## bool

````kototype
|| -> Bool
````

Generates a random bool using the current thread's generator.

### Example

````koto
# Seed the thread Rng so that we get predictable results
random.seed 99

random.bool()
# -> false
random.bool()
# -> true
````

## generator

````kototype
|| -> Rng
````

Creates an [`Rng`](#rng) with a randomly generated seed.

````kototype
|Number| -> Rng
````

Creates an [`Rng`](#rng) with a specified seed.

### Example

````koto
rng = random.generator 99
rng.pick (1, 2, 3)
# -> 3
rng.bool()
# -> false
````

## number

````kototype
|| -> Number
````

Generates a random number using the current thread's generator.

The number will be a floating point value in the range from 0 up to but not
including 1.

### Example

````koto
# Seed the thread Rng so that we get predictable results
random.seed 123

# Print random floats up to 3 decimal places
print '{random.number():.3}'
# -> 0.853
print '{random.number():.3}'
# -> 0.168
````

## pick

````kototype
|Indexable| -> Any?
````

Selects a random value from the input using the current thread's generator.

* If the input is empty, then `null` will be returned.
* If the input is a map, then a tuple containing the key and value of a
  randomly selected entry will be returned.
* If the input is a range, then the result will be an integer within the given
  range.
* If the input is some other indexable type (like a list or tuple), 
  then a randomly selected element from the input will be returned. 

### Example

````koto
# Seed the thread Rng so that we get predictable results
random.seed -1

random.pick (123, -1, 99)
# -> 99
random.pick 10..20
# -> 14
random.pick {foo: 42, bar: 99, baz: 123}
# -> ('bar', 99)
random.pick []
# -> null
````

## seed

````kototype
|Number| -> Null
````

Seeds the current thread's generator so that it produces predictable results.

### Example

````koto
from iterator import generate
from random import pick, seed

# Returns a tuple containing three numbers from 1 to 10
pick_3 = || generate(3, || pick 1..=10).to_tuple()

seed 1
pick_3()
# -> (5, 3, 8)

seed 2
pick_3()
# -> (6, 9, 3)

seed 1
pick_3()
# -> (5, 3, 8)
````

## shuffle

````kototype
|Indexable| -> Any
````

Reorders the entries in a container so that they have a new randomly shuffled order,
and returns the container.

````koto
from random import seed, shuffle

x = [1, 2, 3, 4, 5]

seed 2
shuffle x
# -> [4, 1, 2, 3, 5]
shuffle x
# -> [5, 2, 4, 3, 1]

y = {a: 1, b: 2, c: 3}
shuffle y
# -> {b: 2, a: 1, c: 3}
shuffle y
# -> {a: 1, c: 3, b: 2}
````

## Rng

`Rng` is the `random` module's core random generator.

The ChaCha algorithm with 8 rounds from the `rand_chacha` crate is used to
generate random values. 
See the [implementation's docs](https://docs.rs/rand_chacha/latest/rand_chacha/struct.ChaCha8Rng.html) for more information.

## Rng.bool

See [random.bool](#bool).

## Rng.number

See [random.number](#number).

## Rng.pick

See [random.pick](#pick).

## Rng.shuffle

See [random.shuffle](#shuffle).

## Rng.seed

See [random.seed](#seed).