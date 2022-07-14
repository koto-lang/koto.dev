+++
title = "Core Library"
slug = "core_library"
weight = 8
+++

# Core Library

Koto includes a [Core Library](../../core) of useful functions and values organized into `Map`s known as *modules*. 

````koto
string.size 'hello'
# -> 5

list.first [99, -1, 3]
# -> 99
````

{% example_playground_link() %}
play.clear_output()

print string.size 'hello'
# -> 5

print list.first [99, -1, 3]
# -> 99

{% end %}
Values in Koto automatically have their corresponding core library modules available via `.` access.

````koto
'xyz'.size()
# -> 3

['abc', 123].first()
# -> abc

(11 / 2).round()
# -> 6

{apples: 42, pears: 99}.contains_key 'apples'
# -> true
````

{% example_playground_link() %}
play.clear_output()

print 'xyz'.size()
# -> 3

print ['abc', 123].first()
# -> abc

print (11 / 2).round()
# -> 6

print {apples: 42, pears: 99}.contains_key 'apples'
# -> true

{% end %}
