# Dependency injection

Dependency injection (DI) is the practice of giving an object its collaborators from the outside instead of letting it construct them, and the frameworks built around it — Spring, Guice, Dagger, and their kin — automate the wiring so a large application's object graph assembles itself. CGP solves the same decoupling problem, but at compile time and without a container, so it is the concept a reader with an enterprise background is most likely to reach for when they first meet CGP wiring.

## Purpose

Every non-trivial program has to decide where its components get the things they depend on, and dependency injection is the answer that keeps those decisions out of the components themselves. A class that needs a database, an HTTP client, and a logger can create them in its constructor — hard-coding the concrete types — or it can accept them as parameters and let whoever builds it supply them. The second choice is dependency injection, and its payoff is decoupling: the class names only the *interfaces* it needs, so a test can pass fakes, a different deployment can pass different implementations, and the class never changes. The cost is that something has to do the supplying, and in a large graph that "something" is elaborate enough that frameworks exist to run it.

This is exactly the problem CGP's wiring addresses, which is why the comparison matters. A DI framework assembles an object graph by matching each dependency to a provider; CGP assembles a context by matching each component to a provider. The vocabulary rhymes, the goal is identical — decouple what a piece of code *needs* from what supplies it — and the differences are all in the mechanism: a DI framework resolves the graph at runtime from reflection and configuration, while CGP resolves it at compile time through the trait system. A reader who understands why DI decouples code already understands why CGP does; the work is in showing them what changes when the resolution moves from runtime to types.

## The concept in depth

Dependency injection is a specific form of *inversion of control*: rather than a component reaching out to fetch its dependencies, control is inverted so the dependencies are handed to it. The idea predates any framework and is expressible in plain code — pass collaborators as constructor arguments — but the frameworks are what most practitioners mean by "DI," because they automate the assembly. The sections below cover the container-and-annotation model that Spring popularized, the module-and-binding model of Guice and Dagger, and the plain-code form that Rust already encourages.

### The IoC container and beans (Spring)

Spring's core is an *inversion-of-control container*: an object, the `ApplicationContext`, that instantiates, configures, and connects the application's objects — its *beans* — and manages their lifecycles. A class is marked as a bean with an annotation such as `@Component` or `@Service`, and the container discovers it by scanning the classpath. The container owns every bean it creates, so the application asks the container for a fully-assembled object rather than constructing one itself.

```java
@Service
public class UserService {
    // business logic lives here
}
```

The container is configured either by annotation-driven scanning, as above, or explicitly with a `@Configuration` class whose `@Bean` methods return the objects to manage. The explicit form is where a bean's construction is spelled out, including which implementation stands in for an interface:

```java
@Configuration
public class AppConfig {
    @Bean
    public StorageClient storageClient() {
        return new S3StorageClient(/* ... */);
    }
}
```

### Constructor, setter, and field injection

Once the container holds a set of beans, it injects each bean's dependencies by one of three mechanisms, and the choice among them is the most-discussed decision in day-to-day Spring. **Constructor injection** passes dependencies as constructor arguments, which makes them required and lets the field be `final`:

```java
@Service
public class ProfilePictureService {
    private final StorageClient storage;
    private final UserRepository users;

    public ProfilePictureService(StorageClient storage, UserRepository users) {
        this.storage = storage;
        this.users = users;
    }
}
```

**Setter injection** supplies a dependency through a setter after construction, which suits genuinely optional collaborators, and **field injection** writes the dependency straight into a private field by reflection, marked with `@Autowired`:

```java
@Service
public class ProfilePictureService {
    @Autowired private StorageClient storage;   // field injection
    @Autowired private UserRepository users;
}
```

The `@Autowired` annotation is the instruction to resolve a dependency by type: the container finds the one bean assignable to `StorageClient` and injects it. When a single constructor is present, Spring treats it as autowired without the annotation, which is why modern Spring code favors constructor injection and reserves `@Autowired` for the ambiguous or field-injected cases. The Spring team and the wider community now recommend constructor injection for all required dependencies, because it makes a class's dependencies explicit in its signature, keeps them non-null, and lets the object be built without a container in a test.

