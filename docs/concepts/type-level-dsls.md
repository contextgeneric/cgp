# Type-level DSLs

A type-level DSL encodes a small language as Rust *types* and interprets those types at compile time through CGP wiring, so that a program's syntax is decoupled from its semantics and each can vary independently.

## The idea

The pattern is to represent a program as a type rather than a value, and to make interpreting that program a matter of resolving a CGP component for it. A program fragment becomes a phantom struct that carries no data — it names an operation — and the meaning of that operation is supplied by whichever provider a context wires for it. Because the program is a type, the Rust trait system *is* the interpreter: type checking the call that runs the program is what selects and composes the per-fragment implementations. There is no runtime parser, no abstract-syntax-tree walk, and no dispatch loop; the whole interpretation collapses into trait resolution that the compiler performs once.

What makes this worth doing in CGP specifically is the same property that motivates CGP everywhere else: the separation of an interface from its implementations, lifted here to the separation of *syntax* from *semantics*. A fragment of syntax like a "run this command" marker says nothing about how the command runs — on which async runtime, against which error type, producing which output type. All of that is decided by the provider the context binds to that fragment, so a single program runs differently under different contexts, and a context can replace one fragment's interpreter without disturbing any program that uses it. This is the type-level analogue of a tagless-final or finally-encoded interpreter, but with CGP's extra `Context` parameter threading dependency injection through every fragment.

The pattern composes from constructs documented individually elsewhere; the [shell-scripting DSL example](../examples/shell-scripting-dsl.md) develops a complete instance end to end, and this document explains the shape those pieces form. The running illustrations here use that example's vocabulary — `SimpleExec`, `Pipe`, the `Handler` component — so that the concepts and the worked code share one set of names.

## Abstract syntax as phantom types

Each piece of the language's grammar is a zero-data struct whose type parameters carry the program's structure. A "run a command" fragment is nothing more than its phantom marker:

```rust
pub struct SimpleExec<Path, Args>(pub PhantomData<(Path, Args)>);
```

The struct has no methods and no interpreter attached — it exists only to be named in a type and matched on later. Its parameters are themselves type-level data: a command path, an argument list, a request method. Composite fragments nest the same way, so a pipeline of stages is a single type built from a [`Product!`](../reference/macros/product.md) type-level list of stage types, and a literal string inside a program becomes a [`Symbol!`](../reference/macros/symbol.md) type-level string because a `&str` value cannot appear where only types are allowed. A whole program is therefore one large type assembled from these markers — the language's *abstract syntax*, deliberately holding no information about how any fragment behaves.

## A computation component as the interpreter interface

The interpreter is a single CGP component whose `Code` type parameter is the program fragment being run. The [handler family](handlers.md) is built for exactly this: each component threads a phantom `Code` tag alongside an `Input`, and produces an associated `Output`. The general member, [`Handler`](../reference/components/handler.md), is async and fallible:

```rust
#[async_trait]
#[cgp_component(Handler)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType::Error)]
pub trait CanHandle<Code, Input> {
    type Output;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Error>;
}
```

Running a program is then one call to the consumer method, passing the program type as `PhantomData<Code>`. The `Code` tag carries no value precisely so that one context can host an interpreter for every fragment in the language, each keyed by a distinct `Code` type, and so that the wiring can dispatch on it. A DSL whose fragments never fail or never await can use a simpler member of the family — [`Computer`](../reference/components/computer.md) for pure synchronous fragments — and rely on the family's promotion combinators to lift it where a more capable interpreter is required; the choice of component is the choice of what capabilities the language's fragments are allowed to have.

## Providers as interpreters

A provider interprets one fragment by pattern-matching on the `Code` parameter through its generic arguments. Written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), an interpreter reads like an ordinary method body while the context stays generic, and it states everything the fragment needs from the context as [impl-side dependencies](impl-side-dependencies.md) — capability bounds through [`#[uses(...)]`](../reference/attributes/uses.md) and the remaining structural bounds in the `where` clause:

```rust
#[cgp_impl(new HandleStreamChecksum)]
#[uses(CanRaiseError<Input::Error>)]
#[use_type(HasErrorType::Error)]
impl<Input, Hasher> Handler<Checksum<Hasher>, Input>
where
    Input: Unpin + TryStream,
    Hasher: Digest,
    Input::Ok: AsRef<[u8]>,
{
    type Output = GenericArray<u8, Hasher::OutputSize>;

    async fn handle(/* ... */) -> Result<Self::Output, Error> {
        /* fold the stream into a digest */
    }
}
```

