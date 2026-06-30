# Shell-scripting DSL

This example builds a type-level shell-scripting DSL whose programs are ordinary Rust *types*, interpreted at compile time by whichever context runs them. It progresses from a fixed CLI program, through the component that interprets a program and the namespace that wires the interpreters, to a custom context that supplies runtime values and a language extension that adds new syntax — and is a template for any embedded DSL where the program's *syntax* should be decoupled from its *semantics* so that each can vary independently. The general pattern it instantiates is described in [type-level DSLs](../concepts/type-level-dsls.md).

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- the computation component the DSL is built on — [`Handler` / `CanHandle`](../reference/components/handler.md) in the [handler family](../concepts/handlers.md)
- writing a provider that interprets one syntax — [`#[cgp_impl]`](../reference/macros/cgp_impl.md) with a pattern-matched `Code` parameter
- raising source errors into the context's abstract error — [`CanRaiseError`](../reference/components/can_raise_error.md)
- dispatching on the program type — [`UseDelegate`](../reference/providers/use_delegate.md) and [dispatching](../concepts/dispatching.md)
- bundling and inheriting wiring — [namespaces](../concepts/namespaces.md) with [`cgp_namespace!`](../reference/macros/cgp_namespace.md)
- joining a namespace from a context — [`delegate_components!`](../reference/macros/delegate_components.md) and [`#[derive(HasField)]`](../reference/derives/derive_has_field.md)
- composing handlers into a pipeline provider — [`PipeHandlers`](../reference/providers/handler_combinators.md)

All snippets assume `use cgp::prelude::*;`, and the handler items come from `cgp::extra::handler`.

## A program is a type

The smallest DSL program is a type, not a value. The `hypershell!` macro provides shell-like surface syntax — a pipe operator and bracketed argument lists — that desugars into a chain of phantom-typed syntax markers. This program runs `echo hello world!` and streams the result to standard output:

```rust
pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithStaticArgs["hello", "world!"],
        >
    |   StreamToStdout
};
```

The macro is sugar only: the `|` desugars into a `Pipe<Product![...]>` of handler syntaxes, the bracketed `[...]` shorthand into a `<Product![...]>` type-level list, and each string literal into a `Symbol!` type-level string, since a program lives entirely at the type level where a `&str` value cannot appear. The program above expands to exactly this plain type:

```rust
pub type Program = Pipe<Product![
    SimpleExec<
        StaticArg<Symbol!("echo")>,
        WithStaticArgs<Product![Symbol!("hello"), Symbol!("world!")]>,
    >,
    StreamToStdout,
]>;
```

The macro is entirely optional — writing this type by hand is equivalent. The program carries no data; it is a description of *what* to do, with the *how* supplied separately by the context that runs it.

A context runs a program by calling `handle`, passing the program as a `PhantomData` type argument and an input value. `HypershellCli` is a predefined empty context for CLI-only programs, so it can be constructed directly:

```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
    HypershellCli
        .handle(PhantomData::<Program>, Vec::new())
        .await?;

    Ok(())
}
```

The `Vec::new()` is the program's standard input, which `echo` ignores. Running it prints `hello world!`.

## The component that interprets a program

Every step of a program is interpreted by one component: the [`Handler`](../reference/components/handler.md) component, the async, fallible corner of the [handler family](../concepts/handlers.md). Its consumer trait `CanHandle` is what `handle` above resolves to:

```rust
#[async_trait]
#[cgp_component(Handler)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanHandle<Code, Input>: HasErrorType {
    type Output;

    async fn handle(
        &self,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Error>;
}
```

