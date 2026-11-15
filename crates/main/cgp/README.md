# `cgp`: Context-Generic Programming in Rust

**A language extension for Rust, with pluggable trait implementations at compile-time.**

Still ordinary Rust: a library on the stable toolchain, with provider selection resolved at compile
time, adopted one trait at a time.

In ordinary Rust, a trait has one implementation per type, so a program that needs two behaviors for
the same type has to reach for a wrapper type or a second trait. Context-Generic Programming (CGP)
works around that limit: one trait can have many interchangeable implementations, called
*providers*, and each *context* chooses the one it uses through a small wiring table. This crate is
where you start. It is the facade over the crates that provide CGP, re-exporting them under
`cgp::core` and `cgp::extra` and gathering the names you write in `cgp::prelude`.

## A quick look

The smallest useful example is a greeting. It defines a greeting component, writes one provider
that reads a name, and wires `Person` to that provider. Here `Person` is the data being greeted, and
the `check_components!` line at the end makes the build fail if `Person` lacks the `name` field the
provider reads:

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
needs interchangeable implementations, or wants to reuse and compose them, and that help has a
price: more declarations, extra compile-time work, and concepts to learn. The raw compile errors can
also be verbose, which is why the install steps below suggest a second tool.

## Install

Add the crate to your project:

```bash
cargo add cgp
```

CGP needs Rust 1.89 or newer on the stable toolchain. Its default features need neither `std` nor
`alloc`, so it builds for `no_std` targets as well. Two providers, `DebugError` and `DisplayError`,
format an error into a `String`, so they need an allocator and the `alloc` feature:

```toml
[dependencies]
cgp = { version = "0.8.0", features = ["alloc"] }
```

Once you start writing CGP, install [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp)
beside it, because a wiring mistake otherwise shows up as an error about generated types.
`cargo cgp check` leads with the root cause for the classes it recognizes, and the tool does not yet
reshape every class.

## Learn CGP

The documentation lives on the website, [contextgeneric.dev](https://contextgeneric.dev/), and each
part answers a different question:

- [Quickstart](https://contextgeneric.dev/docs/quickstart): how do I install CGP and run a first
  program?
- [Tutorials](https://contextgeneric.dev/docs/tutorials/hello): how do I learn it by building
  something?
- [Concepts](https://contextgeneric.dev/docs/concepts/): why does CGP work the way it does?
- [Reference](https://contextgeneric.dev/docs/reference/): what does each construct accept and
  generate?
- [Costs and maturity](https://contextgeneric.dev/#costs): what should I weigh before adopting it?

The API documentation on docs.rs describes each item, and a construct's documentation links the
website's reference page that explains how to use it, where the site has one. If you work with a
coding agent, the site's [AI-assisted development](https://contextgeneric.dev/docs/ai/) section
covers the agent skill and the knowledge base the skill draws on.

## License

Licensed under the [Apache License, Version 2.0](https://github.com/contextgeneric/cgp/blob/main/LICENSE).
