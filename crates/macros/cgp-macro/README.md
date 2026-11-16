# cgp-macro

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time, and `cgp-macro` provides the macros that write the traits
and impls behind them: `#[cgp_component]` defines a component, `#[cgp_impl]` writes a provider, and
`delegate_components!` and `check_components!` wire a context and check the wiring. This greeting
uses all four:

```rust,ignore
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

The website's reference covers its main items:
[`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component),
[`#[cgp_impl]`](https://contextgeneric.dev/docs/reference/macros/cgp_impl),
[`delegate_components!`](https://contextgeneric.dev/docs/reference/macros/delegate_components), and
[`check_components!`](https://contextgeneric.dev/docs/reference/macros/check_components).

The `cgp` crate gathers its macros in `cgp::prelude`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.