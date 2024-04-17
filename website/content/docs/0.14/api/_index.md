+++
title = "Rust API"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 6
+++


# Rust API Cookbook

## Hello World

To run a Koto script, instantiate `koto::Koto` and call `compile_and_run`:

````rust
use koto::prelude::*;

fn main() {
    Koto::default()
        .compile_and_run("print 'Hello, World!'")
        .unwrap();
}
````

## Getting a Return Value

The result of calling `compile_and_run` is a `KValue`, which is Koto's main
value type.

`KValue` is an enum that contains variants for each of the core Koto types, 
like `Number`, `String`, etc.

The type of a `KValue` as a string can be retrieved via `KValue::type_as_string`,
and to render a `KValue`, call `Koto::value_to_string`.

````rust
use anyhow::{bail, Result};
use koto::prelude::*;

fn main() -> Result<()> {
    let script = "1 + 2";

    let mut koto = Koto::default();
    match koto.compile_and_run(script)? {
        KValue::Number(result) => {
            println!("The result of '{script}' is {result}");
        }
        other => bail!(
            "Expected a Number, found '{}': ({})",
            other.type_as_string(),
            koto.value_to_string(other)?
        ),
    }

    Ok(())
}
````

## Getting an Exported Value

Values that are exported from the script are inserted in to the *exports* map,
which can be accessed by calling `Koto::exports()`.

````rust
use koto::{prelude::*, Result};

fn main() -> Result<()> {
    let script = "
export
  foo: '42'.to_number()
  bar: 'Hello from Koto'
";

    let mut koto = Koto::default();
    koto.compile_and_run(script)?;

    let exports = koto.exports();
    let foo = exports.get("foo").unwrap();
    let bar = exports.get("bar").unwrap();

    println!("foo: {}", koto.value_to_string(foo)?,);
    println!("bar: {}", koto.value_to_string(bar)?,);

    Ok(())
}
````

## Adding Values to the Prelude

The runtime's prelude is a `KMap`, which is Koto's standard hashmap type. 

Values can be added to the prelude via `KMap::insert`, taking any Rust value
that implements `Into<KValue>`. Basic types like strings and numbers are
automatically converted to corresponding Koto types. 

````rust
use koto::prelude::*;

fn main() {
    let script = "
print 'name: {name}'
print 'how_many: {how_many}'
print 'yes_or_no: {if yes_or_no then 'yes' else 'no'}'
";
    let mut koto = Koto::default();

    let prelude = koto.prelude();
    prelude.insert("name", "Alice");
    prelude.insert("how_many", 99);
    prelude.insert("yes_or_no", true);

    koto.compile_and_run(script).unwrap();
}
````

## Passing Arguments to Koto

The arguments that are accessible in a script from `koto.args` can be set via
`Koto::set_args`.

````rust
use koto::prelude::*;

fn main() {
    let script = "
from koto import args

if (size args) > 0
  for i, arg in args.enumerate()
    print '{i + 1}: {arg}'
else
  print 'No arguments'
";
    let mut koto = Koto::default();
    let args: Vec<_> = std::env::args().collect();
    koto.set_args(&args).unwrap();
    koto.compile_and_run(script).unwrap();
}
````

## Calling Rust Functions in Koto

Any Rust function that implements `KotoFunction` can be made available to the
Koto runtime. 

````rust
use koto::prelude::*;

fn main() {
    let script = "
say_hello()
say_hello 'Alice'
print plus 10, 20
";
    let mut koto = Koto::default();
    let prelude = koto.prelude();

    // Standalone functions can be inserted directly
    prelude.insert("say_hello", say_hello);

    // The add_fn helper avoids the need for type annotations
    prelude.add_fn("plus", |ctx| match ctx.args() {
        [KValue::Number(a), KValue::Number(b)] => Ok((a + b).into()),
        unexpected => type_error_with_slice("two numbers", unexpected),
    });

    koto.compile_and_run(script).unwrap();
}

