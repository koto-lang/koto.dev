+++
title = "Errors"
slug = "errors"
weight = 15
+++

# Errors

`try`, `catch`, and `finally` can be used to catch any errors thrown by the Koto runtime.

````koto
x = [1, 2, 3]
try
  print x[100]
catch error 
  print "Caught an error"
finally
  print "...and finally"
# -> Caught an error
# -> ...and finally
````

{% example_playground_link() %}
play.clear_output()

x = [1, 2, 3]
try
  print x[100]
catch error 
  print "Caught an error"
finally
  print "...and finally"
# -> Caught an error
# -> ...and finally

{% end %}
`throw` can be used to throw an error from within a Koto script.

````koto
f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: $error"
# -> Caught an error: !Error!
````

{% example_playground_link() %}
play.clear_output()

f = || throw "!Error!"

try
  f()
catch error
  print "Caught an error: $error"
# -> Caught an error: !Error!

{% end %}
