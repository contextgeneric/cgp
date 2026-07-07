# Dynamic dispatch, dynamic typing, and prototypal inheritance

Dynamically-typed languages resolve almost everything at runtime: a method call is *dynamically dispatched* to an implementation chosen from the receiver, an object's behavior is looked up in a *vtable* or a method dictionary, code is written in *duck-typed* style that trusts a value to respond to the messages sent to it, and shared behavior is inherited by *delegation* along a prototype chain. CGP reproduces the openness all of this buys — many interchangeable implementations behind one interface, behavior assembled by delegation, defaults inherited and overridden, and provider code that reads as if it just sends messages to an object — but resolves every bit of it at compile time into direct, monomorphized calls, so its vtable is a type-level table erased before the program runs and its prototype chain is walked by the type checker rather than the CPU.

## Purpose

Dynamic dispatch exists to decouple a call site from the implementation it invokes, so that one piece of code works over many implementations chosen later. When code calls `shape.area()`, dynamic dispatch lets the concrete implementation be decided by the receiver rather than fixed where the call is written, which is what makes polymorphism, plugins, and open extension possible. Dynamically-typed languages take this to its limit: nothing about the receiver's type is fixed at compile time, so a value is whatever it can *do*, behavior is shared by pointing one object at another, and the whole program is malleable at runtime. The appeal is flexibility and immediacy — write against an interface without declaring it, extend without recompiling, explore in a REPL.

This is the same decoupling CGP performs, which is why the comparison is illuminating rather than incidental, and the knowledge base already draws the connection: the [`delegate_components!`](../reference/macros/delegate_components.md) reference calls a context's wiring "a type-level table, analogous to an object's method table (vtable)," and the [bypassing coherence](../concepts/coherence.md) concept is careful to add that, unlike a real vtable, CGP's resolution is static and compiles down to direct calls with no runtime table and no dynamic dispatch. Both paradigms answer "how does a call reach an implementation chosen elsewhere?" — dynamic languages by looking it up at runtime, CGP by resolving it at compile time through the trait system. This document meets the reader who thinks in objects, messages, vtables, and prototypes, and shows where CGP's static mirror of those mechanisms lands. It is the object-oriented, dynamic-language counterpart to the type-theoretic [type classes](type-classes.md) and [ML modules](ml-modules.md) comparisons; the *type-system* side of duck typing — structural versus nominal typing — is developed in [row polymorphism](row-polymorphism.md), so this document leans on that one for the shape theory and keeps its own focus on dispatch, delegation, and the feel of the code.

## The concept in depth

Dynamic languages layer several ideas that a reader should keep distinct: *dynamic typing* and the *duck typing* style it enables, *dynamic dispatch* as the runtime resolution of a call, the *vtable* and method-dictionary machinery that implements it, and *prototypal inheritance* as the runtime sharing of behavior by delegation. The subsections build up in that order and close on the property of delegation — that `self` stays bound to the original object — that turns out to line up with CGP most precisely.

### Dynamic typing and duck typing

A dynamically-typed language checks types at runtime, and *duck typing* is the programming style this permits: an object's usability is decided by the methods and properties it actually has, not by a class it declares or an interface it implements. The maxim is "if it walks like a duck and quacks like a duck, then it is a duck" — a function that calls `x.quack()` works for *any* `x` that responds to `quack`, with no declared relationship between them ([*Duck typing*, Wikipedia](https://en.wikipedia.org/wiki/Duck_typing)). Code written this way sends messages to a value and trusts it to respond, deferring the question "does it actually have this method?" to the moment the call runs. Python, Ruby, JavaScript, and Smalltalk are the canonical homes of the style, and its whole appeal is that an interface need never be spelled out: you write the calls, and any object that can service them qualifies.

### Dynamic dispatch and late binding