### Modules and bindings (Guice and Dagger)

Guice and Dagger express the same wiring not by classpath scanning but by explicit *bindings* declared in a *module*. A Guice module maps an interface to an implementation, and the injector resolves a request for the interface to the bound implementation:

```java
public class StorageModule extends AbstractModule {
    @Override
    protected void configure() {
        bind(StorageClient.class).to(S3StorageClient.class);
    }
}
```

A class then requests its dependencies by annotating its constructor with `@Inject`, and the injector supplies them from the bindings. Dagger uses the same `@Inject`/module vocabulary but resolves the graph at *compile time*: its annotation processor generates the wiring code during the build, so a missing or ambiguous binding is a compile error and there is no reflection at runtime. This compile-time-versus-runtime split is the sharpest axis of variation among DI frameworks — Spring and Guice resolve bindings at runtime through reflection, Dagger resolves them at compile time through generated code — and it is the axis along which CGP sits firmly at the compile-time end.

### Dependency injection without a framework (Rust)

Rust practitioners generally hold that the language needs no DI framework, because traits and generics already provide the decoupling a container is built to deliver. A function that needs a capability takes a generic parameter bounded by a trait; a caller supplies any type implementing that trait; a test supplies a fake. Construction is separated from use by the ordinary discipline of taking collaborators as arguments rather than building them internally.

```rust
trait StorageClient {
    fn fetch(&self, object_id: &str) -> Vec<u8>;
}

struct ProfilePictureService<S: StorageClient> {
    storage: S,
}
```

This is dependency injection in the original sense — dependencies supplied from outside, interfaces instead of concrete types — with the compiler doing the checking and no runtime container involved. Its limitation is the one CGP exists to lift: a generic bound like `S: StorageClient` leaks into every caller's signature, and coherence permits only one `impl StorageClient` per type, so swapping implementations per application, or offering several interchangeable ones, runs into the same walls that motivate CGP's [consumer/provider split](../concepts/coherence.md). CGP is best introduced to a Rust audience as the next step along this line they already accept, not as an import of the container model they have rejected.

## How CGP expresses it

CGP performs dependency injection through two mechanisms working together: a provider declares what it needs as [impl-side dependencies](../concepts/impl-side-dependencies.md), and a context supplies them by [wiring](../concepts/consumer-and-provider-traits.md) each component to a provider. The impl-side dependency is the counterpart of a constructor's parameter list, and the wiring table is the counterpart of the container's configuration — but both are resolved by the compiler, so the "container" has no runtime existence at all.

### Impl-side dependencies are the injected constructor parameters

A provider states the collaborators and values it needs in a way that reads like declaring dependencies, and CGP satisfies them from the context rather than from a container. Where a Spring service lists `StorageClient` and `UserRepository` as constructor parameters, a CGP provider lists its capability dependencies with [`#[uses(...)]`](../reference/attributes/uses.md) and its value dependencies with [`#[implicit]`](../reference/attributes/implicit.md) arguments. A user-creation provider that needs a database connection and a censorship service declares both:

```rust
#[cgp_impl(new PostgresUserManager)]
#[uses(CanCensorUsername)]
#[use_type(HasErrorType.Error)]
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
}
```

The `#[uses(CanCensorUsername)]` line injects a *capability* — the same role a `UserRepository` collaborator plays in the Spring constructor — and the `#[implicit] database` argument injects a *value* pulled from the context's `database` field, the role a configuration bean plays. Neither dependency appears in the `CanManageUser` consumer trait a caller invokes, so, unlike a leaked generic bound, they do not cascade to callers — the decoupling a DI framework promises, delivered by hiding the requirements one level down in the provider's impl rather than in a container.

### Wiring is the container configuration

A context selects which provider satisfies each component in a [`delegate_components!`](../reference/macros/delegate_components.md) table, which is the direct analogue of a Spring `@Configuration` class or a Guice module: the one place where interfaces are mapped to implementations. Swapping an implementation is a one-line edit to this table, and — the crucial difference — two contexts can map the same component to different providers with no conflict, because the choice is keyed on the context type. A profile-picture service backed by different object stores in different deployments is two wiring tables:

