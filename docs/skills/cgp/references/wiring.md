# Wiring

How a context selects which provider implements each of its components, by recording a type-level table that maps every `…Component` marker key to a chosen provider.

A CGP [component](components.md) splits the consumer trait callers use (`CanDoX`) from the provider trait implementers write (`SomethingDoer`), but that split leaves one question open: for a given context, *which* provider supplies the behavior? Wiring is the answer. It is the act of telling a context, component by component, which provider stands behind each consumer-trait call — and the mechanism is a small type-level lookup table carried on the context type. This file explains that table, the `delegate_components!` macro that populates it, and the `UseContext` provider that lets a provider trait route back through the context's own consumer-trait impl.

## The table: `DelegateComponent`

The table is the trait `DelegateComponent<Key>`, which maps one key type to one value type:

```rust
pub trait DelegateComponent<Key: ?Sized> {
    type Delegate;
}
```

There is no method and no data — the trait exists purely to associate a value type (`Delegate`) with a key type (`Key`) on a carrier type (`Self`). The mental model is a type-level key-value map living on the `Self` type, analogous to an object's method table (vtable) in object-oriented languages: where a vtable maps method names to function pointers resolved at runtime, this table maps `…Component` marker types to provider types resolved entirely at compile time. Implementing `DelegateComponent<Key>` is "setting" the entry at `Key`; naming `Self: DelegateComponent<Key>` in a bound and reading `Self::Delegate` is "getting" the value back out. Because Rust forbids two impls of the same trait for the same `Self` and `Key`, each key maps to exactly one value, which is what makes the structure a genuine map.

When the key is a component marker such as `GreeterComponent`, a populated entry causes the context to inherit the matching provider trait through the blanket impl, and from there the consumer trait — so `app.greet()` type-checks. The blanket impl's body is itself a `DelegateComponent` lookup: it reads the entry, finds the provider, and forwards the call to it. The lookup *is* the whole routing mechanism. (When the key is some arbitrary type instead of a component marker — a shape, a tag — the same trait serves as a plain dispatch table walked by a higher-order provider, with no provider trait attached; see [higher-order providers](higher-order-providers.md).)

## `delegate_components!`: populating the table

You almost never write `DelegateComponent` impls by hand. The `delegate_components!` macro populates the table from a compact `Key: Value` syntax, one entry per component, the marker as the key and the chosen provider as the value:

```rust
#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_components! {
    Rectangle {
        AreaCalculatorComponent: RectangleArea,
    }
}
```

This wires `Rectangle` to use the `RectangleArea` provider for its `AreaCalculatorComponent`. After this, calling `rect.area()` resolves through `Rectangle`'s table to `RectangleArea`. The target before the brace is the [context](checking.md) (or an intermediary provider table) whose table is being defined; each key is a `…Component` marker type and each value is the provider type to delegate it to.

When several components share one provider, the array form on the key side lets a bracketed list of markers expand to one entry each, all pointing at the same value:

```rust
delegate_components! {
    Rectangle {
        [
            AreaCalculatorComponent,
            PerimeterCalculatorComponent,
        ]: RectangleGeometry,
        GreeterComponent: GreetHello,
    }
}
```

This is exactly equivalent to writing `AreaCalculatorComponent: RectangleGeometry` and `PerimeterCalculatorComponent: RectangleGeometry` on separate lines, plus the `GreeterComponent` entry — three entries in all.

A `new` keyword in front of the target makes the macro define the target struct as well, saving a separate declaration. `new GeometryComponents { … }` emits `struct GeometryComponents;` alongside the table impls. This is the idiomatic way to declare a standalone provider bundle — a type whose only purpose is to hold a table that other contexts can then delegate to as a single unit:

```rust
delegate_components! {
    new GeometryComponents {
        AreaCalculatorComponent: RectangleArea,
        PerimeterCalculatorComponent: RectanglePerimeter,
    }
}
```

A leading generic list on the target makes the whole table generic, so one wiring applies across a family of contexts: `delegate_components! { <T> MyContext<T> { … } }` wires every `MyContext<T>` at once.

### What the macro generates

Each entry expands to a pair of impls. The first is the `DelegateComponent` impl that stores the entry — for the `Rectangle` example above, the macro emits:

