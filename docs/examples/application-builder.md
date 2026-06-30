# Application builder

This example assembles an application context — a struct holding a database pool, an HTTP client, and an AI agent — from independent builder providers that each construct one subsystem and know nothing of the final struct or of each other. It progresses from a hand-written constructor that grows unmanageably, through a builder provider per subsystem, to a builder context that merges them all, and finally to swapping subsystems and producing several application variants from one builder. It is a template for any use case where a context is configured from independently-evolving parts that should compose without a central constructor.

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- assembling a struct from independent contributions — [extensible records](../concepts/extensible-records.md) and the [extensible builder pattern](../concepts/dispatching.md)
- a struct that can be built field by field and merged — [`#[derive(BuildField)]`](../reference/derives/derive_build_field.md) with [`#[derive(HasFields)]`](../reference/derives/derive_has_fields.md), and `build_from` from [casting](../reference/traits/cast.md)
- each subsystem builder is a handler — [`Handler` / `CanHandle`](../reference/components/handler.md) in the [handler family](../concepts/handlers.md)
- writing a builder provider — [`#[cgp_impl]`](../reference/macros/cgp_impl.md) reading config through [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md)
- the abstract error and raising into it — [`HasErrorType`](../reference/components/has_error_type.md) and [`CanRaiseError`](../reference/components/can_raise_error.md)
- merging builder outputs into the target — [`BuildAndMergeOutputs`](../reference/providers/dispatch_combinators.md)
- wiring and checking a context — [`delegate_components!`](../reference/macros/delegate_components.md) and [`check_components!`](../reference/macros/check_components.md)
- choosing among build targets at the type level — [`UseDelegate`](../reference/providers/use_delegate.md)

All snippets assume `use cgp::prelude::*;`, the handler items come from `cgp::extra::handler`, and the builder dispatcher from `cgp::extra::dispatch`.

## The constructor that grows

The application context is an ordinary struct, and the direct way to build it is a constructor that initializes every field at once:

```rust
#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub sqlite_pool: SqlitePool,
    pub http_client: Client,
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

impl App {
    pub async fn new(
        db_options: &str,
        db_journal_mode: &str,
        http_user_agent: &str,
        open_ai_key: &str,
        open_ai_model: &str,
        llm_preamble: &str,
    ) -> Result<Self, Error> {
        let journal_mode = SqliteJournalMode::from_str(db_journal_mode)?;
        let db_options = SqliteConnectOptions::from_str(db_options)?.journal_mode(journal_mode);
        let sqlite_pool = SqlitePool::connect_with(db_options).await?;

        let http_client = Client::builder()
            .user_agent(http_user_agent)
            .connect_timeout(Duration::from_secs(5))
            .build()?;

        let open_ai_client = openai::Client::new(open_ai_key);
        let open_ai_agent = open_ai_client.agent(open_ai_model).preamble(llm_preamble).build();

        Ok(Self { sqlite_pool, http_client, open_ai_client, open_ai_agent })
    }
}
```

Every subsystem's setup lives in one function, so each new field widens the parameter list and every team touches the same constructor. The deriving line is the first move away from that: `App` derives [`#[derive(HasFields)]`](../reference/derives/derive_has_fields.md) and [`#[derive(BuildField)]`](../reference/derives/derive_build_field.md), which expose it as a [product of named fields](../concepts/extensible-records.md) and generate a partial-record builder, so the struct can be filled in field by field instead of all at once.

## A builder provider for one subsystem

Each subsystem becomes its own provider that constructs a small output struct. The SQLite builder reads its configuration from the context and produces a `SqliteClient`:

```rust
#[cgp_auto_getter]
pub trait HasSqliteOptions {
    fn db_options(&self) -> &str;
    fn db_journal_mode(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct SqliteClient {
    pub sqlite_pool: SqlitePool,
}

#[cgp_impl(new BuildSqliteClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasSqliteOptions + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Self::Error> {
        let journal_mode =
            SqliteJournalMode::from_str(self.db_journal_mode()).map_err(Self::raise_error)?;
        let db_options = SqliteConnectOptions::from_str(self.db_options())
            .map_err(Self::raise_error)?
            .journal_mode(journal_mode);
        let sqlite_pool = SqlitePool::connect_with(db_options)
            .await
            .map_err(Self::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}
```

