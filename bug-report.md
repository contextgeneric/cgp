## Bug Report

**Title:** `rustc` stack overflow with old trait solver on recursive generic types

### Summary

The Rust compiler (`rustc`) encounters a stack overflow when compiling a specific crate using `cargo +nightly check`. The error message is `error: rustc unexpectedly overflowed its stack! this is a bug`.

This bug appears to be specific to the old trait solver, as the compilation succeeds when using the new trait solver (`RUSTFLAGS="-Znext-solver=globally"`).

The issue is triggered by a combination of procedural macros, generic traits, and recursive type structures (specifically involving `Either` types). The code that fails involves dispatching a computation over a field extracted from an enum.

### Steps to Reproduce

The bug can be reproduced with the following command inside the project:

```sh
cargo +nightly check --tests -p cgp-tests
```

This will result in the compiler stack overflow.

### Code that causes the bug

The issue is located in the `test_extractor_dispatcher` function within `cgp/crates/cgp-tests/src/tests/extractor/basic.rs`.

Here is the full content of `cgp/crates/cgp-tests/src/tests/extractor/basic.rs`:
```rust
use core::fmt::Display;
use core::marker::PhantomData;

use cgp::extra::handler::{Computer, ComputerComponent, DispatchFields, DispatchHandlers};
use cgp::prelude::*;

#[derive(HasFields, ExtractField)]
pub enum Context {
    Foo(u64),
    Bar(String),
    Baz(bool),
}

fn context_to_string(context: Context) -> String {
    match context
        .extractor_ref()
        .extract_field(PhantomData::<symbol!("Foo")>)
    {
        Either::Left(value) => value.to_string(),
        Either::Right(remainder) => match remainder.extract_field(PhantomData::<symbol!("Bar")>) {
            Either::Left(value) => value.to_string(),
            Either::Right(remainder) => {
                match remainder.extract_field(PhantomData::<symbol!("Baz")>) {
                    Either::Left(value) => value.to_string(),
                    Either::Right(remainder) => remainder.finalize_extract(),
                }
            }
        },
    }
}

#[cgp_new_provider]
impl<Context, Code, Tag, Value> Computer<Context, Code, Field<Tag, Value>> for FieldToString
where
    Value: Display,
{
    type Output = String;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Field<Tag, Value>) -> String {
        input.value.to_string()
    }
}

#[cgp_provider]
impl<Context, Code> Computer<Context, Code, Void> for FieldToString {
    type Output = String;

    fn compute(_context: &Context, _tag: PhantomData<Code>, input: Void) -> String {
        match input {}
    }
}

pub trait CheckComputerImpl: Computer<(), (), Context> {}
impl CheckComputerImpl for DispatchFields<FieldToString> {}

#[test]
fn test_basic_extractor() {
    assert_eq!(context_to_string(Context::Foo(1)), "1");
    assert_eq!(context_to_string(Context::Bar("hello".to_owned())), "hello");
    assert_eq!(context_to_string(Context::Baz(true)), "true");
}

#[test]
fn test_extractor_dispatcher() {
    let res = DispatchFields::<FieldToString>::compute(&(), PhantomData::<()>, Context::Foo(1));

    // let res = DispatchHandlers::<<Context as HasFields>::Fields, FieldToString>::compute(
    //     &(),
    //     PhantomData::<()>,
    //     Context::Foo(1).extractor(),
    // );

    assert_eq!(res, "1");
}
```

The compilation fails on this line:
```rust
let res = DispatchFields::<FieldToString>::compute(&(), PhantomData::<()>, Context::Foo(1));
```

If that line is replaced with the commented-out code that uses `DispatchHandlers`, the compilation is successful. This suggests the issue is with the trait resolution for `DispatchFields`.

### Expected vs. Actual Behavior

- **Expected**: The code should either compile successfully or provide a clear error message about unsatisfiable trait bounds or recursion limits, rather than crashing.
- **Actual**: `rustc` crashes with a stack overflow.

```
error: rustc unexpectedly overflowed its stack! this is a bug

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.89.0-nightly (49a8ba068 2025-06-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C metadata=... -C extra-filename-prefix=...
```

### Rust Version

```
rustc 1.89.0-nightly (49a8ba068 2025-06-14)
binary: rustc
commit-hash: 49a8ba06848fa8f282fe9055b4178350970bb0ce
commit-date: 2025-06-14
host: x86_64-unknown-linux-gnu
release: 1.89.0-nightly
LLVM version: 20.1.5
```

### Additional Context

This bug is only present with the default (old) trait solver. When the new trait solver is enabled globally, the compilation succeeds.

The following command runs without errors:
```sh
RUSTFLAGS="-Znext-solver=globally" cargo +nightly check --tests -p cgp-tests
```

This strongly indicates that the issue lies within the old trait solver's handling of the recursive type definitions involved.

A temporary workaround is to increase the recursion limit in `cgp/crates/cgp-tests/src/lib.rs` by adding `#![recursion_limit = "256"]`. However, this should not be necessary and masks the underlying compiler issue.

### Backtrace

Running `RUST_BACKTRACE=1 cargo +nightly check --tests -p cgp-tests` provides the following backtrace:

```
error: rustc interrupted by SIGSEGV, printing backtrace

/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x3c20c0f)[0x73b13d420c0f]
/lib/x86_64-linux-gnu/libc.so.6(+0x45330)[0x73b139445330]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535e8a1)[0x73b13eb5e8a1]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535e9ec)[0x73b13eb5e9ec]

### cycle encountered after 4 frames with period 5
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]
### recursed 50 times

/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]
/home/soares/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-01acad332a
4cdc9a.so(+0x535eb34)[0x73b13eb5eb34]

note: rustc unexpectedly overflowed its stack! this is a bug
note: maximum backtrace depth reached, frames may have been lost
```