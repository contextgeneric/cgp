# Profile picture lookup

This example fetches a user's profile picture — a real-world operation that queries a database for a user record and, if one is set, downloads and decodes the image from object storage. It progresses from two field-driven async functions to a fully wired application that swaps its database engine and its storage backend per context without touching the orchestration logic. It is a template for any use case where one business operation composes several infrastructure steps, each of which may have more than one implementation.

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- context-generic functions — [`#[cgp_fn]`](../reference/macros/cgp_fn.md) with [implicit arguments](../concepts/implicit-arguments.md)
- async methods in traits — [`#[async_trait]`](../reference/macros/async_trait.md)
- composing capabilities — [`#[uses]`](../reference/attributes/uses.md)
- field access on contexts — [`#[derive(HasField)]`](../reference/derives/derive_has_field.md)
- impl-only generic parameters — [`#[impl_generics]`](../reference/macros/cgp_fn.md)
- components and named providers — [`#[cgp_component]`](../reference/macros/cgp_component.md), [`#[cgp_impl]`](../reference/macros/cgp_impl.md), and the [consumer/provider trait duality](../concepts/consumer-and-provider-traits.md)
- wiring a context to providers — [`delegate_components!`](../reference/macros/delegate_components.md)

All snippets assume `use cgp::prelude::*;`. The operation works over two domain types — a `UserId` newtype and a `User` row that may carry the storage key of a profile picture:

```rust
pub struct UserId(pub u64);

#[derive(sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub email: String,
    pub profile_picture_object_id: Option<String>,
}
```

## Field-driven steps

The two infrastructure steps are each a function that reads its connection from the context's fields rather than from explicit arguments. Marking a function with [`#[cgp_fn]`](../reference/macros/cgp_fn.md) and tagging a parameter [`#[implicit]`](../reference/attributes/implicit.md) turns it into a method on any context that carries a matching field; the remaining parameters stay as ordinary arguments the caller supplies. Both functions are async, so each also carries [`#[async_trait]`](../reference/macros/async_trait.md), which keeps the generated trait's async method lint-clean:

```rust
#[cgp_fn]
#[async_trait]
pub async fn get_user(
    &self,
    #[implicit] database: &PgPool,
    user_id: &UserId,
) -> anyhow::Result<User> {
    let user =
        sqlx::query_as("SELECT name, email, profile_picture_object_id FROM users WHERE id = $1")
            .bind(user_id.0 as i64)
            .fetch_one(database)
            .await?;

    Ok(user)
}

#[cgp_fn]
#[async_trait]
pub async fn fetch_storage_object(
    &self,
    #[implicit] storage_client: &Client,
    #[implicit] bucket_id: &str,
    object_id: &str,
) -> anyhow::Result<Vec<u8>> {
    let output = storage_client
        .get_object()
        .bucket(bucket_id)
        .key(object_id)
        .send()
        .await?;

    let data = output.body.collect().await?.into_bytes().to_vec();
    Ok(data)
}
```

`get_user` reads one implicit field, `database`; `fetch_storage_object` reads two, `storage_client` and `bucket_id`, in addition to its explicit `object_id`. A context becomes eligible for each function purely by deriving [`HasField`](../reference/derives/derive_has_field.md) for fields whose names and types match the implicit arguments — no per-context implementation is written:

```rust
#[derive(HasField)]
pub struct App {
    pub database: PgPool,
    pub storage_client: Client,
    pub bucket_id: String,
}

#[derive(HasField)]
pub struct MinimalApp {
    pub database: PgPool,
}
```

`App` has all three fields, so it can call both functions; `MinimalApp` has only `database`, so it can call `get_user` but the compiler refuses any call to `fetch_storage_object` on it.

## Composing the steps

The orchestration is itself a `#[cgp_fn]` that calls the two steps as methods on `self`. Because those calls are CGP capabilities rather than inherent methods, the function declares them with [`#[uses]`](../reference/attributes/uses.md), which adds each as a hidden bound on the context instead of a visible parameter:

```rust
#[cgp_fn]
#[async_trait]
#[uses(GetUser, FetchStorageObject)]
pub async fn get_user_profile_picture(
    &self,
    user_id: &UserId,
) -> anyhow::Result<Option<RgbImage>> {
    let user = self.get_user(user_id).await?;

    if let Some(object_id) = user.profile_picture_object_id {
        let data = self.fetch_storage_object(&object_id).await?;
        let image = image::load_from_memory(&data)?.to_rgb8();

        Ok(Some(image))
    } else {
        Ok(None)
    }
}
```

The names in `#[uses(GetUser, FetchStorageObject)]` are the traits `#[cgp_fn]` derives from the two step functions — a function `foo_bar` generates a trait `FooBar`. The body reads as plain method calls; `#[uses]` threads the trait bounds behind the scenes so that only a context carrying every required field gains `get_user_profile_picture`. `App` does; `MinimalApp` does not, and the gap is a compile error rather than a runtime failure.

## Varying the database engine

`get_user` above hardcodes `&PgPool`, which ties it to PostgreSQL. To let one implementation serve several database engines, introduce a type parameter that lives on the impl alone — never on the generated trait or its callers — with [`#[impl_generics]`](../reference/macros/cgp_fn.md). The implicit `database` field becomes `&Pool<Db>`, and the `where` clause carries the engine-specific bounds:

```rust
#[cgp_fn]
#[async_trait]
#[impl_generics(Db: Database)]
pub async fn get_user(
    &self,
    #[implicit] database: &Pool<Db>,
    user_id: &UserId,
) -> anyhow::Result<User>
where
    i64: sqlx::Type<Db>,
    for<'a> User: sqlx::FromRow<'a, Db::Row>,
    for<'a> i64: sqlx::Encode<'a, Db>,
    for<'a> <Db as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, Db>,
    for<'a> &'a mut <Db as sqlx::Database>::Connection: sqlx::Executor<'a, Database = Db>,
{
    let user =
        sqlx::query_as("SELECT name, email, profile_picture_object_id FROM users WHERE id = $1")
            .bind(user_id.0 as i64)
            .fetch_one(database)
            .await?;

    Ok(user)
}
```

Each context supplies `Db` implicitly through the type of its `database` field. An `App` carrying a `PgPool` resolves `Db = Postgres`, while an embedded context carrying a `SqlitePool` resolves `Db = Sqlite` — `SqlitePool` is `Pool<Sqlite>` under the hood — and both satisfy the sqlx bounds independently:

```rust
#[derive(HasField)]
pub struct EmbeddedApp {
    pub database: SqlitePool,
    pub storage_client: Client,
    pub bucket_id: String,
}
```

`get_user_profile_picture` is unchanged: it depends on the `GetUser` capability, not on which engine satisfies it, so the same orchestration now runs on PostgreSQL and SQLite alike.

## Varying the storage backend

A single `#[cgp_fn]` defines exactly one implementation, so it cannot offer the storage fetch in more than one flavor. When a step needs interchangeable implementations — say Amazon S3 in one deployment and Google Cloud Storage in another — promote it to a [component](../concepts/consumer-and-provider-traits.md) with [`#[cgp_component]`](../reference/macros/cgp_component.md). The annotated `CanFetchStorageObject` trait is the *consumer trait* callers use; the `StorageObjectFetcher` argument names the generated *provider trait* that implementations target:

```rust
#[async_trait]
#[cgp_component(StorageObjectFetcher)]
pub trait CanFetchStorageObject {
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
}
```

Each backend is a *named provider* written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md). Unlike a blanket `impl`, named providers may overlap freely, and `#[implicit]` works inside them exactly as in `#[cgp_fn]`, so each provider reads whatever connection field its backend needs:

```rust
#[cgp_impl(new FetchS3Object)]
impl StorageObjectFetcher {
    async fn fetch_storage_object(
        &self,
        #[implicit] storage_client: &Client,
        #[implicit] bucket_id: &str,
        object_id: &str,
    ) -> anyhow::Result<Vec<u8>> {
        let output = storage_client
            .get_object()
            .bucket(bucket_id)
            .key(object_id)
            .send()
            .await?;

        let data = output.body.collect().await?.into_bytes().to_vec();
        Ok(data)
    }
}

#[cgp_impl(new FetchGCloudObject)]
impl StorageObjectFetcher {
    async fn fetch_storage_object(
        &self,
        #[implicit] storage_client: &Storage,
        #[implicit] bucket_id: &str,
        object_id: &str,
    ) -> anyhow::Result<Vec<u8>> {
        let mut reader = storage_client
            .read_object(bucket_id, object_id)
            .send()
            .await?;

        let mut contents = Vec::new();
        while let Some(chunk) = reader.next().await.transpose()? {
            contents.extend_from_slice(&chunk);
        }

        Ok(contents)
    }
}
```

`FetchS3Object` reads an `aws_sdk_s3::Client` from its `storage_client` field; `FetchGCloudObject` reads a `google_cloud_storage::client::Storage` from the same field name. The `new` keyword in each attribute defines the provider struct in place.

The orchestration now imports the storage step by its *consumer* trait name, so it depends on the capability rather than on any one backend:

```rust
#[cgp_fn]
#[async_trait]
#[uses(GetUser, CanFetchStorageObject)]
pub async fn get_user_profile_picture(
    &self,
    user_id: &UserId,
) -> anyhow::Result<Option<RgbImage>> {
    let user = self.get_user(user_id).await?;

    if let Some(object_id) = user.profile_picture_object_id {
        let data = self.fetch_storage_object(&object_id).await?;
        let image = image::load_from_memory(&data)?.to_rgb8();

        Ok(Some(image))
    } else {
        Ok(None)
    }
}
```

## Wiring contexts to backends

Defining a provider does not attach it to any context; a context chooses its provider by wiring with [`delegate_components!`](../reference/macros/delegate_components.md). Each entry maps the component — keyed by its generated `…Component` name — to the provider that implements it for that context. An `App` carrying an S3 client wires to `FetchS3Object`, while a `GCloudApp` carrying a GCloud client wires to `FetchGCloudObject`:

```rust
#[derive(HasField)]
pub struct GCloudApp {
    pub database: PgPool,
    pub storage_client: Storage,
    pub bucket_id: String,
}

delegate_components! {
    App {
        StorageObjectFetcherComponent: FetchS3Object,
    }
}

delegate_components! {
    GCloudApp {
        StorageObjectFetcherComponent: FetchGCloudObject,
    }
}
```

Both contexts remain plain data structs with `#[derive(HasField)]`; all backend selection happens in the wiring block, resolved at compile time with no runtime dispatch. The S3 binary contains only the S3 code path and the GCloud binary only the GCloud one. Adding a third backend — Azure Blob Storage, say — is a new `#[cgp_impl(new FetchAzureObject)]` provider and one more `delegate_components!` entry; `get_user`, `get_user_profile_picture`, and every existing context stay untouched.
</content>
</invoke>