Matching `Handler<Checksum<Hasher>, …>` for any `Hasher` lets the one provider cover an entire family of fragments, and because the provider never names a concrete error or runtime type it interprets the same fragment under any context that can satisfy its bounds. This is where syntax and semantics finally meet: the fragment `Checksum<Hasher>` says *what*, the provider says *how*, and the binding between them is made only at wiring time. Swapping the provider re-interprets every program that uses the fragment, without editing a single program.

## Assembling an interpreter by dispatching on the syntax

A whole-language interpreter is assembled by routing each `Code` to its own provider, which is what [`UseDelegate`](../reference/providers/use_delegate.md) does. Because the handler components are declared `#[derive_delegate(UseDelegate<Code>)]`, a context can resolve the handler component through an inner type-level table keyed on the `Code` type, in the manner described in [dispatching](dispatching.md). Each entry maps a fragment type — its generic parameters bound by a leading `<…>` so the key matches every use — to the provider that interprets it:

```rust
@HandlerComponent.<Path, Args> SimpleExec<Path, Args>:
    HandleSimpleExec,
@HandlerComponent.<Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>:
    HandleSimpleHttpRequest,
```

Resolving the interpreter for a program then walks one extra step through this table: the context dispatches the handler component on the fragment type and finds its provider. Because the keys are types rather than values, a single entry can capture generic structure that a value-level lookup table never could, and the per-fragment providers stay completely independent of one another — adding a fragment is adding a row.

## Composing and extending the language

Two further constructs turn a set of fragment interpreters into a real, extensible language. Composition of fragments is itself interpreted by a provider: a pipeline fragment is wired to a combinator like [`PipeHandlers`](../reference/providers/handler_combinators.md), which threads the output of one stage's interpreter into the input of the next, so that a compound program is interpreted by composing the interpreters of its parts. Extensibility comes from [namespaces](namespaces.md): the table of fragment-to-provider wirings is captured once with [`cgp_namespace!`](../reference/macros/cgp_namespace.md), and a context joins it through [`delegate_components!`](../reference/macros/delegate_components.md) to inherit the whole language at once. A language *extension* is then a new namespace that inherits the base and adds rows for new fragments and their interpreters — an independent crate can extend the DSL without patching, forking, or even depending on the parts of the core it does not use. This is the type-level counterpart of an interpreter that anyone can add cases to from outside.

## A surface-syntax layer

A procedural macro can sit on top of the abstract syntax to give the language a readable surface, but it is a strictly optional convenience. Such a macro performs only shallow token rewriting — turning an infix pipe operator into a nested pipeline type, a bracketed list into a `Product!`, a string literal into a `Symbol!` — and emits the same plain type a programmer could write by hand. Because the surface macro produces ordinary types and never touches interpretation, it carries no semantics of its own: the language remains fully usable, and fully extensible, without it. Keeping the surface layer this thin is what lets the *grammar* evolve independently of both the macro and the interpreters.

## Tradeoffs

Interpreting at compile time is the pattern's strength and its limit at once. The payoff is that a program runs at native speed with no interpreter overhead, that the Rust type system checks a program's well-formedness before it ever runs, and that the same program retargets to a different context for free. The cost is that a program must be known at compile time, so the technique does not suit languages loaded dynamically at runtime — configuration read from a file, user-supplied scripts, plugins — without also shipping a compiler. The other practical cost is diagnostics: a type error in one fragment surfaces as a trait-resolution failure that can be verbose and indirect, which the [check traits](check-traits.md) help localize during development but do not entirely tame. The pattern fits best where the language is fixed at build time and the value of zero-cost, type-checked, retargetable programs outweighs the friction of compile-time error messages.

## Related constructs

A type-level DSL is an assembly of constructs documented on their own. The interpreter interface is a member of the [handler family](handlers.md) — usually [`Handler`](../reference/components/handler.md) or [`Computer`](../reference/components/computer.md) — which rests on the [consumer/provider trait duality](consumer-and-provider-traits.md). Fragment interpreters are providers written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md) that draw their needs from the context as [impl-side dependencies](impl-side-dependencies.md), and a fragment parameterized by an abstract type uses [abstract types](abstract-types.md) to stay decoupled from concrete ones. The abstract syntax is built from type-level data — [`Product!`](../reference/macros/product.md) lists and [`Symbol!`](../reference/macros/symbol.md) strings — and dispatched on with [`UseDelegate`](../reference/providers/use_delegate.md) per [dispatching](dispatching.md). Composition uses the [handler combinators](../reference/providers/handler_combinators.md), and the language is bundled and extended through [namespaces](namespaces.md) defined with [`cgp_namespace!`](../reference/macros/cgp_namespace.md) and joined with [`delegate_components!`](../reference/macros/delegate_components.md). The [shell-scripting DSL example](../examples/shell-scripting-dsl.md) is a complete worked instance of the whole pattern.
