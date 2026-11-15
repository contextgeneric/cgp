# `cgp`: Context-Generic Programming in Rust

**A language extension for Rust, with pluggable trait implementations at compile-time.**

Still ordinary Rust: a library on the stable toolchain, with provider selection resolved at compile
time, adopted one trait at a time.

In ordinary Rust, a trait has one implementation per type. Context-Generic Programming (CGP) lets
one trait have many interchangeable implementations, called *providers*, and lets each *context*
choose the one it uses through a small wiring table. The compiler resolves that table statically,
so the choice adds nothing at runtime. This crate is the facade over the crates that provide CGP: it
re-exports them under `cgp::core` and `cgp::extra`, and gathers the names users write in
`cgp::prelude`.

## A quick look

This example defines a greeting component, writes one provider for it, and wires `Person` to that
provider. `Person` is the data being greeted, and the check at the end makes the build fail if
`Person` lacks the `name` field the provider reads:

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

A plain trait would be enough for this greeting alone. Providers help when an application needs
interchangeable implementations, or wants to reuse and compose them, and they cost more
declarations, extra compile-time work, and concepts to learn. The raw compile errors can also be
verbose, which is why the next section suggests a second tool.

## Install

```bash
cargo add cgp
```

CGP requires Rust 1.89 or newer and runs on the stable toolchain. When you start with CGP, install
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) beside it: `cargo cgp check` leads with
the root cause for the classes it recognizes, and the tool does not yet reshape every class.

## Learn CGP

The documentation lives on the website, [contextgeneric.dev](https://contextgeneric.dev/):

- [Quickstart](https://contextgeneric.dev/docs/quickstart): install CGP and run a first program.
- [Tutorials](https://contextgeneric.dev/docs/tutorials/hello): learn by building.
- [Concepts](https://contextgeneric.dev/docs/concepts/): why CGP works the way it does.
- [Reference](https://contextgeneric.dev/docs/reference/): one page per construct.
- [Costs and maturity](https://contextgeneric.dev/#costs): what to weigh before adopting it.

The API documentation here describes each item, and a construct's documentation links the website's
reference page that explains how to use it, where the site has one. If you work with a coding agent,
the site's [AI-assisted development](https://contextgeneric.dev/docs/ai/) section covers the agent
skill and the knowledge base it draws on.

## License

Licensed under the [Apache License, Version 2.0](https://github.com/contextgeneric/cgp/blob/main/LICENSE).