The `Code` parameter is the program fragment being interpreted, `Input` is the data flowing in (a process's standard input, an HTTP body), and the associated `Output` is what the fragment produces. The phantom `PhantomData<Code>` is how a *type* is passed where a value is expected: one context hosts many handlers, each keyed by a distinct `Code` tag, and the wiring dispatches on that tag. Because the program is just a `Code` type, interpreting it is a matter of resolving `CanHandle` for that type.

## Abstract syntax, decoupled from its meaning

A syntax marker like `SimpleExec` is nothing but a phantom struct — it has no interpreter attached:

```rust
pub struct SimpleExec<Path, Args>(pub PhantomData<(Path, Args)>);
```

This is the whole point of the design: how a program is *written* is completely decoupled from how it is *interpreted*. `SimpleExec` names a piece of syntax; what it *does* is decided entirely by which provider a context wires for the `Code = SimpleExec<…>` case. Swapping that provider — to run commands on a different async runtime, say — changes the program's meaning without touching a single program that uses `SimpleExec`. The syntax is the DSL's abstract grammar; the providers are its interpreters.

## Interpreting one syntax with a provider

An interpreter is a `Handler` provider that pattern-matches on a single syntax type through its `Code` parameter. Written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), it reads like an ordinary method implementation while `Context` stays generic. This provider interprets a `Checksum<Hasher>` syntax by consuming a stream of bytes and producing their digest:

```rust
pub struct Checksum<Hasher>(pub PhantomData<Hasher>);

#[cgp_impl(new HandleStreamChecksum)]
impl<Context, Input, Hasher> Handler<Checksum<Hasher>, Input> for Context
where
    Context: CanRaiseError<Input::Error>,
    Input: Unpin + TryStream,
    Hasher: Digest,
    Input::Ok: AsRef<[u8]>,
{
    type Output = GenericArray<u8, Hasher::OutputSize>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Checksum<Hasher>>,
        mut input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let mut hasher = Hasher::new();

        while let Some(bytes) = input.try_next().await.map_err(Context::raise_error)? {
            hasher.update(bytes);
        }

        Ok(hasher.finalize())
    }
}
```

Two things make this provider reusable across contexts. It matches `Handler<Checksum<Hasher>, …>` for *any* `Hasher` that implements `Digest`, so the one provider covers every hash algorithm. And it never names a concrete error type: the `CanRaiseError<Input::Error>` bound lets it convert a stream error into the context's own abstract error via [`CanRaiseError`](../reference/components/can_raise_error.md), so a context using `anyhow`, `eyre`, or a bespoke error type all reuse the same code. The `where` clause states everything the provider needs from the context as an [impl-side dependency](../concepts/impl-side-dependencies.md); a context that cannot meet it simply cannot wire this provider.

## Dispatching on the program

A single provider only interprets one syntax, so something must route each `Code` to its interpreter. That is the job of [`UseDelegate`](../reference/providers/use_delegate.md): because the `Handler` component is declared `#[derive_delegate(UseDelegate<Code>)]`, the `HandlerComponent` lookup can be keyed on the `Code` type through an inner type-level table, as described in [dispatching](../concepts/dispatching.md). A wiring entry therefore reaches *past* the component name to the `Code` it handles — written as a dotted path under `HandlerComponent`, with the syntax's generic parameters bound by a leading `<…>`:

```rust
@HandlerComponent.<Path, Args> SimpleExec<Path, Args>:
    HandleSimpleExec,
@HandlerComponent.<Path, Args> StreamingExec<Path, Args>:
    HandleStreamingExec,
```

The keys are *types*, not values, which is what lets a lookup capture generic parameters like `Path` and `Args` and still match every use of `SimpleExec`. Resolving `CanHandle<SimpleExec<…>, …>` for a context then walks one extra step through the `Code` table: `HandlerComponent` dispatches on `SimpleExec<…>` to `HandleSimpleExec`.

## Bundling the wiring into a namespace

A real DSL has many syntaxes wired to many providers, and several contexts that should share that wiring. A [namespace](../concepts/namespaces.md) captures the whole table once and lets contexts inherit it. `HypershellNamespace` is defined with [`cgp_namespace!`](../reference/macros/cgp_namespace.md), inheriting CGP's built-in `DefaultNamespace` and adding entries that map groups of syntaxes to their interpreters under the `HandlerComponent` path:

```rust
cgp_namespace! {
    new HypershellNamespace: DefaultNamespace {
        @cgp.core.error.ErrorTypeProviderComponent:
            UseAnyhowError,

        @cgp.extra.handler.HandlerComponent.[
            <Path, Args> SimpleExec<Path, Args>,
            <Path, Args> StreamingExec<Path, Args>,
            StreamToStdout,
        ]:
            HypershellTokioProvider,

        @cgp.extra.handler.HandlerComponent.[
            <Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>,
            <Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>,
        ]:
            HypershellReqwestProvider,
    }
}
```

