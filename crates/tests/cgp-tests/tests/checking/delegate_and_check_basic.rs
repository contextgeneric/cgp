//! `delegate_and_check_components!` in its basic form: it wires a context to
//! providers *and* asserts the wiring is usable in one step, generating a
//! `CanUseComponent`-supertraited check trait (here renamed with
//! `#[check_trait(...)]`). This concept owns the macro's expansion snapshot.
//!
//! See docs/reference/macros/delegate_and_check_components.md and
//! docs/reference/traits/can_use_component.md.

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

#[derive(HasField)]
pub struct MyContext {
    pub name: String,
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        #[check_trait(CheckMyContext)]
        MyContext {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<Symbol!("name")>,
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
        trait CheckMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl CheckMyContext<NameTypeProviderComponent, ()> for MyContext {}
        impl CheckMyContext<NameGetterComponent, ()> for MyContext {}
        "#)
    }
}
