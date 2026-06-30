# Social media app

This example builds the CRUD backend for a small social media service — managing users and posts — and follows it as the wiring grows from a handful of components into something a real application would have. It progresses from one coarse manager trait per domain, through fine-grained per-operation traits and a higher-order provider that adds input filtering, to provider bundles and finally namespace-grouped wiring that keeps the top-level configuration short even as the component count climbs. It is a template for any application whose component count grows past the point where a flat delegation table stays readable.

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- consumer/provider trait pairs — [`#[cgp_component]`](../reference/macros/cgp_component.md) and [consumer and provider traits](../concepts/consumer-and-provider-traits.md)
- providers that read context fields as method arguments — [`#[cgp_impl]`](../reference/macros/cgp_impl.md) with [`#[implicit]`](../reference/attributes/implicit.md) arguments backed by [`#[derive(HasField)]`](../reference/derives/derive_has_field.md)
- importing a capability a provider depends on — [`#[uses]`](../reference/attributes/uses.md), an [impl-side dependency](../concepts/impl-side-dependencies.md)
- a provider that wraps another provider — [higher-order providers](../concepts/higher-order-providers.md) and [`#[use_provider]`](../reference/attributes/use_provider.md)
- wiring a context and bundling providers into reusable groups — [`delegate_components!`](../reference/macros/delegate_components.md)
- grouping component keys so a context inherits a whole bundle at once — [namespaces](../concepts/namespaces.md), the [`#[prefix(...)]`](../reference/macros/cgp_component.md) attribute, and [`cgp_namespace!`](../reference/macros/cgp_namespace.md)
- checking that a wiring is complete — [`check_components!`](../reference/macros/check_components.md)

All snippets assume `use cgp::prelude::*;` and share a small set of domain types — the entities the service manipulates and the database handle the providers read:

```rust
pub struct Email(pub String);

pub struct User;
pub struct UserData;
pub struct UserId;

pub struct Post;
pub struct PostId;

pub enum Error {
    InvalidUsername,
    InvalidMessage,
}

pub struct PostgresDb;

#[derive(PartialOrd, PartialEq)]
pub struct Probability(pub f64);

impl Probability {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}
```

## One manager per domain

The first cut gives each domain a single manager trait that exposes all of its operations. A `UserManager` covers the user lifecycle and a `PostManager` the post lifecycle:

```rust
#[cgp_component(UserManager)]
pub trait CanManageUser {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error>;

    fn get_user(&self, user_id: &UserId) -> Result<User, Error>;

    fn update_user_data(&self, user_id: &UserId, user_data: &UserData) -> Result<(), Error>;
}

#[cgp_component(PostManager)]
pub trait CanManagePost {
    fn create_post(&self, title: &str, content: &str) -> Result<Post, Error>;

    fn get_post(&self, post_id: &PostId) -> Result<Post, Error>;

    fn update_post(&self, post_id: &PostId, content: &str) -> Result<(), Error>;

    fn delete_post(&self, post_id: &PostId) -> Result<(), Error>;
}
```

Each trait is an ordinary trait turned into a CGP component by [`#[cgp_component]`](../reference/macros/cgp_component.md), which names the provider trait (`UserManager`, `PostManager`) that implementers write against. The service also needs two content-safety checks — rejecting banned usernames and spam posts — so those become components of their own, each returning a `Probability` the managers can threshold:

```rust
#[cgp_component(UsernameCensor)]
pub trait CanCensorUsername {
    fn username_is_censored(&self, username: &str) -> Probability;
}

#[cgp_component(SpamMessageDetector)]
pub trait CanDetectSpamMessage {
    fn message_is_spam(&self, message: &str) -> Probability;
}
```

The production manager talks to PostgreSQL. Its provider reads the database handle straight out of the context as an [`#[implicit]`](../reference/attributes/implicit.md) argument — CGP extracts the `database` field from the context rather than threading it through every call site — and pulls in `CanCensorUsername` with [`#[uses]`](../reference/attributes/uses.md) so `create_user` can reject censored names before writing:

```rust
#[cgp_impl(new PostgresUserManager)]
#[uses(CanCensorUsername)]
impl UserManager {
    fn create_user(
        &self,
        #[implicit] database: &PostgresDb,
        username: &str,
        email: &Email,
    ) -> Result<User, Error> {
        if self.username_is_censored(username) > Probability::new(0.8) {
            return Err(Error::InvalidUsername);
        }

        // ... insert the user with `database`
    }

    fn get_user(&self, #[implicit] database: &PostgresDb, user_id: &UserId) -> Result<User, Error> {
        // ...
    }

    fn update_user_data(
        &self,
        #[implicit] database: &PostgresDb,
        user_id: &UserId,
        user_data: &UserData,
    ) -> Result<(), Error> {
        // ...
    }
}
```

`PostgresUserManager` is a provider written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), so its body reads like methods on the context while staying generic over any context that supplies a `database` field and a `CanCensorUsername` implementation. The `PostManager` provider follows the same shape, thresholding `CanDetectSpamMessage` inside `create_post`. The content checks themselves get cheap stand-in providers for now — `DummyUserCensor` and `DummySpamMessageDetector` — that always pass.

A concrete context ties it together. `ProductionApp` holds the database handle and names a provider for every component in one [`delegate_components!`](../reference/macros/delegate_components.md) table:

```rust
#[derive(HasField)]
pub struct ProductionApp {
    pub database: PostgresDb,
}

delegate_components! {
    ProductionApp {
        UserManagerComponent: PostgresUserManager,
        PostManagerComponent: PostgresPostManager,
        UsernameCensorComponent: DummyUserCensor,
        SpamMessageDetectorComponent: DummySpamMessageDetector,
    }
}
```

The [`#[derive(HasField)]`](../reference/derives/derive_has_field.md) is what makes the `#[implicit] database` arguments resolve, by exposing the `database` field for [`HasField`](../reference/traits/has_field.md) lookup. This wiring is compact, but it hides a strain that grows with the application: `PostgresUserManager` depends on `CanCensorUsername` even though only `create_user` uses it, so `get_user` and `update_user_data` carry a dependency they never touch. As managers accumulate methods, these spurious dependencies pile up, and there is no way to grant `create_user` more capabilities without granting them to the whole manager.

## One trait per operation

Breaking each manager into a trait per operation lets every provider declare exactly the dependencies its one method needs. The user manager becomes three fine-grained components:

```rust
#[cgp_component(UserCreator)]
pub trait CanCreateUser {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error>;
}

#[cgp_component(UserGetter)]
pub trait CanGetUser {
    fn get_user(&self, user_id: &UserId) -> Result<User, Error>;
}

#[cgp_component(UserUpdater)]
pub trait CanUpdateUser {
    fn update_user_data(&self, user_id: &UserId, user_data: &UserData) -> Result<(), Error>;
}
```

The post manager splits the same way into `PostCreator`, `PostGetter`, `PostUpdater`, and `PostDeleter`. Each operation now has its own provider, and the providers that do not need a content check no longer mention one. `GetUserWithPostgres` reads only the database; `CreateUserWithPostgres` adds nothing else either, because — as the next step shows — the censoring will move out of it entirely:

```rust
#[cgp_impl(new CreateUserWithPostgres)]
impl UserCreator {
    fn create_user(
        &self,
        #[implicit] database: &PostgresDb,
        username: &str,
        email: &Email,
    ) -> Result<User, Error> {
        // ...
    }
}

#[cgp_impl(new GetUserWithPostgres)]
impl UserGetter {
    fn get_user(&self, #[implicit] database: &PostgresDb, user_id: &UserId) -> Result<User, Error> {
        // ...
    }
}
```

Splitting the traits also makes capability isolation possible: because deleting a post is its own `PostDeleter` component, code that should only read and write posts can be given the getter and updater without ever receiving the destructive `delete_post`. The price is that there are now seven CRUD components plus two content-safety ones to wire, where before there were four.

