//! `delegate_components!` shorthand for the obvious type and getter providers.
//!
//! `type Name = Type` is `NameTypeProviderComponent: UseType<Type>`.
//! `getter field` is `FieldGetterComponent: UseField` of that field's symbol.
//! The explicit `Key: Provider` form is unchanged.
//!
//! See cgp-knowledge-base/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_delegate_components;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

pub struct GreetHello;

snapshot_delegate_components! {
    delegate_components! {
        new PersonComponents {
            type Name = String,
            getter name,
            GreeterComponent: GreetHello,
        }
    }

    expand_shorthand(output) {
        insta::assert_snapshot!(output, @"
        pub struct PersonComponents;
        impl DelegateComponent<NameTypeProviderComponent> for PersonComponents {
            type Delegate = UseType<String>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for PersonComponents
        where
            UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<NameGetterComponent> for PersonComponents {
            type Delegate = UseField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
            >;
        }
        impl<__Context__, __Params__> IsProviderFor<NameGetterComponent, __Context__, __Params__>
        for PersonComponents
        where
            UseField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<GreeterComponent> for PersonComponents {
            type Delegate = GreetHello;
        }
        impl<__Context__, __Params__> IsProviderFor<GreeterComponent, __Context__, __Params__>
        for PersonComponents
        where
            GreetHello: IsProviderFor<GreeterComponent, __Context__, __Params__>,
        {}
        ");
    }
}

snapshot_delegate_components! {
    delegate_components! {
        new ExplicitComponents {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<Symbol!("name")>,
            GreeterComponent: GreetHello,
        }
    }

    expand_explicit(output) {
        insta::assert_snapshot!(output, @r#"
        pub struct ExplicitComponents;
        impl DelegateComponent<NameTypeProviderComponent> for ExplicitComponents {
            type Delegate = UseType<String>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>
        for ExplicitComponents
        where
            UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<NameGetterComponent> for ExplicitComponents {
            type Delegate = UseField<Symbol!("name")>;
        }
        impl<__Context__, __Params__> IsProviderFor<NameGetterComponent, __Context__, __Params__>
        for ExplicitComponents
        where
            UseField<
                Symbol!("name"),
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<GreeterComponent> for ExplicitComponents {
            type Delegate = GreetHello;
        }
        impl<__Context__, __Params__> IsProviderFor<GreeterComponent, __Context__, __Params__>
        for ExplicitComponents
        where
            GreetHello: IsProviderFor<GreeterComponent, __Context__, __Params__>,
        {}
        "#);
    }
}
