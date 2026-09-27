//! `delegate_and_check_components!` with a `#[skip_check]` entry beside checked
//! entries: the skipped key still gets its `DelegateComponent` and `IsProviderFor`
//! impls, but no check impl. `MyContext` has no `age` field, so checking
//! `AgeGetterComponent` would fail; the table compiling is what shows the skip
//! took effect, while the name entries are still checked.
//!
//! See cgp-knowledge-base/cgp/reference/macros/delegate_and_check_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_and_check_components;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

#[cgp_getter]
pub trait HasName: HasNameType {
    fn name(&self) -> &Self::Name;
}

#[cgp_getter]
pub trait HasAge {
    fn age(&self) -> &u8;
}

#[derive(HasField)]
pub struct MyContext {
    pub name: String,
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        MyContext {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<Symbol!("name")>,
            #[skip_check]
            AgeGetterComponent: UseField<Symbol!("age")>,
        }
    }

    expand_my_context(output) {
        insta::assert_snapshot!(output, @r#"
        impl DelegateComponent<NameTypeProviderComponent> for MyContext {
            type Delegate = UseType<String>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for MyContext
        where
            UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<NameGetterComponent> for MyContext {
            type Delegate = UseField<Symbol!("name")>;
        }
        impl<__Context__, __Params__> IsProviderFor<NameGetterComponent, __Context__, __Params__>
        for MyContext
        where
            UseField<
                Symbol!("name"),
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AgeGetterComponent> for MyContext {
            type Delegate = UseField<Symbol!("age")>;
        }
        impl<__Context__, __Params__> IsProviderFor<AgeGetterComponent, __Context__, __Params__>
        for MyContext
        where
            UseField<Symbol!("age")>: IsProviderFor<AgeGetterComponent, __Context__, __Params__>,
        {}
        trait __CanUseMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseMyContext<NameTypeProviderComponent, ()> for MyContext {}
        impl __CanUseMyContext<NameGetterComponent, ()> for MyContext {}
        "#)
    }
}