```rust
impl DelegateComponent<AreaCalculatorComponent> for Rectangle {
    type Delegate = RectangleArea;
}
```

This impl alone is what the provider blanket impl reads when it looks the component up. The second impl is an `IsProviderFor` impl that forwards the chosen provider's requirements back through the table, so a missing transitive [impl-side dependency](components.md) still surfaces as a usable compiler error rather than a dead end. The details of that propagation belong to [components](components.md) and [checking](checking.md) — here it is enough to know that wiring an entry makes it both *resolvable* (the `DelegateComponent` half) and *checkable* (the `IsProviderFor` half).

## Explicit delegation: what wiring effectively does

To see what `delegate_components!` buys you, it helps to write the routing out by hand. Without any table, a context can implement a consumer trait directly by calling a provider's provider-trait method itself. Given the `AreaCalculator` provider trait (whose method takes the context as an explicit `Context` parameter rather than `&self`), a context can hand-route its `CanCalculateArea` consumer impl through the `RectangleArea` provider:

```rust
impl CanCalculateArea for Rectangle {
    fn area(&self) -> f64 {
        <RectangleArea as AreaCalculator<Self>>::area(self)
    }
}
```

This is the manual equivalent of one `delegate_components!` entry: it names the provider explicitly, invokes its provider-trait method, and passes `self` as the context. The single line `AreaCalculatorComponent: RectangleArea` in the table achieves the same routing generically, through the generated `DelegateComponent` impl and the blanket impl that reads it — for every method on the trait, without you writing out each call. Use this explicit form only as a teaching device or in the rare case where you want a context to bypass the table for one trait; the table form is the idiom.

## Direct implementation of a consumer trait

A context need not delegate at all. When the behavior is specific to one context and there is no provider worth naming, you can implement the consumer trait directly — plain vanilla Rust, no table involved:

```rust
impl CanGreet for Person {
    fn greet(&self) {
        println!("Hello, I am {}", self.name);
    }
}
```

This bypasses the provider-trait machinery entirely: `person.greet()` calls this impl directly. Direct implementation and table-driven wiring are mutually exclusive for a given component on a given context — the table's blanket impl already supplies the consumer trait, so a hand-written consumer impl would collide with it. Reach for direct implementation when a component has exactly one context-specific behavior and the indirection of a separate provider adds nothing.

## `UseContext`: routing a provider trait back to the consumer trait

`UseContext` is a [provider](components.md) — a zero-sized marker struct with no runtime value — that implements *any* provider trait by forwarding its methods back to the context's own consumer-trait implementation:

```rust
pub struct UseContext;
```

It is the exact dual of the consumer-trait blanket impl. That blanket impl runs consumer-to-provider — a context implements `CanGreet` by delegating to whichever provider implements `Greeter` for it. `UseContext` runs the opposite direction: it implements the provider trait `Greeter` by calling whatever `CanGreet` implementation the context already has. `#[cgp_component]` generates a `UseContext` impl of the provider trait for every component, of roughly the shape:

```rust
impl<Context> Greeter<Context> for UseContext
where
    Context: CanGreet,
{
    fn greet(context: &Context) {
        Context::greet(context)
    }
}
```

So wiring a component to `UseContext` means "use whatever this context already does for this trait." Its purpose is to turn a context's existing consumer-trait impl into a provider that *another* provider can call. The pattern matters most for [higher-order providers](higher-order-providers.md), which take an inner provider as a type parameter and often default it to `UseContext`, so the inner step falls back to the context's own wiring unless an explicit provider is named.

The one rule to respect is the circular-dependency caveat: never delegate a component to `UseContext` when the context's only impl of that component *is* the delegation itself. Doing so asks the context to implement the consumer trait by delegating to a provider (`UseContext`) that in turn implements the provider trait by calling the consumer trait — a cycle the trait solver cannot resolve, surfacing as an overflow or unsatisfied-bound compile error. `UseContext` is meant to be supplied to another provider as its inner provider, not wired as a context's own delegate for the same component.

## Dispatching a component per type with `open`

When a provider trait carries an extra generic parameter and the right provider depends on which concrete type that parameter is, the choice is made by a second lookup keyed on that parameter rather than on the component marker. The `open` statement inside `delegate_components!` is the way to write that per-value wiring: it folds the per-type entries directly into the context's own table, one provider per concrete value of the dispatch parameter. Given a `CanCalculateArea<Shape>` consumer trait whose `Shape` parameter selects the area formula, a context wires each shape to its own provider like this:

```rust
delegate_components! {
    MyApp {
        open { AreaCalculatorComponent };

        @AreaCalculatorComponent.Rectangle: RectangleArea,
        @AreaCalculatorComponent.Circle: CircleArea,
    }
}
```

The leading `open { AreaCalculatorComponent };` header opens one or more components for per-value wiring — list several inside the braces to open them together. The header is a *leading* statement: when a single block mixes plain `Component: Provider` mappings with an `open` block, the `open { … };` header must come first, before any plain mappings, or the macro fails to parse. Each subsequent `@Component.Key: Provider` entry then assigns a provider for one value of that component's dispatch parameter: `@AreaCalculatorComponent.Rectangle: RectangleArea` says that when `Shape` is `Rectangle`, `MyApp` calculates area through `RectangleArea`, and the `Circle` line does the same for `Circle`. After this wiring, `MyApp` implements `CanCalculateArea<Rectangle>` via `RectangleArea` and `CanCalculateArea<Circle>` via `CircleArea`.

Two shorthands keep the entries compact. When several values of the dispatch parameter share one provider, an array on the final path segment expands to one entry each — `@AreaCalculatorComponent.[Rectangle, Circle]: SomeProvider` wires both shapes to `SomeProvider`. When a dispatch value needs generic parameters of its own, they precede the value: `@SomeComponent.<'a, T> &'a T: SomeProvider` dispatches on the type `&'a T` for all `'a` and `T`.

The `open` form needs no extra macro on the component. It works through the `RedirectLookup` impl that every `#[cgp_component]` already generates, so dispatching a component per type requires no `#[derive_delegate]` on the trait — the same component you wire by marker is the one you open by value. `open` is a lightweight form of the full namespace feature, suited to a context wiring its own components directly; it does not combine with a joined namespace where the component carries `#[prefix(...)]`. The full namespace machinery, including `@`-path keys and the `namespace` statement, is described in [namespaces](namespaces.md).

### Legacy: `UseDelegate` nested tables

An older form writes the per-type entries into a separate table wrapped in the `UseDelegate` provider, rather than folding them into the context's own table with `open`. It nests a `new` table inside a single `UseDelegate<…>` value:

```rust
delegate_components! {
    MyApp {
        AreaCalculatorComponent: UseDelegate<new AreaCalculatorComponents {
            Rectangle: RectangleArea,
            Circle: CircleArea,
        }>,
    }
}
```

This desugars `UseDelegate<new AreaCalculatorComponents { … }>` into a standalone `delegate_components! { new AreaCalculatorComponents { … } }` plus an outer entry `AreaCalculatorComponent: UseDelegate<AreaCalculatorComponents>` pointing the component at that inner table. The end effect matches the `open` example above — `MyApp` dispatches `Rectangle` to `RectangleArea` and `Circle` to `CircleArea` — but the dispatch values live in a named side table reached through `UseDelegate` instead of in `MyApp`'s table directly.

This is a legacy dispatch mechanism, retained for compatibility and expected to be deprecated and removed. It is documented here so that the nested-table form can be *read* where it still appears in existing code; for *writing* new wiring, prefer `open`, which dispatches the same component without the extra `UseDelegate` indirection or the separate inner table. The detailed mechanics of the `UseDelegate` provider — how it reads its inner table at the provider level — belong to [higher-order providers](higher-order-providers.md).

## Related constructs

Wiring is the step that connects [components](components.md) — the consumer/provider trait pairs and `…Component` markers — to the providers that implement them, and the `IsProviderFor` impls it generates feed the completeness guarantees described in [checking](checking.md). The `open` statement shown above dispatches a generic-parameter component per value through the `RedirectLookup` impl every component generates; the full namespace feature it derives from, including `@`-path keys and the `namespace` statement, is described in [namespaces](namespaces.md). `UseContext` and the legacy `UseDelegate` provider — including how it reads an inner dispatch table — are covered in [higher-order providers](higher-order-providers.md).

## Further reference

Online docs: [delegate_components.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/delegate_components.md), [traits/delegate_component.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/delegate_component.md), [providers/use_context.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/use_context.md).
