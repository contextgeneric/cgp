# Type-level primitives

The handful of zero-sized types and type macros that CGP folds strings, numbers, lists, choices, and routes into the type system, so the compiler can match a field name or a wiring route through trait resolution alone.

## The idea

CGP keys nearly everything by *type*, not by value. A getter looks up a field by a tag type; a [wiring](wiring.md) table selects a [provider](components.md) by a [component](components.md) key type; a [namespace](namespaces.md) re-routes a lookup along a path type. For that to work, things that are normally values — a field name string, a tuple position, a list of fields, a lifetime — have to be encoded as types the compiler can compare and dispatch on. The primitives in this reference are those encodings. Each is a familiar value-level idea lifted into a type: a string becomes a type-level character list, a number becomes a const-generic marker, a list becomes a recursive cons cell, a sum becomes a recursive branch.

These types are almost never written by hand. They are produced by macros (`Symbol!`, `Product!`, `Sum!`, `Path!`) or emitted by derives, and a reader mostly meets them when *decoding a type the compiler prints* — in an error message, a `cargo expand` dump, or a hover. This reference is a decoder ring: skim it to read off what a long nested type means. One note on the printed forms first: the compiler abbreviates the busiest spines with Greek letters — `π` for `Cons`, `ε` for `Nil`, `ψ` for `Symbol`, `ζ` for `Chars`, `δ` for `Index` — so a printed `π<A, π<B, ε>>` is just `Cons<A, Cons<B, Nil>>`. The prose below always uses the readable `Cons`/`Nil`/`Symbol!` forms; treat the Greek as a transcription you may have to reverse.

Assume `use cgp::prelude::*;` throughout.

## Type-level lists: `Product!`, `Cons`, `Nil`

A type-level list is a compile-time linked list — the analogue of a tuple that generic code can take apart one element at a time. It is built from two cells: `Cons<Head, Tail>`, a pair holding the first element and the rest of the list, and `Nil`, a unit struct marking the end. Chained to the right and terminated by `Nil`, they form an *anonymous product type* — a record-shaped type whose width and contents a provider can walk without knowing the concrete struct it came from.

```rust
pub struct Cons<Head, Tail>(pub Head, pub Tail);
pub struct Nil;
```

The `Product!` macro is the sugar a programmer writes instead of nesting `Cons` by hand, and the lowercase `product!` builds a matching value with the tuple-struct constructor:

```rust
type Row = Product![u32, String, bool];
// Row == Cons<u32, Cons<String, Cons<bool, Nil>>>

let row: Row = product![1, "hi".to_string(), true];
// row == Cons(1, Cons("hi".to_string(), Cons(true, Nil)))
```

