# `cgp`: Context-Generic Programming in Rust

**A language extension for Rust, with pluggable trait implementations at compile-time.**

Still ordinary Rust: a library on the stable toolchain, with provider selection resolved at compile
time, adopted one trait at a time.

In ordinary Rust, a trait has one implementation per type, so a program that needs two behaviors for
the same type has to use a wrapper type or a second trait. Context-Generic Programming (CGP)
works around that limit: one trait can have many interchangeable implementations, called
*providers*, and each *context* chooses the one it uses through a small wiring table.

To get started, use this crate: it re-exports the crates that provide CGP under `cgp::core` and
`cgp::extra`, and gathers the names you write in `cgp::prelude`.

## A quick look

A greeting shows how these pieces fit together. The example below defines a greeting component
and a provider that reads a name, then wires `Person` to that provider. Here `Person` holds the data
being greeted. The `check_components!` line checks the wiring at compile time, making the build
fail if `Person` lacks the `name` field the provider reads:

```rust
use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self, #[implicit] name: &str) -> String {
        format!("Hello, {name}!")
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! { Person { GreeterComponent: GreetHello } }
check_components! { Person { GreeterComponent } }

fn main() {
    let person = Person { name: "World".to_owned() };
    assert_eq!(person.greet(), "Hello, World!");
}
```

For a greeting alone, a plain trait would be enough. Providers start to help once an application
needs interchangeable implementations or wants to reuse and compose them. That flexibility comes
with more declarations, extra compile-time work, and new concepts to learn. The raw compile errors
can also be verbose, so the installation steps below include a tool to help interpret them.

## Install

Add the crate to your project:

```bash
cargo add cgp
```

CGP needs Rust 1.89 or newer on the stable toolchain. With its default features, it needs neither
`std` nor `alloc`, so it builds for `no_std` targets as well. The `DebugError` and `DisplayError`
providers format an error into a `String`, so using them requires an allocator and the `alloc`
feature:

```toml
[dependencies]
cgp = { version = "0.8.0", features = ["alloc"] }
```

Once you start writing CGP, install [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp)
alongside it to help diagnose wiring mistakes. These otherwise show up as errors about generated
types. `cargo cgp check` leads with the root cause for the classes it recognizes, and the tool does
not yet reshape every class.

## Learn CGP

To learn more, visit [contextgeneric.dev](https://contextgeneric.dev/). Its documentation covers
everything from your first program to the details of individual constructs:

- [Quickstart](https://contextgeneric.dev/docs/quickstart): how do I install CGP and run a first
  program?
- [Tutorials](https://contextgeneric.dev/docs/tutorials/hello): how do I learn it by building
  something?
- [Concepts](https://contextgeneric.dev/docs/concepts/): why does CGP work the way it does?
- [Reference](https://contextgeneric.dev/docs/reference/): what does each construct accept and
  generate?
- [Costs and maturity](https://contextgeneric.dev/#costs): what should I weigh before adopting it?

For details about individual API items, use the documentation on docs.rs. Each construct's
documentation also links to the website's reference page explaining how to use it, where one is
available. If you work with a coding agent, the site's
[AI-assisted development](https://contextgeneric.dev/docs/ai/) section explains the agent skill
and the knowledge base it draws on.

## License

Licensed under the [Apache License, Version 2.0](https://github.com/contextgeneric/cgp/blob/main/LICENSE).