## Lifting the filter into a higher-order provider

With creation isolated in its own trait, the username check no longer belongs inside the database provider — it can become a separate provider that wraps any user creator. `FilterCensoredUsername` is a [higher-order provider](../concepts/higher-order-providers.md): it takes an inner `UserCreator` as a type parameter, runs the censor check, and forwards to the inner provider only if the name is allowed:

```rust
#[cgp_impl(new FilterCensoredUsername<InnerCreator>)]
#[uses(CanCensorUsername)]
#[use_provider(InnerCreator: UserCreator)]
impl<InnerCreator> UserCreator {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error> {
        if self.username_is_censored(username) > Probability::new(0.8) {
            return Err(Error::InvalidUsername);
        }

        InnerCreator::create_user(self, username, email)
    }
}
```

The [`#[use_provider]`](../reference/attributes/use_provider.md) attribute is what keeps the inner type readable: `InnerCreator: UserCreator` declares that `InnerCreator` must itself be a user-creation provider for this context, and `InnerCreator::create_user(self, …)` dispatches through it. `FilterCensoredUsername` knows nothing about databases, and `CreateUserWithPostgres` now knows nothing about censoring — the two compose only when wired together as `FilterCensoredUsername<CreateUserWithPostgres>`. Because the wrapper is generic, the same filter applies to any creator: a SQLite-backed `CreateUserWithSqlite`, were one added, would gain censoring through `FilterCensoredUsername<CreateUserWithSqlite>` with no new code. The post side gets the matching `FilterSpamMessage<InnerCreator>` wrapper around any post creator.

## Grouping providers into bundles

A [`delegate_components!`](../reference/macros/delegate_components.md) table with `new` defines a standalone provider — a bundle — whose only job is to hold a sub-table other contexts can reuse as a unit. Grouping the user providers, the post providers, and the AI-backed content filters each into their own bundle keeps related wiring together:

```rust
delegate_components! {
    new PostgresUserComponents {
        UserCreatorComponent:
            FilterCensoredUsername<CreateUserWithPostgres>,
        UserGetterComponent:
            GetUserWithPostgres,
        UserUpdaterComponent:
            UpdateUserWithPostgres,
    }
}

delegate_components! {
    new PostgresPostComponents {
        PostCreatorComponent:
            FilterSpamMessage<CreatePostWithPostgres>,
        PostGetterComponent:
            GetPostWithPostgres,
        PostUpdaterComponent:
            UpdatePostWithPostgres,
        PostDeleterComponent:
            DeletePostWithPostgres,
    }
}

delegate_components! {
    new AiContentFilterComponents {
        UsernameCensorComponent:
            AiUserCensor,
        SpamMessageDetectorComponent:
            AiSpamMessageDetector,
    }
}
```

The top-level context then forwards each component to the bundle that owns it. Using the array-key form of `delegate_components!`, several keys can share one bundle in a single entry:

```rust
delegate_components! {
    ProductionApp {
        [
            UserCreatorComponent,
            UserGetterComponent,
            UserUpdaterComponent,
        ]:
            PostgresUserComponents,
        [
            PostCreatorComponent,
            PostGetterComponent,
            PostUpdaterComponent,
            PostDeleterComponent,
        ]:
            PostgresPostComponents,
        [
            UsernameCensorComponent,
            SpamMessageDetectorComponent,
        ]:
            AiContentFilterComponents,
    }
}
```

The bundles read cleanly on their own, but the top-level table still has to spell out every component name to route it to a bundle. The grouping exists in the bundle definitions, yet the context cannot refer to "all the user components" as one thing — it must list them.

## Grouping component keys with namespaces

A [namespace](../concepts/namespaces.md) gives a group of components a shared key, so a context can route the whole group with one entry instead of listing each name. A component joins a namespace under a dotted path with the [`#[prefix(...)]`](../reference/macros/cgp_component.md) attribute on its trait; here the three user components all register under `@app.core.user` in the built-in `DefaultNamespace`:

```rust
#[cgp_component(UserCreator)]
#[prefix(@app.core.user in DefaultNamespace)]
pub trait CanCreateUser {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error>;
}

#[cgp_component(UserGetter)]
#[prefix(@app.core.user in DefaultNamespace)]
pub trait CanGetUser {
    fn get_user(&self, user_id: &UserId) -> Result<User, Error>;
}

#[cgp_component(UserUpdater)]
#[prefix(@app.core.user in DefaultNamespace)]
pub trait CanUpdateUser {
    fn update_user_data(&self, user_id: &UserId, user_data: &UserData) -> Result<(), Error>;
}
```

The post components register under `@app.core.post` the same way, and the two content-safety components under `@app.extra.content_filter`. A context opts into the namespace with a `namespace` header line in its delegation table, after which a path key stands in for every component registered under it:

```rust
delegate_components! {
    ProductionApp {
        namespace DefaultNamespace;

        @app.core.user: PostgresUserComponents,
        @app.core.post: PostgresPostComponents,
        @app.extra.content_filter: AiContentFilterComponents,
    }
}
```

The `namespace DefaultNamespace;` line tells `ProductionApp` to resolve lookups through the namespace, and `@app.core.user: PostgresUserComponents` forwards every component sitting under that path to the bundle in one entry. The top-level table now has three lines regardless of how many operations the user and post domains grow to, because adding a method means adding a `#[prefix]`-tagged component to an existing path, not a new line at the top level. Listing the bundled components in a [`check_components!`](../reference/macros/check_components.md) block confirms the namespace wiring actually resolves each one:

```rust
check_components! {
    ProductionApp {
        UserCreatorComponent,
        UserGetterComponent,
        UserUpdaterComponent,
        PostCreatorComponent,
        PostGetterComponent,
        PostUpdaterComponent,
        PostDeleterComponent,
        UsernameCensorComponent,
        SpamMessageDetectorComponent,
    }
}
```

## Hierarchical grouping and context variants

Because the prefixes form a hierarchy, a parent path can group its children, so the wiring can be collapsed another level. Both `@app.core.user` and `@app.core.post` sit under `@app.core`, which lets an intermediary bundle gather all the core CRUD wiring behind a single path, with the extras gathered the same way:

```rust
delegate_components! {
    new PostgresCoreComponents {
        namespace DefaultNamespace;

        @app.core.user: PostgresUserComponents,
        @app.core.post: PostgresPostComponents,
    }
}

delegate_components! {
    new ProductionExtraComponents {
        namespace DefaultNamespace;

        @app.extra.content_filter: AiContentFilterComponents,
    }
}
```

The top-level context drops to two entries — its essential core behavior and its swappable extras:

```rust
delegate_components! {
    ProductionApp {
        namespace DefaultNamespace;

        @app.core: PostgresCoreComponents,
        @app.extra: ProductionExtraComponents,
    }
}
```

The real payoff is that contexts now differ at a glance. A test context reuses the same PostgreSQL-backed core but swaps the AI content filters for dummy ones, so its full definition makes the single difference obvious:

```rust
delegate_components! {
    new DummyExtraComponents {
        namespace DefaultNamespace;

        @app.extra.content_filter: DummyContentFilterComponents,
    }
}

delegate_components! {
    TestApp {
        namespace DefaultNamespace;

        @app.core: PostgresCoreComponents,
        @app.extra: DummyExtraComponents,
    }
}
```

`ProductionApp` and `TestApp` share `@app.core` and differ only in `@app.extra`, so a reader sees immediately that the two run identical core logic and diverge only in their content filtering — a comparison that, with the flat per-component table, would mean scanning a dozen entries that may not even appear in the same order. A local-first variant would follow the same shape, keeping the production extras but pointing `@app.core` at an SQLite core bundle, and the one swapped line would again be the whole story.
