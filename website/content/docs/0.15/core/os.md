+++
title = "os"
slug = "os"
+++

# os

A collection of utilities for working with the operating system.

## command

````kototype
|program: String| -> Command
````

Creates a new [Command](#command-1), which supports executing external programs in separate processes.

Builder methods allow configuration of properties like [command arguments](#command-args) or [environment variables](#command-env) before [spawning](#command-spawn) the program in a new process.

### Example

````koto
os.command('ls')
  .args('-al', '/tmp')
  .wait_for_output()
  .stdout()
# -> ...
````

{% example_playground_link(version = "0.15") %}
print os.command('ls')
  .args('-al', '/tmp')
  .wait_for_output()
  .stdout()
# -> ...

{% end %}
## name

````kototype
|| -> String
````

Returns a string containing the name of the current operating system, e.g.
"linux", "macos", "windows", etc.

## process_id

````kototype
|| -> Number
````

Returns the ID associated with the current process.

## start_timer

````kototype
|| -> Timer
````

Returns a timer that can be used to measure how much time has passed while a
script is running.

### Example

````koto
t = os.start_timer()

# ...after some time...
print "Time taken: ${t.elapsed()}s"

t2 = os.start_timer()
print "Seconds between then and now: ${t2 - t}"
````

{% example_playground_link(version = "0.15") %}
t = os.start_timer()

# ...after some time...
print "Time taken: ${t.elapsed()}s"

t2 = os.start_timer()
print "Seconds between then and now: ${t2 - t}"

{% end %}
## time

````kototype
|| -> DateTime
````

Returns a [DateTime](#datetime) set to the current time, using the local timezone.

````kototype
|timestamp: Number| -> DateTime
````

Returns a [DateTime](#datetime) set to the provided `timestamp` in seconds,
using the local timezone.

````kototype
|timestamp: Number, offset: Number| -> DateTime
````

Returns a [DateTime](#datetime) set to the provided `timestamp` in seconds,
using an `offset` in seconds.

### Example

````koto
now = os.time()
# e.g. 2021-12-11 21:51:14

now.year()
# e.g. 2021

now.hour()
# e.g. 21

now.timestamp()
# e.g. 1639255874.53419
````

{% example_playground_link(version = "0.15") %}
print now = os.time()
# e.g. 2021-12-11 21:51:14

print now.year()
# e.g. 2021

print now.hour()
# e.g. 21

print now.timestamp()
# e.g. 1639255874.53419

{% end %}
## Command

See [`os.command`](#command)

## Command.args

````kototype
|Command, args...| -> Command
````

Adds the given arguments to the command, and returns the command.

### Example

````koto
os.command('ls')
  .args('-al', '/tmp')
  .wait_for_output()
  .stdout()
# -> ...
````

{% example_playground_link(version = "0.15") %}
print os.command('ls')
  .args('-al', '/tmp')
  .wait_for_output()
  .stdout()
# -> ...

{% end %}
## Command.current_dir

````kototype
|Command, path: String| -> Command
````

Sets the command's working directory, and returns the command.

### Example

````koto
os.command('ls')
  .current_dir('/tmp')
  .wait_for_output()
  .stdout()
# -> ...
````

{% example_playground_link(version = "0.15") %}
print os.command('ls')
  .current_dir('/tmp')
  .wait_for_output()
  .stdout()
# -> ...

{% end %}
## Command.env

````kototype
|Command, key: String, value: String| -> Command
````

Sets an environment variable, and returns the command.

### Example

````koto
assert os.command('env')
  .env 'FOO', '123'
  .wait_for_output()
  .stdout()
  .contains 'FOO=123'
````

{% example_playground_link(version = "0.15") %}
assert os.command('env')
  .env 'FOO', '123'
  .wait_for_output()
  .stdout()
  .contains 'FOO=123'

{% end %}
## Command.env_clear

````kototype
|Command| -> Command
````

Clears all environment variables for the command, and returns the command.

This prevents the command from inheriting any environment variables from the parent process.

### Example

````koto
assert os.command('env')
  .env_clear()
  .wait_for_output()
  .stdout()
  .is_empty()
````

{% example_playground_link(version = "0.15") %}
assert os.command('env')
  .env_clear()
  .wait_for_output()
  .stdout()
  .is_empty()

{% end %}
## Command.env_remove

````kototype
|Command, key: String| -> Command
````

Removes the environment variable matching the given key, and returns the command.

### Example

````koto
assert os.command('env')
  .env_clear()
  .env 'FOO', '123'
  .env_remove 'FOO'
  .wait_for_output()
  .stdout()
  .is_empty()
````

{% example_playground_link(version = "0.15") %}
assert os.command('env')
  .env_clear()
  .env 'FOO', '123'
  .env_remove 'FOO'
  .wait_for_output()
  .stdout()
  .is_empty()

{% end %}
## Command.stdin

````kototype
|Command, stream_config: String| -> Command
````

Configures the command's `stdin` stream.

Valid values of `stream_config` are:

* `inherit`: the stream will be inherited from the parent process.
* `piped`: a pipe will be created to connect the parent and child processes.
* `null`: the stream will be ignored.

The default stream behavior is `inherit` when the command is used with `spawn` or `wait_for_exit`, and `piped` when used with `wait_for_output`.

## Command.stdout

````kototype
|Command, stream_config: String| -> Command
````

Configures the command's `stdout` stream.

See [Command.stdin](#command-stdin) for valid values of `stream_config`.

## Command.stderr

````kototype
|Command, stream_config: String| -> Command
````

Configures the command's `stderr` stream.

See [Command.stdin](#command-stdin) for valid values of `stream_config`.

## Command.spawn

````kototype
|Command| -> Child
````

Executes the command, returning the command's [Child](#child) process.

### Example

````koto
spawned = os.command('ls')
  .stdout('piped')
  .spawn()

spawned
  .wait_for_output()
  .stdout()
# -> ...
````

{% example_playground_link(version = "0.15") %}
spawned = os.command('ls')
  .stdout('piped')
  .spawn()

print spawned
  .wait_for_output()
  .stdout()
# -> ...

{% end %}
## Command.wait_for_output

````kototype
|Command| -> CommandOutput
````

Executes the command and waits for it to exit, returning its captured [output](#CommandOutput).

### Example

````koto
os.command('ls').wait_for_output().stdout()
# -> ...
````

{% example_playground_link(version = "0.15") %}
print os.command('ls').wait_for_output().stdout()
# -> ...

{% end %}
## Command.wait_for_exit

````kototype
|Command| -> Number?
````

Executes the command and waits for it to exit, returning its exit code if the command exited normally, or `null` if it was interrupted.

### Example

````koto
os.command('ls').wait_for_exit()
# -> 0
````

{% example_playground_link(version = "0.15") %}
print os.command('ls').wait_for_exit()
# -> 0

{% end %}
## CommandOutput

Contains captured output from a command, and information about how the command exited.

See [Command.wait_for_output](#command-wait-for-output) and [Child.wait_for_output](#child-wait-for-output).

## CommandOutput.exit_code

````kototype
|CommandOutput| -> Number?
````

Returns the command's exit code if available.

## CommandOutput.success

````kototype
|CommandOutput| -> Bool
````

Returns `true` if the command exited successfully.

## CommandOutput.stdout

````kototype
|CommandOutput| -> String?
````

Returns the contents of the command's `stdout` stream if it contains valid unicode, or `null` otherwise.

### See also

* [CommandOutput.stdout_bytes](#commandoutput-stdout-bytes)

## CommandOutput.stderr

````kototype
|CommandOutput| -> String?
````

Returns the contents of the command's `stderr` stream if it contains valid unicode, or `null` otherwise.

## CommandOutput.stdout_bytes

````kototype
|CommandOutput| -> Iterator
````

Returns an iterator that yields the bytes contained in the command's `stdout` stream.

## CommandOutput.stderr_bytes

````kototype
|CommandOutput| -> Iterator
````

Returns an iterator that yields the bytes contained in the command's `stderr` stream.

## Child

A handle to a child process, see [Command.spawn](#command-spawn).

## Child.stdin

````kototype
|Child| -> File
````

Returns the child process's `stdin` standard input stream as a [File](./io#File) that supports write operations.

### Example

````koto
spawned = os.command('cat')
  .stdin 'piped'
  .spawn()

stdin = spawned.stdin()
stdin.write_line 'hello'
stdin.write_line 'one two three'

spawned.wait_for_output().stdout()
# -> hello
# -> one two three
````

{% example_playground_link(version = "0.15") %}
spawned = os.command('cat')
  .stdin 'piped'
  .spawn()

stdin = spawned.stdin()
stdin.write_line 'hello'
stdin.write_line 'one two three'

print spawned.wait_for_output().stdout()
# -> hello
# -> one two three

{% end %}
## Child.stdout

````kototype
|Child| -> File
````

Returns the child process's `stdout` standard output stream as a [File](./io#File) that supports read operations.

Calling this function will prevent the stream from being included in [wait_for_output](#child-wait-for-output).

## Child.stderr

````kototype
|Child| -> File
````

Returns the child process's `stderr` standard error stream as a [File](./io#File) that supports read operations.

Calling this function will prevent the stream from being included in [wait_for_output](#child-wait-for-output).

## Child.has_exited

````kototype
|Child| -> Bool
````

Returns `true` without blocking if the child process has exited, and `false` otherwise.

## Child.wait_for_output

````kototype
|Child| -> CommandOutput
````

Closes all input and output streams, waits for the command to exit, and then returns the captured output.

Note that if the `stdout` or `stderr` streams were manually retrieved via [Child.stdout](#child-stdout)/[Child.stderr](#child-stderr) then they won't be included in the captured output.

## Child.wait_for_exit

````kototype
|Child| -> Number?
````

Closes all input and output streams, waits for the command to exit, and then returns the command's exit code if available.

## DateTime

See [`os.time`](#time).

## DateTime.year

````kototype
|DateTime| -> Number
````

Returns the year component of the provided DateTime.

## DateTime.month

````kototype
|DateTime| -> Number
````

Returns the month component of the provided DateTime.

## DateTime.day

````kototype
|DateTime| -> Number
````

Returns the day component of the provided DateTime.

## DateTime.hour

````kototype
|DateTime| -> Number
````

Returns the hour component of the provided DateTime.

## DateTime.minute

````kototype
|DateTime| -> Number
````

Returns the minute component of the provided DateTime.

## DateTime.nanosecond

````kototype
|DateTime| -> Number
````

Returns the nanosecond component of the provided DateTime.

## DateTime.timestamp

````kototype
|DateTime| -> Number
````

Returns the number of seconds since 00:00:00 UTC on January 1st 1970.

## DateTime.timezone_offset

````kototype
|DateTime| -> Number
````

Returns the DateTime's timezone offset in seconds.

## DateTime.timestamp_string

````kototype
|DateTime| -> String
````

Returns a string representing the DateTime's timezone offset in seconds.

## Timer

See [`os.start_timer`](#start-timer).

## Timer.@- (subtract)

````kototype
|Timer, Timer| -> Number
````

Returns the time difference in seconds between two timers.

### Example

````koto
t1 = os.start_timer()
t2 = os.start_timer()
# t2 was started later than t1, so the time difference is positive
assert (t2 - t1) > 0
# t1 was started earlier than t2, so the time difference is negative
assert (t1 - t2) < 0
````

{% example_playground_link(version = "0.15") %}
t1 = os.start_timer()
t2 = os.start_timer()
# t2 was started later than t1, so the time difference is positive
assert (t2 - t1) > 0
# t1 was started earlier than t2, so the time difference is negative
assert (t1 - t2) < 0

{% end %}
## Timer.elapsed

````kototype
|Timer| -> Number
````

Returns the number of seconds that have elapsed since the timer was started.

### Example

````koto
t = os.start_timer()

# ...after some time...
print "Time taken: ${t.elapsed()}s"
````

{% example_playground_link(version = "0.15") %}
t = os.start_timer()

# ...after some time...
print "Time taken: ${t.elapsed()}s"

{% end %}