`BuildSqliteClient` is a [`Handler`](../reference/components/handler.md) provider written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), so it reads like a method on the builder context while staying generic over it. It does not know the final `App` type — only that its context can supply SQLite options (through the [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) getter `HasSqliteOptions`, satisfied by any context with the matching fields) and can raise an `sqlx::Error` into its own abstract error via [`CanRaiseError`](../reference/components/can_raise_error.md). Its output `SqliteClient` derives the same record traits as `App`, which is what lets its single field be merged into the larger struct later.

The other subsystems follow the same shape. The HTTP and OpenAI builders each read their own config and produce their own output struct:

```rust
#[cgp_auto_getter]
pub trait HasHttpClientConfig {
    fn http_user_agent(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct HttpClient {
    pub http_client: Client,
}

#[cgp_impl(new BuildHttpClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasHttpClientConfig + CanRaiseError<reqwest::Error>,
{
    type Output = HttpClient;

    async fn handle(&self, _code: PhantomData<Code>, _input: Input) -> Result<Self::Output, Self::Error> {
        let http_client = Client::builder()
            .user_agent(self.http_user_agent())
            .connect_timeout(Duration::from_secs(5))
            .build()
            .map_err(Self::raise_error)?;

        Ok(HttpClient { http_client })
    }
}
```

```rust
#[cgp_auto_getter]
pub trait HasOpenAiConfig {
    fn open_ai_key(&self) -> &str;
    fn open_ai_model(&self) -> &str;
    fn llm_preamble(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct OpenAiClient {
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

#[cgp_impl(new BuildOpenAiClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasOpenAiConfig + HasErrorType,
{
    type Output = OpenAiClient;

    async fn handle(&self, _code: PhantomData<Code>, _input: Input) -> Result<Self::Output, Self::Error> {
        let open_ai_client = openai::Client::new(self.open_ai_key());
        let open_ai_agent = open_ai_client
            .agent(self.open_ai_model())
            .preamble(self.llm_preamble())
            .build();

        Ok(OpenAiClient { open_ai_client, open_ai_agent })
    }
}
```

Each provider states exactly what it needs from the context in its `where` clause as an [impl-side dependency](../concepts/impl-side-dependencies.md), and nothing more. A builder whose construction cannot fail — like `BuildOpenAiClient` — only requires [`HasErrorType`](../reference/components/has_error_type.md) so its output type aligns with the others, while a fallible one adds the `CanRaiseError` it needs.

## Merging the outputs into the application

The builder context names the configuration fields and wires the providers together. It is a plain struct that derives [`HasField`](../reference/derives/derive_has_field.md) — which makes the getters above resolve against its fields — and wires the `HandlerComponent` to [`BuildAndMergeOutputs`](../reference/providers/dispatch_combinators.md):

```rust
#[derive(HasField, Deserialize)]
pub struct FullAppBuilder {
    pub db_options: String,
    pub db_journal_mode: String,
    pub http_user_agent: String,
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub llm_preamble: String,
}

delegate_components! {
    FullAppBuilder {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildSqliteClient,
                    BuildHttpClient,
                    BuildOpenAiClient,
                ]>,
    }
}

check_components! {
    FullAppBuilder {
        HandlerComponent: ((), ()),
    }
}
```

`BuildAndMergeOutputs<App, Product![...]>` is the heart of the [extensible builder pattern](../concepts/extensible-records.md): it starts an empty `App` builder, runs each provider in the list, merges each output struct into the builder with `build_from` from [casting](../reference/traits/cast.md), and finalizes the complete `App`. The merge is name-driven, so `SqliteClient`'s `sqlite_pool` field lands in `App`'s `sqlite_pool` field with no conversion written. The error wiring picks `anyhow::Error` as the abstract error through `UseAnyhowError` and lets the source errors raise into it through `RaiseAnyhowError`, satisfying the `CanRaiseError` bounds the providers declared. The [`check_components!`](../reference/macros/check_components.md) block asserts at compile time that the handler is wired for the unit `Code` and `Input` the build is invoked with — if any provider's required field were missing from `FullAppBuilder`, this would fail to compile rather than at runtime.

Building the `App` is then one call. The builder is constructed from its config fields — or deserialized from a file, since it derives `Deserialize` — and `handle` runs the whole pipeline:

```rust
pub async fn main() -> Result<(), Error> {
    let builder = FullAppBuilder {
        db_options: "file:./db.sqlite".to_owned(),
        db_journal_mode: "WAL".to_owned(),
        http_user_agent: "SUPER_AI_AGENT".to_owned(),
        open_ai_key: "1234567890".to_owned(),
        open_ai_model: "gpt-4o".to_owned(),
        llm_preamble: "You are a helpful assistant".to_owned(),
    };

    let _app = builder.handle(PhantomData::<()>, ()).await?;

    Ok(())
}
```

The `PhantomData::<()>` is the `Code` and the `()` the `Input`; neither is constrained here, so the unit type stands in for both.

## Swapping a subsystem

Because each builder is decoupled from the target struct, replacing one subsystem reuses everything else. An enterprise variant that uses Postgres instead of SQLite needs a new output struct and builder, plus a target struct that holds a `PgPool`:

```rust
#[cgp_auto_getter]
pub trait HasPostgresUrl {
    fn postgres_url(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct PostgresClient {
    pub postgres_pool: PgPool,
}

#[cgp_impl(new BuildPostgresClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasPostgresUrl + CanRaiseError<sqlx::Error>,
{
    type Output = PostgresClient;

    async fn handle(&self, _code: PhantomData<Code>, _input: Input) -> Result<Self::Output, Self::Error> {
        let postgres_pool = PgPool::connect(self.postgres_url()).await.map_err(Self::raise_error)?;
        Ok(PostgresClient { postgres_pool })
    }
}
```

The new builder context swaps `BuildPostgresClient` in for `BuildSqliteClient` and reuses the HTTP and OpenAI builders unchanged:

```rust
#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub postgres_pool: PgPool,
    pub http_client: Client,
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

#[derive(HasField, Deserialize)]
pub struct AppBuilder {
    pub postgres_url: String,
    pub http_user_agent: String,
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub llm_preamble: String,
}

delegate_components! {
    AppBuilder {
        ErrorTypeProviderComponent: UseAnyhowError,
        ErrorRaiserComponent: RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildPostgresClient,
                    BuildHttpClient,
                    BuildOpenAiClient,
                ]>,
    }
}
```

Unlike feature flags, which force an either/or split at compile time, the SQLite and Postgres variants can coexist in the same codebase and even compile together, which keeps both paths tested.

## Many applications from one builder

A single builder can produce several application variants by dispatching on its `Code` parameter. Given a builder whose fields cover the needs of every variant, marker types name the build targets and [`UseDelegate`](../reference/providers/use_delegate.md) routes each one to its own provider list:

```rust
pub struct BuildChatGptApp;
pub struct BuildAnthropicApp;
pub struct BuildAnthropicAndChatGptApp;

delegate_components! {
    AnthropicAndChatGptAppBuilder {
        ErrorTypeProviderComponent: UseAnyhowError,
        ErrorRaiserComponent: RaiseAnyhowError,
        HandlerComponent:
            UseDelegate<new BuilderHandlers {
                BuildChatGptApp:
                    BuildAndMergeOutputs<App, Product![
                        BuildSqliteClient, BuildHttpClient, BuildOpenAiClient,
                    ]>,
                BuildAnthropicApp:
                    BuildAndMergeOutputs<AnthropicApp, Product![
                        BuildSqliteClient, BuildHttpClient, BuildDefaultAnthropicClient,
                    ]>,
                BuildAnthropicAndChatGptApp:
                    BuildAndMergeOutputs<AnthropicAndChatGptApp, Product![
                        BuildSqliteClient, BuildHttpClient, BuildDefaultAnthropicClient, BuildOpenAiClient,
                    ]>,
            }>,
    }
}
```

Each marker selects a different target struct and provider list, and the `llm_preamble` field is shared by both the Anthropic and OpenAI builders with no coordination — a value-level dependency injected once and read by every provider that needs it. Choosing a variant is then a matter of which `Code` is passed to `handle`:

```rust
let chat_gpt_app: App = builder.handle(PhantomData::<BuildChatGptApp>, ()).await?;
let anthropic_app: AnthropicApp = builder.handle(PhantomData::<BuildAnthropicApp>, ()).await?;
let combined_app: AnthropicAndChatGptApp =
    builder.handle(PhantomData::<BuildAnthropicAndChatGptApp>, ()).await?;
```

The same builder, the same config, three different application contexts — selected by type, dispatched at compile time, with the builder pipeline for each one assembled from the same decoupled providers.