```rust
#[cgp_component(StorageObjectFetcher)]
pub trait CanFetchStorageObject {
    fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
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

`FetchS3Object` and `FetchGCloudObject` are interchangeable providers of the same capability — the CGP equivalent of two beans bound to one interface — and the wiring picks one per context. Because the selection is resolved during type-checking and monomorphized to a direct call, the `App` binary contains only the S3 code path and the `GCloudApp` binary only the GCloud one, with no runtime dispatch. A DI container makes the same substitution, but by holding both implementations and choosing at startup from configuration.

### Checking replaces the container's startup validation

A DI container discovers a missing or ambiguous binding when it assembles the graph — at startup for Spring and Guice, at build time for Dagger. CGP's counterpart is [`check_components!`](../reference/macros/check_components.md), which asserts at compile time that a context's wiring is complete and every provider's transitive dependencies are satisfied:

```rust
check_components! {
    App {
        StorageObjectFetcherComponent,
    }
}
```

If `FetchS3Object` needs a field or capability the `App` context does not supply, this fails to compile with the missing dependency named, rather than surfacing as a startup exception or a `NullPointerException` deep in a request. It is the same guarantee Dagger gives — the graph is verified before the program runs — reached through the trait system instead of an annotation processor. CGP wiring is [lazy](../concepts/check-traits.md), so this check is what turns a latent gap into an early, readable error.

## What users like and dislike

Dependency-injection frameworks are among the most widely adopted tools in enterprise software, and the reasons practitioners value them are real. They decouple components from their collaborators, which makes code testable — a fake is injected exactly where a real dependency would be — and swappable across environments. They centralize wiring, so the shape of an application's object graph lives in one readable place rather than scattered through constructors. And a mature framework like Spring brings an enormous ecosystem: transaction management, security, web bindings, and configuration all keyed off the same bean model, so adopting the container brings far more than injection.

The complaints are equally well-documented, and they cluster around the runtime, reflective nature of the popular frameworks. The most common is that dependencies become *hidden*: with field injection, a class's signature says nothing about what it needs, so a reader must scan the whole class and a maintainer can add a dependency invisibly. Reflection-based resolution means a missing or mis-typed binding is a *runtime* failure — a startup exception or a `NullPointerException` in production — not a compile error, which is why the Spring community now steers hard toward constructor injection and away from field injection. There is a performance and startup cost to scanning the classpath and building the graph reflectively, which is why Dagger's compile-time generation exists and why it wins on Android and in latency-sensitive services. And there is the recurring complaint of *magic*: the framework does so much automatically, through reflection and proxies, that when something goes wrong the developer has little visibility into why, and the behavior is hard to reason about from the code alone. Even proponents concede the learning curve is steep and the machinery is heavy for a small program.

## How CGP compares

CGP takes the compile-time end of every axis the DI frameworks vary along, which is its central trade-off: it gives up runtime flexibility to gain static guarantees and zero cost. Because wiring is resolved by the trait system and monomorphized, there is no container, no reflection, no runtime graph, and no dynamic dispatch — a wired call compiles to a direct function call, and an unused provider is not in the binary. A dependency that a context fails to satisfy is a compile error at the wiring site, not a startup exception, so the class of failures that field injection is criticized for cannot occur. And a provider's dependencies are never hidden: they are stated in its `#[uses]` and `#[implicit]` declarations and enforced by the compiler, giving the explicitness the Spring community prizes in constructor injection, by default.

The costs are just as real and should be stated plainly. CGP resolves everything at compile time, so it cannot do what a runtime container does best: reconfigure an application without recompiling, load plugins chosen at startup from a config file, or build a graph whose shape is not known until the program runs. It is confined to Rust, whereas Spring anchors a vast cross-cutting ecosystem that injection is only the entry point to. Its machinery — the consumer/provider split, [coherence-bypassing](../concepts/coherence.md) wiring, type-level tables — has a learning curve of its own, and its error messages, though they name the missing dependency under a check, can be verbose. A DI framework remains the better choice when the application genuinely needs runtime reconfiguration, when it lives in a JVM or .NET ecosystem whose libraries assume the container, or when the team's familiarity with the framework outweighs the benefits of static wiring. CGP wins where the graph is known at build time and the guarantees and zero cost matter: systems programming, latency-sensitive services, and libraries that must not impose a runtime.