The list is what makes structural, field-by-field code possible. A struct's fields are exposed as one `Product!` type through `HasFields`, so a provider written once to recurse over `Cons`/`Nil` — a `Nil` impl for the base case, a `Cons<Head, Tail>` impl for the step — iterates, reads, or rebuilds *any* struct's fields. The elements are usually [`Field`](#field-a-named-value) entries pairing a name with a value, so a derived struct's layout reads as a `Product!` of `Field` cells. See [extensible data](extensible-data.md) for how that machinery is used.

## Type-level sums: `Sum!`, `Either`, `Void`

A type-level sum is the dual of the list: where a `Product!` holds a value for *every* element at once, a `Sum!` holds a value for exactly *one* branch — a tagged union the compiler can walk variant by variant. It shares the same right-nested shape but branches at each step instead of pairing, and terminates in an uninhabited marker instead of a constructible one.

```rust
pub enum Either<Head, Tail> { Left(Head), Right(Tail) }
pub enum Void {}
```

`Either<Head, Tail>` is the sum cell — `Left(head)` selects this branch, `Right(tail)` defers to the rest — and `Void`, an empty enum with no values, closes the chain. The `Sum!` macro folds a list of types onto that spine, and a value picks one branch by how deep it sits:

```rust
type Token = Sum![u32, String, bool];
// Token == Either<u32, Either<String, Either<bool, Void>>>

let t: Token = Either::Right(Either::Left("hi".to_string())); // the String branch
```

The terminator is the one real difference from the product spine, and it is load-bearing. A product ends in the constructible `Nil` because an empty record is a valid value; a sum ends in the *uninhabited* `Void` because an empty choice has no value to pick. After an extractor has tried every variant and matched none, the leftover has type `Void` — a value that cannot exist — which the machinery discharges with an empty `match self {}`, making a fully-handled variant match total at compile time with no unreachable runtime branch. An enum's variants are exposed as a `Sum!` of `Field` entries through `HasFields`, mirroring how a struct's fields are a `Product!`.

## Type-level strings: `Symbol!`, `Symbol`, `Chars`

A type-level string is a field name encoded as a type, so the name can drive trait resolution. CGP's getter, `HasField<Tag>`, keys each field by a `Tag` type; to read a field called `name`, the string `"name"` must become a unique type, so that two symbols from the same string are the *same* type and two from different strings are *different* types. `Symbol!("name")` produces exactly that.

The reason it is a *list of characters* rather than one const value is a stable-Rust limitation: a `&str` cannot be a const-generic parameter, but a single `char` can. So `Chars<const CHAR: char, Tail>` spells the string out one character at a time — the specialized analogue of `Cons` where the head is a `const char` — and `Symbol<const LEN: usize, Chars>` wraps that list together with the byte length:

```rust
pub struct Chars<const CHAR: char, Tail>(pub PhantomData<Tail>);
pub struct Symbol<const LEN: usize, Chars>(pub PhantomData<Chars>);
```

The `Symbol!` macro hides all of this. The leading `LEN` const is the part most likely to puzzle a reader: stable Rust cannot compute a `Chars` chain's length inside a const-generic context, so the macro precomputes `str::len()` — the *byte* length — and bakes it into the type:

```rust
// before
Symbol!("abc")
// after
Symbol<3, Chars<'a', Chars<'b', Chars<'c', Nil>>>>
```

Because `LEN` is the byte length, a multi-byte string records its UTF-8 width: `Symbol!("世界你好")` records `12`, while its character list still has one `Chars` node per scalar. The empty string is `Symbol<0, Nil>`. A type-level string is most often seen as the tag in a getter bound, where it names the field a provider reads without that context naming the provider:

```rust
#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasField<Symbol!("name"), Value = String>,
{
    fn greet(&self) {
        println!("Hello, {}!", self.get_field(PhantomData::<Symbol!("name")>));
    }
}
```

The same string can be recovered at runtime — see [`StaticFormat`](#staticformat-recovering-strings-and-paths) below.

## `Index<N>`: type-level numbers

`Index<const I: usize>` is the numeric counterpart to `Symbol!` — a `usize` lifted into a type, used to tag a tuple-struct field that has a position but no name. Where a named field is keyed by `Symbol!("name")`, the field at position `N` is keyed by `Index<N>`, so `Index<0>`, `Index<1>`, and `Index<2>` are three distinct tag types standing in for `.0`, `.1`, and `.2`.

```rust
pub struct Index<const I: usize>;
```

It is a zero-sized marker: the number lives entirely in the type, so a tuple struct can carry a `HasField<Index<0>>` impl and a `HasField<Index<1>>` impl side by side and the compiler selects the right one purely from the tag. Selecting a wrong position — `Index<5>` on a three-field struct — is a type error, not a runtime panic, because no matching impl exists. Unlike the Greek-abbreviated spines, `Index` prints its number directly through `Display`, so `Index::<2>.to_string()` is `"2"` and the tag is legible in diagnostics.

## `Field`: a named value

`Field<Tag, Value>` is the element type that fills both spines — a value paired with the type-level tag naming it. A bare `Product![String, u8]` records only types and order; wrapping each element as `Field<Symbol!("name"), String>` attaches the name as a phantom type, making the structural representation self-describing so a provider can match on the tag to find the field it wants.

```rust
pub struct Field<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}
```

The tag is a phantom — needed only at compile time for resolution — so a `Field` is exactly as large as its `Value` and costs nothing at runtime. It is built from a value with no tag argument, since the tag is fixed by the target type: `let f: Field<Symbol!("name"), String> = "Alice".to_string().into();`. The same shape names a record field (tag from `Symbol!` or `Index`) and an enum variant (tag from `Symbol!`, value being the payload), which is why a derived `HasFields` is a `Product!` or `Sum!` of `Field` entries:

```rust
#[derive(HasFields)]
pub struct Person { pub name: String, pub age: u8 }

// generated:
// type Fields = Product![
//     Field<Symbol!("name"), String>,
//     Field<Symbol!("age"), u8>,
// ];
```

## `Path!` and `PathCons`: type-level routes

A type-level path is a route through nested [wiring](wiring.md) tables, expressed as a single type. Where a bare component key picks one entry out of a context's table, a path points at an entry behind one or more layers of indirection — inside a [namespace](namespaces.md), under a prefix — by listing the segments to walk left to right. `PathCons<Head, Tail>` is the cons cell of that route, terminated by `Nil`, and it differs from the `Cons` product spine in one way: both `Head` and `Tail` are `?Sized`, because a path segment is a pure type-level marker that never needs a known size.

```rust
pub struct PathCons<Head: ?Sized, Tail: ?Sized>(pub PhantomData<Head>, pub PhantomData<Tail>);
```

The `Path!` macro builds the route from a dotted, `@`-prefixed name, encoding each segment by case: a lowercase, non-primitive identifier becomes a `Symbol!` type-level string, and a capitalized name stays the named type it spells (typically a component key or namespace marker):

```rust
type ErrorRoute = Path!(@app.error.ErrorRaiserComponent);
// PathCons<Symbol!("app"),
//     PathCons<Symbol!("error"),
//         PathCons<ErrorRaiserComponent, Nil>>>
```

A path names only *where to look*, never a provider directly, so the same path resolves to different providers depending on the table it is walked against — the job of the `RedirectLookup` provider that consumes it. This same `@`-path syntax appears verbatim inside namespace entries, which is where paths are most often written rather than through the bare macro. See [namespaces](namespaces.md) for the redirected-lookup mechanism.

## `Life<'a>`: a lifetime as a type

`Life<'a>` lifts a lifetime into a type, so a CGP trait that borrows can still ride through wiring machinery that only accepts types. CGP's dependency marker, `IsProviderFor`, takes a tuple of a component's generic parameters as one type argument — and a tuple member must be a type, never a bare lifetime. A consumer trait declaring `fn get_reference(&self) -> &'a T` therefore cannot record its `'a` directly; `Life<'a>` is the conversion that packages the lifetime as a type so it can sit in the tuple as `(Life<'a>, T)`.

```rust
pub struct Life<'a>(pub PhantomData<*mut &'a ()>);
```

The `*mut &'a ()` phantom is deliberate: a raw pointer is *invariant* in its lifetime, so `Life<'a>` is invariant in `'a`. That is correct here — the lifetime is an exact identity in the dependency marker, and a variant `Life` would let the compiler silently coerce one instantiation into another and pick the wrong provider. The macros insert `Life` automatically when a component carries a lifetime; a reader meets it in the generated provider trait, where `IsProviderFor<…, (Life<'a>, T)>` names the lifetime as a type rather than a bare `'a`. Conceptually it joins `Index` (which lifts a `usize`) and `Symbol` (which lifts a string) as another marker making a non-type thing addressable in trait resolution.

## `MRef<'a, T>`: owned-or-borrowed

`MRef<'a, T>` is a "maybe-reference" — an enum holding either a borrow of a `T` or an owned `T` — so a single getter signature serves both the context that already stores a value and the one that must produce it. Unlike the rest of this reference, there is nothing type-level about it: it is an ordinary runtime value, the payload a getter hands back.

```rust
pub enum MRef<'a, T> { Ref(&'a T), Owned(T) }
```

A getter declared to return `MRef<'a, T>` lets a context with the value in a field return `MRef::Ref` and lend it, while a context that computes the value returns `MRef::Owned` and gives it away — with no extra cost in the common stored-field case. The caller treats both uniformly because `MRef` derefs to `T`: it implements `Deref<Target = T>` and `AsRef<T>`, builds either variant through `From<T>` and `From<&'a T>`, and promotes a borrow to ownership with `get_or_clone` when `T: Clone`.

```rust
let stored = String::from("hello");
let borrowed: MRef<'_, String> = MRef::from(&stored);     // lends a stored value
let made: MRef<'_, String>     = MRef::from(String::from("world")); // hands over a built one
assert_eq!(&*borrowed, "hello");
let owned: String = borrowed.get_or_clone();              // clones the borrowed case
```

It is one of the getter return modes recognized by the field macros, parallel to `&T`, `Option<&T>`, or `&str`; see [functions and getters](functions-and-getters.md). Its lifetime is an ordinary borrow and is unrelated to the `Life<'a>` lift above.

## `StaticFormat`: recovering strings and paths

The type-level encodings need a way back to runtime data, and three traits provide it. `StaticFormat` recovers a type-level string *lazily* by writing into a formatter — it backs the `Display` impls on `Symbol` and `Chars`, recursing down the `Chars` spine to emit each character, so any symbol prints with `to_string()` or `{}`:

```rust
let s = <Symbol!("hello")>::default();
assert_eq!(s.to_string(), "hello");
```

`StaticString` recovers it *eagerly*, as a compile-time `&'static str` constant: a blanket impl walks the `Chars` list and UTF-8-encodes it into a `[u8; LEN]` at const-evaluation time — which is the consumer that `Symbol`'s `LEN` byte length exists to size — then validates the bytes as a `&'static str`. Use `Display` when a runtime value will do; use `StaticString::VALUE` when a `const` is needed or in a hot path. Both round-trip multi-byte Unicode faithfully:

```rust
use cgp::core::field::traits::StaticString;
assert_eq!(<Symbol!("世界你好") as StaticString>::VALUE, "世界你好");
```

`ConcatPath` works one level up, joining two `PathCons` paths into one as a pure type-level computation — it keeps each `Head` and splices the second path on where the first reaches `Nil`, the operation behind composing nested accessors:

```rust
type Joined = <Path!(@a.b) as ConcatPath<Path!(@c.d)>>::Output; // the path a.b.c.d
```

## Further reference

Online source-of-truth documents: the [types directory](https://github.com/contextgeneric/cgp/tree/main/docs/reference/types) (`cons.md`, `either.md`, `chars.md`, `index.md`, `field.md`, `life.md`, `mref.md`, `path_cons.md`), the construction macros [`macros/symbol.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/symbol.md), [`macros/product.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/product.md), [`macros/sum.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/sum.md), [`macros/path.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/path.md), and the recovery traits [`traits/static_format.md`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/static_format.md).
