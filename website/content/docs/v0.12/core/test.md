+++
title = "test"
slug = "test"
+++

# test

A collection of utilities for writing tests.

## assert

````kototype
|Bool| -> Null
````

Throws a runtime error if the argument if false.

### Example

````koto
# This assertion will pass, and no error will be thrown
assert 1 < 2

# This assertion will fail and throw an error
try 
  assert 1 > 2
catch error
  print error
````

{% example_playground_link() %}
# This assertion will pass, and no error will be thrown
assert 1 < 2

# This assertion will fail and throw an error
try 
  assert 1 > 2
catch error
  print error

{% end %}
## assert_eq

````kototype
|Value, Value| -> Null
````

Checks the two input values for equality and throws an error if they're not
equal.

### Example

````koto
# This assertion will pass, and no error will be thrown
assert_eq 1 + 1, 2

# This assertion will fail and throw an error
try 
  assert_eq 2 + 2, 5
catch error
  print error
````

{% example_playground_link() %}
# This assertion will pass, and no error will be thrown
assert_eq 1 + 1, 2

# This assertion will fail and throw an error
try 
  assert_eq 2 + 2, 5
catch error
  print error

{% end %}
## assert_ne

````kototype
|Value, Value| -> Null
````

Checks the two input values for inequality and throws an error if they're equal.

### Example

````koto
# This assertion will pass, and no error will be thrown
assert_ne 1 + 1, 3

# This assertion will fail and throw an error
try
  assert_ne 2 + 2, 4
catch error
  print error
````

{% example_playground_link() %}
# This assertion will pass, and no error will be thrown
assert_ne 1 + 1, 3

# This assertion will fail and throw an error
try
  assert_ne 2 + 2, 4
catch error
  print error

{% end %}
## assert_near

````kototype
|Number, Number| -> Null
````

````kototype
|Number, Number, Number| -> Null
````

Checks that the two input numbers are equal, within an allowed margin of error.

This is useful when testing floating-point operations, where the result can be
close to a target with some acceptable imprecision.

The margin of error is optional, defaulting to `1.0e-12`.

### Example

````koto
allowed_error = 0.01
# This assertion will pass, and no error will be thrown
assert_near 1.3, 1.301, allowed_error

# This assertion will fail and throw an error
try
  assert_near 1.3, 1.32, allowed_error
catch error
  print error
# error: Assertion failed, '1.3' and '1.32' are not within 0.01 of each other

# The allowed margin of error is optional, defaulting to a very small value
assert_near 1 % 0.2, 0.2
````

{% example_playground_link() %}
allowed_error = 0.01
# This assertion will pass, and no error will be thrown
assert_near 1.3, 1.301, allowed_error

# This assertion will fail and throw an error
try
  assert_near 1.3, 1.32, allowed_error
catch error
  print error
# error: Assertion failed, '1.3' and '1.32' are not within 0.01 of each other

# The allowed margin of error is optional, defaulting to a very small value
assert_near 1 % 0.2, 0.2

{% end %}
## run_tests

````kototype
|Map| -> Null
````

Runs the tests contained in the map.

### Example

````koto
my_tests =
  @pre_test: || self.test_data = 1, 2, 3
  @post_test: || self.test_data = null

  @test data_size: || assert_eq self.test_data.size(), 3
  @test failure: || assert_eq self.test_data.size(), 0

try
  test.run_tests my_tests
catch error
  print "An error occurred while running my_tests:\n  {}", error
````

{% example_playground_link() %}
my_tests =
  @pre_test: || self.test_data = 1, 2, 3
  @post_test: || self.test_data = null

  @test data_size: || assert_eq self.test_data.size(), 3
  @test failure: || assert_eq self.test_data.size(), 0

try
  test.run_tests my_tests
catch error
  print "An error occurred while running my_tests:\n  {}", error

{% end %}
