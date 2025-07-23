+++
title = "Rust API"
template = "docs-guide.html"
insert_anchor_links = "heading"
weight = 4
+++


# Rust API

This document contains a collection of examples of how to interact with Koto from Rust code.

The complete API documentation can be found [here][koto-docs].

## Hello World

To run a Koto script, instantiate `koto::Koto` and call `compile_and_run`:

````rust
use koto::{Result, prelude::*};

fn main() -> Result<()> {
    let script = "print 'Hello, World!'";

    Koto::default().compile_and_run(script)?;

    Ok(())
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
use anyhow::{Result, bail};
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
use anyhow::{Result, bail};
use koto::prelude::*;

fn main() -> Result<()> {
    let script = "
export
  number: 42
  string: 'Hello from Koto'
";

    let mut koto = Koto::default();
    koto.compile_and_run(script)?;

    let exports = koto.exports();

    let Some(KValue::Number(exported_number)) = exports.get("number") else {
        bail!("Expected an exported number");
    };
    let Some(KValue::Str(exported_string)) = exports.get("string") else {
        bail!("Expected an exported string");
    };

    println!("Exported number: {exported_number}");
    println!("Exported string: '{exported_string}'");

    Ok(())
}
````

## Using Serde for Value Conversions

Types that implement `serde::Deserialize` and `Serialize` can be converted
to and from Koto values via `koto::serde::to_koto_value` and `from_koto_value`.

````rust
use koto::{
    Result,
    prelude::*,
    serde::{from_koto_value, to_koto_value},
};
use serde::{Deserialize, Serialize};

fn main() -> Result<()> {
    let script = "
match request
  'one_to_four' then
    caption = 'one to four'
    numbers = 1, 2, 3, 4
  'five_to_eight' then
    caption = 'five to eight'
    numbers = 5, 6, 7, 8

export {caption, numbers}
";

    let mut koto = Koto::default();

    // Add a 'request' value to the prelude
    koto.prelude()
        .insert("request", to_koto_value(Request::FiveToEight)?);
    koto.compile_and_run(script)?;

    // After running the script, deserialize the values that the script exported
    let exported: Exported = from_koto_value(koto.exports().clone())?;

    println!("Exported: '{}': {:?}", exported.caption, exported.numbers);

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Exported {
    caption: String,
    numbers: Vec<i64>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Request {
    OneToFour,
    FiveToEight,
}
````

## Adding Values to the Prelude

The runtime's prelude is a `KMap`, which is Koto's standard hashmap type.

Values can be added to the prelude via `KMap::insert`, taking any Rust value
that implements `Into<KValue>`. Basic types like strings and numbers are
automatically converted to corresponding Koto types.

````rust
use koto::{Result, prelude::*};

fn main() -> Result<()> {
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

    koto.compile_and_run(script)?;

    Ok(())
}
````

## Removing Values from the Prelude

Values can also be removed from the prelude, which can be useful if you want
to restrict the capabilities of a script.

````rust
use koto::prelude::*;

fn main() {
    let mut koto = Koto::default();
    let prelude = koto.prelude();

    // Remove the core library's io module from the prelude.
    prelude.remove("io");
    // Remove the os.command function while allowing access to the rest of the os module.
    prelude.remove_path("os.command");

    // These scripts will now throw errors when run.
    assert!(koto.compile_and_run("io.create('temp.txt')").is_err());
    assert!(koto.compile_and_run("os.command('ls')").is_err());

    // os.name is still available so this script will run successfully.
    assert!(koto.compile_and_run("print os.name()").is_ok());
}
````

## Passing Arguments to Koto

The arguments that are accessible in a script from `os.args` can be set via
`Koto::set_args`.

````rust
use koto::{Result, prelude::*};

fn main() -> Result<()> {
    let script = "
from os import args

if (size args) > 1
  for i, arg in args.enumerate()
    print '{i + 1}: {arg}'
else
  print 'No arguments'
";

    let mut koto = Koto::default();

    koto.set_args(std::env::args())?;
    koto.compile_and_run(script)?;

    Ok(())
}
````

## Calling Rust Functions in Koto

Any Rust function that implements `KotoFunction` can be made available to the
Koto runtime.

````rust
use koto::{Result, prelude::*, runtime};

fn main() -> Result<()> {
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
        unexpected => unexpected_args("|Number, Number|", unexpected),
    });

    koto.compile_and_run(script)?;

    Ok(())
}

fn say_hello(ctx: &mut CallContext) -> runtime::Result<KValue> {
    match ctx.args() {
        [] => println!("Hello?"),
        [KValue::Str(name)] => println!("Hello, {name}"),
        unexpected => return unexpected_args("||, or |String|", unexpected),
    }

    Ok(KValue::Null)
}
````

## Calling Koto Functions in Rust

`Koto::call_function` can be used to call Koto functions, or any other callable
Koto values.

````rust
use koto::{Result, prelude::*};

fn main() -> Result<()> {
    let script = "
export my_fn = |a, b| '{a} + {b} is {a + b}'
";
    let mut koto = Koto::default();

    // Run the script, which exports the `my_fn` function
    koto.compile_and_run(script)?;

    let result = koto.call_exported_function("my_fn", &[1.into(), 2.into()])?;
    println!("Result: {}", koto.value_to_string(result)?);

    Ok(())
}
````

## Adding a Module to the Prelude

A module in Koto is simply a `KMap`, conventionally with a defined
[`@type`][type].

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
        unexpected => unexpected_args("|String|", unexpected),
    });

    module.add_fn("square", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok((n * n).into()),
        unexpected => unexpected_args("|Number|", unexpected),
    });

    module
}
````

