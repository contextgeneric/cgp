# Checking wiring

How to verify at compile time that a context's [wiring](wiring.md) is complete, using check traits and the `check_components!` / `delegate_and_check_components!` macros to turn confusing use-site errors into precise ones at the wiring site.

## Why wiring is lazy

CGP wiring is lazy: recording that a context delegates a [component](components.md) to a provider does not verify that the provider can actually satisfy that component for that context. When you write a `DelegateComponent` entry mapping `GreeterComponent` to `GreetHello`, the type system stores "this key points to this provider" as an associated type and asks no further questions. Whether the provider's own `where` bounds — its impl-side dependencies — hold for this particular context is simply not checked at the point of delegation. The check is deferred until something downstream actually *uses* the component, which is the first moment the compiler is forced to evaluate the provider's bounds against the concrete context.

This laziness is what makes CGP composable, but it has a cost: a context can look fully wired and still be broken. Every entry compiles, the struct compiles, the whole module compiles — and then the first call to a consumer trait method fails, often far from the wiring that caused it. A missing field, a missing abstract type, an unsatisfied transitive bound three providers deep: none of these surface where the mistake was made.

Consider a greeter provider that depends on a `name` field, wired onto a context whose field is misnamed:

```rust
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String, // mismatch: GreetHello needs `name`
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}
```

This whole block compiles. `GreetHello` needs `HasName`, `Person` has no `name` field, and nothing complains — until some distant `person.greet()` call fails to typecheck.

## Why the resulting errors are poor

When a lazily-wired context is finally used and a dependency is missing, the compiler reports the outermost unmet conclusion and hides the reasoning behind it. Asking the plain question "does `Person` implement `CanGreet`?" makes Rust answer with the last link in the chain — typically that `GreetHello` does not implement the provider trait for `Person` — without explaining *why* the provider's bounds were not met. The provider blanket impl is a competing candidate that suppresses the detailed diagnostic. The root cause, a single missing `name` getter, is buried beneath a conclusion that points at the provider rather than at the gap, and the surface error spells field names in their verbose type-level form (`Symbol<…, Chars<…>>`), making even that hard to read.

The fix is to force the compiler to evaluate the provider's bounds *and report them in detail*, at a location you control — the wiring site — rather than wherever the component happens to be used first.

## How check traits force readable errors

A check trait is a dummy trait whose supertrait is the requirement being asserted; implementing it for a context with an empty body compiles only if that requirement holds. The hand-written form is plain Rust:

```rust
trait CanUsePerson: CanGreet {}
impl CanUsePerson for Person {}
```

The `impl` block has nothing to prove on its own, so it succeeds exactly when `Person: CanGreet` holds and fails otherwise. Placed next to the wiring, it converts a latent gap into an immediate compile error at a known line. But asserting the *consumer* trait directly is not enough: it reproduces the same vague error as before, naming the provider rather than the missing dependency. The remedy is to route the assertion through `CanUseComponent` instead.

`CanUseComponent<Component, Params>` is satisfied only when the context both delegates the component and the delegated provider satisfies `IsProviderFor` for that context. The crucial property is that `IsProviderFor` carries the provider's *real* `where` bounds — the same impl-side dependencies the provider needs to implement its provider trait. Routing a check through `CanUseComponent` therefore forces the compiler to evaluate those bounds and, because they are stated explicitly through the marker, to report the specific one that failed. A missing `name` field surfaces as an unsatisfied `HasName`/`HasField` bound pointing at the context, not as a bare "provider not implemented." The two bounds also distinguish the two ways wiring goes wrong: failing the `DelegateComponent` bound means the component was never wired (add the delegation), while failing the `IsProviderFor` bound means it was wired to a provider whose dependencies are unmet (supply the missing dependency). You rarely name `CanUseComponent` directly — its job is to be the bound the check macros emit.

## Generating checks with `check_components!`

Rather than spell out check traits by hand, `check_components!` generates them from a short table of components to verify. Given a context and a list of components, it emits a marker trait aliasing `CanUseComponent` and one empty impl per component:

```rust
check_components! {
    Person {
        GreeterComponent,
    }
}
```

The generated impl compiles only if `Person: CanUseComponent<GreeterComponent, ()>`, which drags in `GreetHello`'s `IsProviderFor` bounds and reports the first that fails — here, the missing `name` field, pinpointed at the wiring site instead of at a future `person.greet()`. A successful build *is* the passing assertion; these checks have no runtime existence.

The check trait is named `__Check{Context}` by default — `__CheckPerson` here. When two `check_components!` tables in the same module would collide on that name, override it with `#[check_trait(Name)]` on the table:

```rust
check_components! {
    #[check_trait(CheckPersonGreeting)]
    Person {
        GreeterComponent,
    }
}
```

A component with generic parameters cannot be checked bare, because the check must name concrete parameters to have anything to verify. List them after a colon: a single parameter bare, multiple parameters grouped into a tuple, mirroring how the provider trait groups them in its `IsProviderFor` `Params` slot. Given an area calculator generic over a shape:

```rust
#[cgp_component(AreaOfShapeCalculator)]
pub trait CanCalculateAreaOfShape<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}

check_components! {
    MyApp {
        AreaOfShapeCalculatorComponent: Rectangle,         // one parameter
        TransformCalculatorComponent: (Rectangle, f64),    // two, as a tuple
    }
}
```

