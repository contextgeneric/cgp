# cgp-component

In [Context-Generic Programming (CGP)](https://contextgeneric.dev/), each context chooses, at
compile time, which implementation of a trait it uses. `cgp-component` provides the machinery behind
that choice: the delegation traits that record which provider a context chose for each component,
the lookup that reads the choice, and the providers CGP ships with. The choice is recorded in a
type-level table, which the lookup reads:

```rust,ignore
use cgp::core::component::DelegateComponent;
use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> String {
        "Hello!".to_owned()
    }
}

pub struct Person;

delegate_components! { Person { GreeterComponent: GreetHello } }

fn main() {
    // The wiring is a type-level table: this entry names `GreetHello`.
    let _provider: <Person as DelegateComponent<GreeterComponent>>::Delegate = GreetHello;
    assert_eq!(Person.greet(), "Hello!");
}
```

The website's reference covers its main items:
[`DelegateComponent`](https://contextgeneric.dev/docs/reference/traits/wiring/delegate_component),
[`IsProviderFor`](https://contextgeneric.dev/docs/reference/traits/wiring/is_provider_for),
[`CanUseComponent`](https://contextgeneric.dev/docs/reference/traits/wiring/can_use_component),
[`UseContext`](https://contextgeneric.dev/docs/reference/providers/use_context),
[`UseDefault`](https://contextgeneric.dev/docs/reference/providers/use_default),
[`UseDelegate`](https://contextgeneric.dev/docs/reference/providers/use_delegate), and
[`RedirectLookup`](https://contextgeneric.dev/docs/reference/providers/redirect_lookup).

The `cgp` crate re-exports it as `cgp::core::component`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.