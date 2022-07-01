+++
title = "Koto"
template = "index.html"
+++

# Welcome to Koto

```koto
# Numbers
x = 1 + 2.5 + 100.sqrt()
assert_eq x, 13.5

# Strings
name = "Koto"
print "Hello, $name!"

# Functions
square = |n| n * n
print "8 squared is ${square 8}"

add_squares = |a, b| (square a) + (square b)
assert_eq (add_squares 2, 4), 20

# Iterators, ranges, and lists
fizz_buzz = (1..100)
  .keep |n| (10..=15).contains n
  .each |n|
    match n % 3, n % 5
      0, 0 then "Fizz Buzz"
      0, _ then "Fizz"
      _, 0 then "Buzz"
      else n
  .to_list()
assert_eq
  fizz_buzz,
  ["Buzz", 11, "Fizz", 13, 14, "Fizz Buzz"]
```