Array syntax on either side of the colon expands to the cartesian product, so a set of components can be checked against a set of parameters in one line. A bracketed value checks one component against several parameter sets; a bracketed key checks several components against one set; bracketing both checks every combination:

```rust
check_components! {
    MyApp {
        AreaOfShapeCalculatorComponent: [Rectangle, Circle],   // one component, two shapes
    }
}
```

This verifies `MyApp: CanCalculateAreaOfShape<Rectangle>` and `MyApp: CanCalculateAreaOfShape<Circle>` in one entry.

## Checking providers directly with `#[check_providers(...)]`

For [higher-order providers](higher-order-providers.md), checking the context as a whole tells you a layer is broken but not which one. The `#[check_providers(...)]` attribute changes what is checked: instead of asserting `CanUseComponent` on the context, it asserts `IsProviderFor` directly on each named provider, so each layer is verified on its own impl:

```rust
check_components! {
    #[check_trait(CheckScaledRectangleProviders)]
    #[check_providers(
        RectangleAreaCalculator,
        ScaledAreaCalculator<RectangleAreaCalculator>,
    )]
    ScaledRectangle {
        AreaCalculatorComponent,
    }
}
```

Because each provider is checked independently, a dependency missing only from the outer `ScaledAreaCalculator` wrapper errors on the wrapper's line alone, while one missing from the inner `RectangleAreaCalculator` errors on both lines — which narrows down where in a nested stack the gap lives.

## Wiring and checking together with `delegate_and_check_components!`

Keeping a standalone `check_components!` block in sync with the wiring is manual bookkeeping: add a delegation and you must remember to add its check. `delegate_and_check_components!` fuses the two — it wires each entry exactly as `delegate_components!` would and derives a check for each delegated key, so every wiring is proven the moment it is written:

```rust
#[derive(HasField)]
pub struct MyContext {
    pub name: String,
}

delegate_and_check_components! {
    MyContext {
        NameTypeProviderComponent: UseType<String>,
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}
```

If `MyContext` were missing the `name` field, the derived check on `NameGetterComponent` would fail to compile and report the missing bound, rather than letting the gap slip through to a later use.

Its check trait is named `__CanUse{Context}` by default — deliberately distinct from `check_components!`'s `__Check{Context}` — so one of each macro can appear in the same module without a clash. As with `check_components!`, `#[check_trait(Name)]` overrides the derived name.

Because the delegation half is generic over a component's parameters but the check half needs concrete ones, an entry for a component with generic parameters *requires* a `#[check_params(...)]` attribute supplying them, using the same single-versus-tuple convention:

```rust
delegate_and_check_components! {
    MyApp {
        #[check_params(Rectangle, Circle)]
        AreaOfShapeCalculatorComponent:
            UseDelegate<new AreaOfShapeCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

The nested `UseDelegate` table shown here is the legacy form of per-type dispatch; the modern equivalent opens the component with the `open` statement of `delegate_components!` (see [wiring](wiring.md)). The `#[check_params(...)]` requirement is the same either way — whichever wiring form supplies the dispatch entries, the check half still needs the concrete parameters spelled out.

To wire an entry without checking it — for instance a higher-order delegation you verify separately with a `#[check_providers(...)]` block — mark it `#[skip_check]`. The two attributes are mutually exclusive on a given entry:

```rust
delegate_and_check_components! {
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,

        #[skip_check]
        TransformCalculatorComponent:
            ComplexTransform<RectangleAreaCalculator>, // checked in a dedicated check_components! block
    }
}
```

The recommendation follows from these defaults: reach for `delegate_and_check_components!` for a main context's wiring, where catching a broken or incomplete wiring as early as possible is most valuable, and keep plain `delegate_components!` for intermediary provider-bundle tables that group providers without being a context in their own right. Use a standalone `check_components!` block when a check needs `#[check_providers(...)]` or other control beyond per-entry `#[check_params]` and `#[skip_check]`.

## Debugging an unsatisfied check

When a check fails, the error names the unmet bound — read it as a thread to pull rather than a final verdict. The reported bound is the first impl-side dependency the compiler could not satisfy; walk the transitive dependencies from there. If `GreetHello` requires `HasName`, the error names `HasName` on the context, and the fix is to supply that field or wire the getter component. A bound several providers deep names the innermost requirement, so trace from the failing component through each provider it delegates into.

When a large `delegate_and_check_components!` table reports a tangle of errors, narrow it down by adding the suspect component to a separate `check_components!` block on its own. Checking one component in isolation, with its parameters spelled out, strips away the noise from the other entries and forces the compiler to report just that component's unmet dependency. For a higher-order stack, switch to `#[check_providers(...)]` to see which layer fails on its own line.

Finally, remember that not every unsatisfied bound is a CGP component. A check trait only verifies wiring routed through `CanUseComponent` — it cannot prove a plain trait or a blanket-impl bound that a provider also depends on. If the error names a trait that has no `…Component` marker and no entry in any delegation table, no amount of checking will surface it through the wiring; that bound must be satisfied by ordinary Rust means (an `impl`, a derive, a `where` clause on the context), and the check will pass only once it is.

## Further reference

Online docs: [`check_components.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/check_components.md), [`delegate_and_check_components.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/delegate_and_check_components.md), and the conceptual overview [`concepts/check-traits.md`](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/check-traits.md).
