//! `delegate_and_check_components!` on a generic context: the leading generic
//! list (`<T> MyContext<T>`) wires and checks a whole context family at once, and
//! the check trait defaults to `__CanUse{Context}`. This concept owns the macro's
//! expansion snapshot.
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
pub struct MyContext<T> {
    pub name: T,
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        <T>
        MyContext<T> {
            NameTypeProviderComponent: UseType<T>,
            NameGetterComponent: UseField<Symbol!("name")>,
        }
    }

    expand_my_context(output) {
        insta::assert_snapshot!(output, @r#"
        impl<T> DelegateComponent<NameTypeProviderComponent> for MyContext<T> {
            type Delegate = UseType<T>;
        }
        impl<
            T,
            __Context__,
            __Params__,
        > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for MyContext<T>
        where
            UseType<T>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
        {}
        impl<T> DelegateComponent<NameGetterComponent> for MyContext<T> {
            type Delegate = UseField<Symbol!("name")>;
        }
        impl<
            T,
            __Context__,
            __Params__,
        > IsProviderFor<NameGetterComponent, __Context__, __Params__> for MyContext<T>
        where
            UseField<
                Symbol!("name"),
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        trait __CanUseMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<T> __CanUseMyContext<NameTypeProviderComponent, ()> for MyContext<T> {}
        impl<T> __CanUseMyContext<NameGetterComponent, ()> for MyContext<T> {}
        "#)
    }
}