## Adding a Custom Object Type

Any Rust type that implements `KotoObject` can be used in the Koto runtime.
`KotoObject` requires `KotoType`, `KotoCopy`, and `KotoEntries` to be
implemented.

````rust
use koto::{Result, derive::*, prelude::*, runtime};

fn main() -> Result<()> {
    let script = "
my_type = make_my_type 41
print my_type.get()
print my_type.set 99
";
    let mut koto = Koto::default();

    koto.prelude()
        .add_fn("make_my_type", |ctx| match ctx.args() {
            [KValue::Number(n)] => Ok(MyType::make_koto_object(*n).into()),
            unexpected => unexpected_args("|Number|", unexpected),
        });

    koto.compile_and_run(script)?;

    Ok(())
}

// MyType is a type that we want to use in Koto
//
// The KotoCopy and KotoType traits are automatically derived.
#[derive(Clone, Copy, KotoCopy, KotoType)]
struct MyType(i64);

// The KotoEntries trait is implemented by the koto_impl macro,
// generating Koto functions for any impl function tagged with #[koto_method],
// and inserting them into a cached KMap.
#[koto_impl]
impl MyType {
    fn make_koto_object(n: KNumber) -> KObject {
        // From is available for any type that implements KotoObject
        let my_type = Self(n.into());
        KObject::from(my_type)
    }

    // A simple getter function
    #[koto_method]
    fn get(&self) -> runtime::Result<KValue> {
        Ok(self.0.into())
    }

    // A function that returns the object instance as the result
    #[koto_method]
    fn set(ctx: MethodContext<Self>) -> runtime::Result<KValue> {
        match ctx.args {
            [KValue::Number(n)] => {
                ctx.instance_mut()?.0 = n.into();
                ctx.instance_result()
            }
            unexpected => unexpected_args("|Number|", unexpected),
        }
    }
}

impl KotoObject for MyType {
    // KotoObject::Display allows mytype to be used with Koto's print function
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(format!("MyType({})", self.0));
        Ok(())
    }
}
````

## Disabling type checks

Runtime type checks are enabled by default, the compiler can be prevented from
emitting type check instructions by disabling the `enable_type_checks` flag.

````rust
use koto::{Result, prelude::*};

fn main() -> Result<()> {
    let script = "
let x: String = 123
";
    let mut koto = Koto::default();

    // Type checks are enabled by default. Running the script will produce an error.
    let result = koto.compile_and_run(script);
    assert!(result.is_err());

    // Type checks can disabled via `CompileArgs::enable_type_checks`.
    // It should go without saying that checks should only be disabled if you're confident that your
    // code is correct!
    let result = koto.compile_and_run(CompileArgs::new(script).enable_type_checks(false));
    assert!(result.is_ok());

    Ok(())
}
````

## Using the multi-threaded runtime

By default, Koto's runtime is single-threaded, and many of its core types (e.g. `KValue`) don't
implement `Send` or `Sync`.

For applications that need to support multi-threaded scripting, the `arc` feature switches from an
`Rc<RefCell<T>>`-based memory strategy to one using `Arc<RwLock<T>>`.

Only one memory strategy can be enabled at a time, so default features need to be disabled.

````toml
# Cargo.toml
# ...

[dependencies.koto]
version = "0.15"
default-feautures = false
features = ["arc"]
````

## Using Koto in a REPL

Some applications (like REPLs) require assigned variables to persist between each script evaluation.
This can be achieved by enabling the `export_top_level_ids` flag,
which will result in all top-level assignments being exported.

````rust
use koto::{Result, prelude::*};

fn main() -> Result<()> {
    let mut koto = Koto::default();

    // When using Koto in a REPL, variables from previous evaluations need to be made available
    // for the next evaluation.
    //
    // This is achieved by telling the compiler to treat each top-level assignment as if it had been
    // exported. i.e., the compiler turns the script `x = 1` into `export x = 1`.
    koto.compile_and_run(CompileArgs::new("x = 1").export_top_level_ids(true))?;
    assert!(koto.exports().get("x").is_some());

    // The exports map gets reused by the Koto instance for each run.
    match koto.compile_and_run(CompileArgs::new("x + x").export_top_level_ids(true))? {
        KValue::Number(result) => assert_eq!(result, KNumber::from(2)),
        unexpected => unexpected_type("Number", &unexpected)?,
    }

    Ok(())
}
````

---

[koto-docs]: https://docs.rs/koto/latest/koto/
[type]: ../language/#type