Each entry is keyed by a *path* — a dotted sequence written with the `@` sigil, like `@cgp.extra.handler.HandlerComponent` — rather than a bare component name, which is what lets a namespace inherit another's entries and lets a context override a single one without disturbing the rest. The array form groups several `Code` keys that share one provider. Because the wiring is expressed through trait resolution, all of this dispatch is resolved at compile time with no runtime indirection.

## Running on a custom context

A program that reads runtime values — a URL, a name — needs a context that carries them. Deriving [`HasField`](../reference/derives/derive_has_field.md) exposes a struct's fields so that syntaxes like `FieldArg<"name">` can read them, and joining `HypershellNamespace` inside [`delegate_components!`](../reference/macros/delegate_components.md) inherits every interpreter at once:

```rust
pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithArgs[
                StaticArg<"Hello,">,
                FieldArg<"name">,
            ],
        >
    |   StreamToStdout
};

#[derive(HasField)]
pub struct MyApp {
    pub name: String,
}

delegate_components! {
    MyApp {
        namespace HypershellNamespace;
    }
}
```

The `namespace HypershellNamespace;` header makes every lookup `MyApp` cannot resolve directly fall back to the namespace's entries, so `MyApp` interprets the full DSL without restating any wiring. `WithArgs` mixes static arguments with `FieldArg<"name">`, which resolves through `HasField` to the context's `name` field. Constructing `MyApp { name: "Alice".into() }` and calling `handle` prints `Hello, Alice`. The predefined `HypershellCli` is wired the same way with an empty struct — it joins `HypershellNamespace` and adds nothing.

## Extending the DSL

A language extension adds new syntax and its interpreters without forking the core. Define the new syntax markers, write providers for them, and publish a namespace that inherits `HypershellNamespace` and layers the new entries on top. Here a `Checksum<Hasher>` syntax (interpreted by `HandleStreamChecksum` from earlier) and a `BytesToHex` syntax join the language:

```rust
pub struct BytesToHex;

#[cgp_impl(new HandleBytesToHex)]
impl<Context, Code, Input> Handler<Code, Input> for Context
where
    Context: HasErrorType,
    Input: AsRef<[u8]>,
{
    type Output = String;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<String, Context::Error> {
        Ok(hex::encode(input))
    }
}

cgp_namespace! {
    new HypershellChecksumNamespace: HypershellNamespace {
        @cgp.extra.handler.HandlerComponent.<Hasher> Checksum<Hasher>:
            PipeHandlers<Product![
                HandleToFuturesStream,
                HandleStreamChecksum,
            ]>,

        @cgp.extra.handler.HandlerComponent.BytesToHex:
            HandleBytesToHex,
    }
}
```

`HandleBytesToHex` interprets *any* `Code` whose `Input` is byte-like — it does not inspect the tag — showing that a provider can be as generic as its logic allows. The `Checksum` entry wires not a single provider but a [`PipeHandlers`](../reference/providers/handler_combinators.md) composition: `HandleToFuturesStream` adapts the incoming stream into the `TryStream` that `HandleStreamChecksum` expects, and the two run as one handler. `HypershellChecksumNamespace` inherits everything `HypershellNamespace` resolves and adds the two new keys, so a context joining it speaks the extended language:

```rust
pub type Program = hypershell! {
        StreamingHttpRequest<GetMethod, FieldArg<"url">, WithHeaders[]>
    |   Checksum<Sha256>
    |   BytesToHex
    |   StreamToStdout
};

#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url: String,
}

delegate_components! {
    MyApp {
        namespace HypershellChecksumNamespace;
    }
}
```

The program fetches a URL, hashes the streamed response natively, hex-encodes the digest, and prints it — and the only change from joining the core namespace is the namespace name. Because a namespace is just one more entry in an inheritance chain, an extension composes onto the language the same way a context composes onto a namespace, with the whole resolution settled at compile time.