Dynamic dispatch is the runtime selection of which implementation a method call invokes, based on the receiver. Also called *late binding*, it is the mechanism where the association between a call and the code it runs is resolved at runtime rather than at compile time, and it is what makes a single interface invoke different underlying methods depending on the object's actual type ([*Dynamic dispatch*, Wikipedia](https://en.wikipedia.org/wiki/Dynamic_dispatch)). Smalltalk gave the purest form: every call is a *message send*, resolved by a `send` operation that takes the receiver and the message name and, *at call time*, consults the receiver's class method dictionary to find the method to run ([*Dynamic dispatch*, Wikipedia](https://en.wikipedia.org/wiki/Dynamic_dispatch)). Statically-typed object languages — C++, Java, Rust — offer the same late binding for methods marked virtual, dispatching on the single receiver; a few languages (CLOS, Julia) generalize to *multiple dispatch*, choosing on the runtime types of several arguments at once.

### Vtables and method dictionaries

Dynamic dispatch is implemented, in compiled languages, by a *virtual method table*: a per-class array of function pointers, one slot per virtual method, that each object reaches through a hidden vtable pointer. A virtual call loads the vtable pointer from the object, indexes to the method's slot, and calls the function pointer found there — an indirection resolved entirely at runtime ([*Virtual method table*, Wikipedia](https://en.wikipedia.org/wiki/Virtual_method_table)). Rust's own dynamic dispatch works exactly this way: a `dyn Trait` value is a *fat pointer* pairing a data pointer with a vtable pointer, and the vtable is a statically-built table holding the type's destructor, size, and alignment followed by its method pointers, so a call through `dyn Trait` loads the vtable, looks up the method, and calls it with the data pointer ([geo-ant, *Rust Dyn Trait Objects and Fat Pointers*](https://geo-ant.github.io/blog/2023/rust-dyn-trait-objects-fat-pointers/); [*Trait objects*, The Rust Programming Language Book](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)). The cost is real: an indirect call defeats inlining and adds a load per dispatch. Dynamically-typed languages pay even more, resolving a method by name through a dictionary up the class or prototype chain — which is why their implementations invest heavily in *inline caches* and *hidden classes*, techniques pioneered in the Self language's *maps* and *polymorphic inline caches* and inherited by V8 to make repeated same-shape access fast ([*Maps (Hidden Classes) in V8*](https://v8.dev/docs/hidden-classes)). The vtable is the enduring image: a table of function pointers, consulted at runtime, that maps an interface's methods to a type's implementations.

### Prototypal inheritance and delegation

Prototype-based languages share behavior not through classes but by *delegation*: an object holds a link to another object, its prototype, and a message the object does not handle is forwarded to the prototype, walking a chain until the message is answered or the chain ends. This model, introduced by Henry Lieberman's 1986 *Using Prototypical Objects to Implement Shared Behavior in Object-Oriented Systems* and realized in the Self language, needs no classes: an object is a blueprint for others, and behavior is inherited by pointing at it ([Lieberman, *Using Prototypical Objects…*](https://web.media.mit.edu/~lieber/Lieberary/OOP/Delegation/Delegation.html)). JavaScript is its most widely-used descendant — every object has a `[[Prototype]]` link, `Object.create(proto)` sets it, and a property lookup walks the prototype chain, with an own property *shadowing* an inherited one ([MDN, *Inheritance and the prototype chain*](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Inheritance_and_the_prototype_chain)). Lua expresses the same idea through the `__index` metamethod, and a missing method can even be caught by a fallback hook — Ruby's `method_missing`, Python's `__getattr__`, Lua's `__index` function.

The property that distinguishes *delegation* from mere forwarding is the one that matters most for CGP: `self` stays bound to the original receiver. When an object delegates a message to its prototype and the prototype's method refers to `self`, that `self` denotes the object that *originally received* the message, not the prototype the method was found on ([Lieberman, *Using Prototypical Objects…*](https://web.media.mit.edu/~lieber/Lieberary/OOP/Delegation/Delegation.html)). This is what makes delegation a form of inheritance rather than plain message forwarding: the delegate supplies the *behavior*, but the original object remains the *identity* that behavior runs against, so a method inherited from a prototype still sees the receiver's own state. Forwarding, by contrast, rebinds `self` to the object the message was passed to, losing that connection. The delegation-keeps-self-bound rule is the precise hinge on which the CGP comparison turns.

## How CGP expresses it

CGP reproduces dynamic dispatch, vtables, and prototypal delegation, but resolves each at compile time, so what is a runtime lookup in a dynamic language is a type-resolution step in CGP that monomorphizes to a direct call. A [component](../reference/macros/cgp_component.md) is the interface, a [provider](../reference/macros/cgp_impl.md) is an implementation, the context's [wiring table](../reference/macros/delegate_components.md) is its vtable, and [aggregate providers](../concepts/aggregate-providers.md) and [namespaces](../concepts/namespaces.md) are its prototype chain. The correspondence is close construct by construct, and it breaks in exactly one place — everything is static — which is the source of both what CGP gains and what it gives up.

### CGP code reads like a duck-typed program

The most immediate resemblance is stylistic: a CGP provider is written against an unknown context and reads like duck-typed code that sends messages to a receiver and trusts it to respond. A provider calls methods on `self` and reads values from it without ever naming a concrete type or declaring the interface it depends on:

```rust
#[cgp_impl(new GreetHello)]
#[uses(HasName)]
impl Greeter {
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}
```

The body `self.name()` is a message send to a context whose type is not written down, and with an [`#[implicit]`](../reference/attributes/implicit.md) argument the dynamic feel is stronger still — a value simply materializes from the context, as an unbound name would in Ruby or Python:

```rust
#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) -> String {
    format!("Hello, {name}!")
}
```

This is nothing like ordinary statically-typed Rust, where calling `x.name()` demands a concrete `x: Person` or a spelled-out bound `fn greet<C: HasName>(c: &C)` that names the capability in the signature. CGP hides the bound behind [`#[uses]`](../reference/attributes/uses.md) and abstracts the context away, so the provider body carries *less* visible type ceremony than a plain generic function and reads as if it were duck-typed — "assume the context has a `name`, and greeting works." The decisive difference is *when* the trust is discharged. A duck-typed program finds out at runtime whether the object responds, failing with an `AttributeError` or `NoMethodError` if not; CGP finds out at compile time, because the `#[uses(HasName)]` dependency and the field access are checked by trait resolution and by [`check_components!`](../reference/macros/check_components.md) — which is Ruby's `respond_to?` guard moved to compile time and made total. And the field-based half of this is genuinely structural, not merely stylistic: an `#[implicit]` argument or a [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) resolves against *any* context carrying a matching field, the duck-typing-as-a-type-system idea that [row polymorphism](row-polymorphism.md) develops in full. CGP code looks duck-typed on the surface while its resolution stays static and, at the wiring, nominal.

### Static dispatch with the flexibility of dynamic dispatch

CGP delivers the openness of dynamic dispatch — one interface, many implementations, the choice made late — but resolves the choice at compile time and compiles it to a direct call. A caller writes `context.area()` against the `CanCalculateArea` consumer trait without naming an implementation, exactly as a dynamic call names a method and lets the receiver decide; the difference is that CGP's "receiver decides" happens during type checking. The consumer-trait blanket impl generated by [`#[cgp_component]`](../reference/macros/cgp_component.md) routes the call through the context's [`DelegateComponent`](../reference/traits/delegate_component.md) table to the wired provider, and the whole routing is resolved by the compiler and [monomorphized to a direct, zero-cost call](../concepts/coherence.md) — no fat pointer, no vtable load, no indirect jump. Rust already offers *real* dynamic dispatch through `dyn Trait`, and CGP is deliberately its static sibling: both let an implementation be chosen after the calling code is written, one paying a runtime vtable indirection to do it and the other resolving it away entirely. Where a component carries a generic parameter, CGP even reproduces *multiple* dispatch — the [`open` statement](../reference/macros/delegate_components.md) selects a provider per value of a type argument, dispatching on the context *and* that argument the way multimethods dispatch on several runtime types, but decided at compile time.

### `DelegateComponent` is a compile-time vtable

The type-level table a context carries is a vtable that exists only during compilation. Each [`DelegateComponent<Key>`](../reference/traits/delegate_component.md) impl on a context maps one component key to the provider that implements it, precisely as a vtable slot maps a method to its implementation:

```rust
delegate_components! {
    Rectangle {
        AreaCalculatorComponent: RectangleArea,
    }
}

// expands to the type-level table entry:
// impl DelegateComponent<AreaCalculatorComponent> for Rectangle {
//     type Delegate = RectangleArea;
// }
```

The mapping to a vtable is exact on structure and opposite on timing. A context type plays the role of a class, a `DelegateComponent` impl is a vtable slot, and the provider it names is the function that slot points to — but the key is a *type* (the component marker) rather than a method offset, the lookup is performed *once by the compiler* rather than on every call by the CPU, and the table is *erased* before the program runs rather than materialized as data an object points to. This is why the [`delegate_components!`](../reference/macros/delegate_components.md) reference introduces the table as "analogous to a vtable… resolved at compile time" — the analogy is precise, and the qualification is the whole point. The honest cost of resolving the vtable away is that CGP loses the runtime heterogeneity a real vtable enables: a `Vec<Box<dyn CanCalculateArea>>` can hold different concrete shapes and dispatch each at runtime, whereas a CGP context is one monomorphic type resolved once, so mixing implementations in a single collection and choosing between them at runtime is the job of Rust's `dyn`, not of CGP.

### Component delegation is delegation, with `self` bound

CGP's delegation chain is delegation in Lieberman's exact sense, because the context stays bound to the original as lookup walks the chain. An [aggregate provider](../concepts/aggregate-providers.md) bundles a group of component wirings, and a context delegates a whole group to it in one entry, so resolution walks from the context through the bundle to a leaf provider:

```rust
delegate_components! {
    new GeometryComponents {
        AreaCalculatorComponent: RectangleArea,
        PerimeterCalculatorComponent: RectanglePerimeter,
    }
}

delegate_components! {
    Rectangle {
        [AreaCalculatorComponent, PerimeterCalculatorComponent]: GeometryComponents,
    }
}
```

When `rect.area()` resolves, the lookup walks `Rectangle` → `GeometryComponents` → `RectangleArea`, and — this is the crux — the *context stays `Rectangle`* at every step: the [aggregate-providers concept](../concepts/aggregate-providers.md) states outright that "the context argument is `Rectangle` at every step; `GeometryComponents` appears only in the `Self`/delegate position, never as the `Context`," so the leaf provider reads its fields and capabilities from `Rectangle`, the real context, not from the bundle. That is delegation's defining property precisely: the delegate chain supplies the *behavior* while `self` — the context — remains the original *identity* that behavior runs against, never rebound to the prototype the method was found on. The [`UseContext`](../reference/providers/use_context.md) provider is the same relationship pointed the other way, letting a provider route a call back to the context's own wiring — the prototype consulting its delegator. CGP's delegation is not analogous to prototypal delegation; on the axis that separates delegation from forwarding, it *is* delegation, resolved statically.

### Namespaces are prototypal inheritance with override

A CGP namespace is a prototype: a reusable object of default wirings that a context inherits and then selectively shadows. A context joins a namespace and inherits every entry it does not wire directly, and a directly-wired entry on the context *wins* over the inherited one — own-property-shadows-prototype, expressed at the type level ([namespaces concept](../concepts/namespaces.md)):

```rust
delegate_components! {
    App {
        namespace DefaultNamespace;   // inherit the namespace's wirings as defaults

        // a directly-wired entry here shadows the namespace's entry for that key
    }
}
```

Namespaces inherit from one another exactly as prototypes chain into further prototypes, so a base namespace can be extended into a richer one that contexts downstream pick up:

```rust
cgp_namespace! {
    new ExtendedNamespace: DefaultNamespace {
        @cgp.core.error => @app,
    }
}
```

`ExtendedNamespace` resolves everything `DefaultNamespace` does plus its own entries, the prototype-of-a-prototype relationship at the wiring level. The lookup that walks these layers is the [`RedirectLookup`](../reference/providers/redirect_lookup.md) provider tracing a type-level path, which is the prototype chain being walked — and the redirect that fires when a key is not directly present is the compile-time analogue of a `__index` or `method_missing` fallback. As with everything else in CGP, the chain is walked by trait resolution during compilation rather than by property lookup at runtime, so the inheritance is real but pays nothing at runtime and cannot be mutated once the program is built.

## What users like and dislike

Dynamic dispatch and dynamic typing are loved for the immediacy and flexibility they give, and the praise is consistent across their communities. Duck typing lets a function work over any object that responds to the messages it sends, which programmers value for *flexibility and reuse* — no interface to declare, no hierarchy to fit into — and for *cleaner, shorter code* and *rapid prototyping*, the reasons Ruby and Python developers reach for it ([SitePoint, *Making Ruby Quack*](https://www.sitepoint.com/making-ruby-quack-why-we-love-duck-typing/); [GeeksforGeeks, *Type Systems*](https://www.geeksforgeeks.org/python/type-systemsdynamic-typing-static-typing-duck-typing/)). Dynamic dispatch is what makes polymorphism and open extension work, and prototypal inheritance is prized for its malleability — objects can be created, linked, and modified at runtime, behavior can be shared without a class hierarchy, and a running program can be reshaped live. Metaprogramming hooks like `method_missing` and `__getattr__` let a single object answer messages it was never written to handle, which powers proxies, DSLs, and ORMs.

The complaints are the mirror image of that flexibility, and they cluster on safety and cost. Because type checks are deferred, a mistake surfaces as a *runtime error* — an `AttributeError`, a `NoMethodError`, an "undefined is not a function" — discovered only when the offending line runs, which makes refactoring hazardous and large systems harder to trust; the standard mitigation is to add tests, type annotations, or a `respond_to?` guard before the call ([DevGex, *Duck Typing*](https://devgex.com/en/article/00035033); [SitePoint](https://www.sitepoint.com/making-ruby-quack-why-we-love-duck-typing/)). Dynamic dispatch has a *performance cost* — the vtable indirection defeats inlining, and dictionary-based lookup in dynamic languages is worse, which is why so much engineering goes into inline caches and hidden classes to claw it back ([V8, *Hidden Classes*](https://v8.dev/docs/hidden-classes)). Prototypal inheritance draws its own specific gripes: a mutable prototype chain is easy to get confused about, and JavaScript's `this` binding — the very self-binding that makes delegation work — is a notorious source of bugs when a method is detached from its receiver. And tooling suffers throughout, since an IDE cannot reliably know what a value responds to when its type is not fixed.

## How CGP compares

CGP takes the static end of every axis these mechanisms define, which is its central trade: it keeps the openness and gives up the runtime. On *dispatch*, CGP resolves at compile time and monomorphizes where a dynamic language resolves at runtime, so there is no vtable, no method-dictionary lookup, and no indirect call — a wired call is a direct one, and nothing about it survives into the running program. On *typing*, CGP writes duck-typed-looking provider code but checks it statically, so the missing-method failure that a dynamic language hits at runtime is a compile error at the wiring site, caught by `check_components!` rather than by a production stack trace. On *inheritance*, CGP's delegation and namespace chains are walked by the type checker, not by runtime property lookup, so they cost nothing and cannot go wrong through a mis-set prototype or a detached `this`. Each side pays for what the other gets. Dynamic languages get runtime malleability — heterogeneous collections, plugins loaded at startup, live redefinition, metaprogramming — and pay with runtime errors and dispatch overhead; CGP gets zero-cost dispatch and static guarantees and pays by fixing the wiring at compile time, so nothing about it can change while the program runs.

The costs on CGP's side are worth naming plainly, because they are exactly the capabilities dynamic dispatch exists to provide. CGP cannot hold a heterogeneous collection of implementations and choose among them at runtime — that is what Rust's own `dyn Trait` is for, and a program that needs it should reach for a trait object, not for CGP. It cannot load an implementation chosen at runtime from configuration or a plugin file, monkey-patch a live object, or synthesize a response to a message it was not built to handle, because there is no runtime object graph to mutate and no runtime dispatch to intercept. And its errors, though caught early, are the verbose generated-type messages the [check traits](../concepts/check-traits.md) exist to localize, rather than the direct `NoMethodError` a dynamic language reports.

Neither is uniformly better, and the honest positioning names where each wins. When a program needs runtime openness — plugins discovered at startup, objects of mixed types in one collection, behavior reshaped live, or the metaprogramming that proxies and DSLs rely on — dynamic dispatch is the right tool, and emulating it with CGP's compile-time wiring is not merely awkward but impossible, since the whole point is deferral to runtime. When a program's set of implementations is known at build time and it wants the decoupling of dynamic dispatch with none of the cost or the runtime failure modes — polymorphism that inlines, duck-typed ergonomics that cannot throw `NoMethodError`, and inheritance that the compiler resolves — CGP delivers that on stable Rust, and does so as ordinary types with no runtime machinery at all.

## Presenting CGP to someone who knows this

A reader who thinks in objects, messages, and prototypes holds most of CGP's structure already, and the way in is to map the vocabulary and then flag the one change. A **context is an object**, a **component is a message or method**, a **provider is a method implementation**, **wiring is the object's method table**, [`DelegateComponent`](../reference/traits/delegate_component.md) **is its vtable**, an **aggregate provider or namespace is a prototype** the object delegates to, and [`check_components!`](../reference/macros/check_components.md) **is the guarantee that the object responds to every message it will be sent** — `respond_to?` made total and moved to compile time. Reading a provider body, which sends messages to a context it never names, is reading duck-typed code. Framed this way, CGP is not a foreign paradigm to this reader but the mechanisms they already use — dispatch, vtables, delegation, duck typing — with the runtime taken out.

The single expectation to correct, from which everything else follows, is that any of this happens at runtime. This reader will assume a vtable is a data structure the program carries, that dispatch chooses at the moment of the call, that a prototype chain is walked when a property is missing, and that the object graph can change while the program runs — and in CGP none of that is so. The table is erased after compilation, the dispatch is resolved by the type checker and inlined, the chain is walked during type resolution, and the wiring is fixed once the program is built. Present "late binding" honestly: in CGP the binding is late to the *wiring site* — the place a context declares its providers — not to runtime, so the flexibility is real but it is spent at compile time. The pitch that lands for this audience is the pair of things they most wish their own tools had: *duck typing that cannot blow up at runtime*, because a context missing a capability is a compile error rather than a 2 a.m. `NoMethodError`, and *dynamic dispatch that costs nothing*, because the polymorphism they rely on is resolved away instead of paid for on every call. For the prototype-minded reader specifically, the resonant framing is that CGP's delegation keeps `self` bound to the original context exactly as theirs keeps `this` bound to the receiver — same inheritance-by-delegation, but checked and free.

The analogy to avoid is promising runtime behavior CGP does not have. A reader sold on heterogeneous collections, runtime plugins, or monkey-patching will feel misled the first time they reach for them and find the wiring frozen at compile time; say plainly that those live on the runtime side of the line, where Rust's `dyn Trait` and the dynamic languages themselves remain the right tools. Sell CGP as what it is — the static resolution of the mechanisms they know, trading runtime malleability for zero cost and compile-time safety — and the reader who has debugged a `this`-binding bug or chased a `NoMethodError` through a plugin will hear the trade as a good one.

## Sources

The account of the related work draws on standard references for dynamic dispatch and object models, the primary literature on prototype-based programming, and cited community writing for sentiment; the CGP snippets are drawn from the knowledge base's [aggregate providers](../concepts/aggregate-providers.md), [namespaces](../concepts/namespaces.md), and [consumer/provider](../concepts/consumer-and-provider-traits.md) material and verified against current macro behavior.

- [*Dynamic dispatch* (Wikipedia)](https://en.wikipedia.org/wiki/Dynamic_dispatch) and [*Virtual method table* (Wikipedia)](https://en.wikipedia.org/wiki/Virtual_method_table) — late binding as runtime resolution of a call, Smalltalk's message-send `send`, single versus multiple dispatch, and the per-class vtable of function pointers reached through an object's vtable pointer.
- [*Duck typing* (Wikipedia)](https://en.wikipedia.org/wiki/Duck_typing) — an object's usability decided by the methods and properties it has rather than a declared class or interface.
- [geo-ant, *Rust Dyn Trait Objects and Fat Pointers*](https://geo-ant.github.io/blog/2023/rust-dyn-trait-objects-fat-pointers/) and [*Trait objects*, The Rust Programming Language Book](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) — Rust's own dynamic dispatch as a data-pointer/vtable-pointer fat pointer, and the vtable's contents, the runtime counterpart to CGP's compile-time table.
- [Lieberman, *Using Prototypical Objects to Implement Shared Behavior in Object-Oriented Systems* (OOPSLA 1986)](https://web.media.mit.edu/~lieber/Lieberary/OOP/Delegation/Delegation.html) — delegation as forwarding unhandled messages to a prototype, and the defining rule that `self` stays bound to the original receiver, which distinguishes delegation from forwarding and matches CGP's context binding.
- [MDN, *Inheritance and the prototype chain*](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Inheritance_and_the_prototype_chain) — JavaScript's `[[Prototype]]` link, `Object.create`, prototype-chain lookup, and own-property shadowing, the runtime form of namespace inherit-and-override.
- [*Maps (Hidden Classes) in V8*](https://v8.dev/docs/hidden-classes) — hidden classes and inline caches descending from the Self language's maps and polymorphic inline caches, the engineering that dynamic dispatch's runtime cost demands.
- [SitePoint, *Making Ruby Quack — Why We Love Duck Typing*](https://www.sitepoint.com/making-ruby-quack-why-we-love-duck-typing/), [GeeksforGeeks, *Type Systems: Dynamic, Static & Duck Typing*](https://www.geeksforgeeks.org/python/type-systemsdynamic-typing-static-typing-duck-typing/), and [DevGex, *Duck Typing*](https://devgex.com/en/article/00035033) — community sentiment on what duck typing and dynamic typing buy (flexibility, brevity, rapid development) and cost (runtime errors, the `respond_to?` guard, refactoring hazards).