fn say_hello(ctx: &mut CallContext) -> koto::Result<KValue> {
    match ctx.args() {
        [] => println!("Hello?"),
        [KValue::Str(name)] => println!("Hello, {name}"),
        unexpected => return type_error_with_slice("an optional string", unexpected),
    }

    Ok(KValue::Null)
}
````

## Calling Koto Functions in Rust

`Koto::call_function` can be used to call Koto functions, or any other callable
Koto values.

````rust
use anyhow::Result;
use koto::prelude::*;

fn main() -> Result<()> {
    let script = "
export foo = |a, b| '{a} + {b} is {a + b}'
";
    let mut koto = Koto::default();

    // Running the script exports the `foo` function
    koto.compile_and_run(script).unwrap();
    let foo = koto.exports().get("foo").unwrap();
    assert!(foo.is_callable());

    let result = koto.call_function(foo, &[1.into(), 2.into()])?;
    println!("Result: {}", koto.value_to_string(result)?);

    Ok(())
}
````

## Adding a Module to the Prelude

A module in Koto is simply a `KMap`, conventionally with a defined
[`@type`](../language/#type).

````rust
use koto::prelude::*;

fn main() {
    let script = "
from my_module import echo, square

print echo 'Hello'
print square 9
";
    let mut koto = Koto::default();
    koto.prelude().insert("my_module", make_module());
    koto.compile_and_run(script).unwrap();
}

fn make_module() -> KMap {
    // The `KMap::with_type` initializer sets up an empty map with a `@type` entry.
    let module = KMap::with_type("my_module");

    module.add_fn("echo", |ctx| match ctx.args() {
        [KValue::Str(s)] => Ok(format!("{s}!").into()),
        unexpected => type_error_with_slice("a string", unexpected),
    });

    module.add_fn("square", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok((n * n).into()),
        unexpected => type_error_with_slice("a number", unexpected),
    });

    module
}
````

## Adding a Custom Object Type

Any Rust type that implements `KotoObject` can be used in the Koto runtime.
`KotoObject` requires `KotoType`, `KotoCopy`, and `KotoEntries` to be
implemented. 

````rust
use koto::{derive::*, prelude::*, Result};

fn main() {
    let script = "
foo = make_foo 41
print foo.get()
print foo.set 99
";
    let mut koto = Koto::default();

    koto.prelude().add_fn("make_foo", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok(Foo::make_koto_object(*n).into()),
        unexpected => type_error_with_slice("a number", unexpected),
    });

    koto.compile_and_run(script).unwrap();
}

// Foo is a type that we want to use in Koto
//
// The KotoCopy and KotoType traits are automatically derived.
#[derive(Clone, Copy, KotoCopy, KotoType)]
struct Foo(i64);

// The KotoEntries trait is implemented by the koto_impl macro,
// generating Koto functions for any impl function tagged with #[koto_method],
// and inserting them into a cached KMap.
#[koto_impl]
impl Foo {
    fn make_koto_object(n: KNumber) -> KObject {
        // From is available for any type that implements KotoObject
        let foo = Self(n.into());
        KObject::from(foo)
    }

    // A simple getter function
    #[koto_method]
    fn get(&self) -> Result<KValue> {
        Ok(self.0.into())
    }

    // A function that returns the object instance as the result
    #[koto_method]
    fn set(ctx: MethodContext<Self>) -> Result<KValue> {
        match ctx.args {
            [KValue::Number(n)] => {
                ctx.instance_mut()?.0 = n.into();
                ctx.instance_result()
            }
            unexpected => type_error_with_slice("a Number", unexpected),
        }
    }
}

impl KotoObject for Foo {
    // KotoObject::Display allows Foo to be used with Koto's print function
    fn display(&self, ctx: &mut DisplayContext) -> Result<()> {
        ctx.append(format!("Foo({})", self.0));
        Ok(())
    }
}
````