## Presenting CGP to someone who knows this

A reader who knows dependency injection arrives with the right instinct — decouple what code needs from what supplies it — and the fastest way in is to map their vocabulary onto CGP's directly. A **provider** is a bean or a binding: an interchangeable implementation of a capability. **Wiring** with `delegate_components!` is the container configuration — the `@Configuration` class or the Guice module — the single place where interfaces are matched to implementations. An **impl-side dependency** is a constructor parameter: what a provider needs from the outside, declared where the implementation lives and never leaked to callers. And **`check_components!`** is the graph validation a container runs — the difference being *when* it runs. Leading with this dictionary lets the reader reuse everything they know about why DI decouples code, and spend their attention only on what is new.

The one analogy to defuse immediately is the runtime container. A DI-trained reader will assume there is an object somewhere holding the graph, resolving dependencies by reflection, choosing implementations at startup — and there is not. CGP's "container" is the type system, the "graph" is a set of trait impls, and the resolution happens during compilation and compiles away to direct calls. Say this explicitly, because leaving it unsaid invites the reader to imagine a runtime cost and a runtime failure mode that do not exist. The framing that lands is *Dagger, taken further*: a reader who knows Dagger already accepts compile-time-verified injection with no reflection, and CGP is that same bargain with per-context choice added — the same interface can resolve to different implementations in different contexts, which a single global binding graph cannot express.

The advantages worth foregrounding are the ones that answer this audience's own complaints. Dependencies are explicit and compiler-checked, so the hidden-dependency and `NullPointerException` failure modes that drove the community off field injection are gone by construction. There is no startup cost or reflection, so the performance objection that motivates Dagger is answered more completely. And nothing unused reaches the binary, which reads to this audience as automatic tree-shaking of the object graph. The expectation to address head-on is that CGP asks for explicit wiring and cannot scan-and-discover: there is no classpath scanning, no `@Component` auto-registration, no runtime rebinding, and the graph must be known at compile time. Present that not as a limitation grafted on but as the deliberate price of the guarantees — the same trade Dagger made, carried to its conclusion — and the reader who values static safety will read it as a feature rather than a loss.

## Sources

The account of the related work draws on the official framework documentation and representative community writing, cited where a specific claim rests on one.

- [Spring Framework reference — Dependency Injection](https://docs.spring.io/spring-framework/reference/core/beans/dependencies/factory-collaborators.html) — the authoritative description of the IoC container, beans, and the constructor/setter injection mechanisms.
- [Baeldung — Inversion of Control and Dependency Injection in Spring](https://www.baeldung.com/inversion-control-and-dependency-injection-in-spring) — the distinction between IoC and DI and the `@Autowired` autowiring-by-type behavior.
- [Comparing Dependency Injection Frameworks — Spring, Guice, Dagger, and Micronaut](https://medium.com/@AlexanderObregon/comparing-dependency-injection-frameworks-spring-guice-and-dagger-a614dccd5859) and [Dagger vs Guice](https://www.hackingnote.com/en/versus/dagger-vs-guice/) — the runtime-versus-compile-time split and the reflection-versus-generated-code trade-off across frameworks.
- [Field injection is not recommended (Marc Nuri)](https://blog.marcnuri.com/field-injection-is-not-recommended) and [James Shore — The Problem With Dependency Injection Frameworks](https://www.jamesshore.com/v2/blog/2023/the-problem-with-dependency-injection-frameworks) — the hidden-dependency, runtime-failure, and "magic" criticisms, and the case for constructor injection.
- [Rust traits and dependency injection (jmmv.dev)](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html) — the position that Rust performs dependency injection through traits and generics without a framework.
