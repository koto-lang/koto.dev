+++
title = "Loops"
slug = "loops"
weight = 11
+++

# Loops

## for

Iterable values can be iterated over with `for` loops.

````koto
for n in [10, 20, 30]
  print n
# -> 10
# -> 20
# -> 30
````

{% example_playground_link() %}
play.clear_output()

for n in [10, 20, 30]
  print n
# -> 10
# -> 20
# -> 30

{% end %}
Loops can be stopped early with `break`.

````koto
x = for n in (11, 22, 33, 44, 55)
  if n > 30 
    break n
x
# -> 33
````

{% example_playground_link() %}
play.clear_output()

x = for n in (11, 22, 33, 44, 55)
  if n > 30 
    break n
print x
# -> 33

{% end %}
`continue` can be used to skip ahead to the next iteration of the loop.

````koto
for n in (-2, -1, 1, 2)
  if n < 0
    continue
  print n
# -> 1
# -> 2
````

{% example_playground_link() %}
play.clear_output()

for n in (-2, -1, 1, 2)
  if n < 0
    continue
  print n
# -> 1
# -> 2

{% end %}
## while

`while` loops continue to repeat *while* a condition is true.

````koto
x = 0
while x < 5
  x += 1
x
# -> 5
````

{% example_playground_link() %}
play.clear_output()

x = 0
while x < 5
  x += 1
print x
# -> 5

{% end %}
## until

`until` loops continue to repeat *until* a condition is true.

````koto
z = [1, 2, 3]
until z.is_empty()
  print z.pop()
# -> 3
# -> 2
# -> 1
````

{% example_playground_link() %}
play.clear_output()

z = [1, 2, 3]
until z.is_empty()
  print z.pop()
# -> 3
# -> 2
# -> 1

{% end %}
## loop

`loop` creates a loop that will repeat indefinitely.

````koto
x = 0
y = loop
  x += 1
  if x > 4
    break x
y
# -> 5
````

{% example_playground_link() %}
play.clear_output()

x = 0
y = loop
  x += 1
  if x > 4
    break x
print y
# -> 5

{% end %}
