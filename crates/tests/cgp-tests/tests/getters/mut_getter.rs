//! `#[cgp_getter]` with a `&mut self` receiver returning `&mut u32`: the three
//! generated provider impls read the field mutably through `get_field_mut`. The
//! `UseFields` and `UseField` impls bound the context by `HasFieldMut`, and the
//! `WithProvider` impl delegates to a `MutFieldGetter` provider rather than a
//! `FieldGetter`. The context binds a differently-named `value` field by wiring
//! to `UseField`.
//!
//! See docs/reference/macros/cgp_getter.md and docs/reference/providers/use_field.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasCount {
        fn count(&mut self) -> &mut u32;
    }

    expand_has_count(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasCount {
            fn count(&mut self) -> &mut u32;
        }
        impl<__Context__> HasCount for __Context__
        where
            __Context__: CountGetter<__Context__>,
        {
            fn count(&mut self) -> &mut u32 {
                __Context__::count(self)
            }
        }
        pub trait CountGetter<
            __Context__,
        >: IsProviderFor<CountGetterComponent, __Context__, ()> {
            fn count(__context__: &mut __Context__) -> &mut u32;
        }
        impl<__Provider__, __Context__> CountGetter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<CountGetterComponent>
                + IsProviderFor<CountGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                CountGetterComponent,
            >>::Delegate: CountGetter<__Context__>,
        {
            fn count(__context__: &mut __Context__) -> &mut u32 {
                <__Provider__ as DelegateComponent<
                    CountGetterComponent,
                >>::Delegate::count(__context__)
            }
        }
        pub struct CountGetterComponent;
        impl<__Context__> CountGetter<__Context__> for UseContext
        where
            __Context__: HasCount,
        {
            fn count(__context__: &mut __Context__) -> &mut u32 {
                __Context__::count(__context__)
            }
        }
        impl<__Context__> IsProviderFor<CountGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasCount,
        {}
        impl<__Context__, __Components__, __Path__> CountGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: CountGetter<__Context__>,
        {
            fn count(__context__: &mut __Context__) -> &mut u32 {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::count(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<CountGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<CountGetterComponent, __Context__, ()>
                + CountGetter<__Context__>,
        {}
        impl<__Context__> CountGetter<__Context__> for UseFields
        where
            __Context__: HasFieldMut<
                Symbol<5, Chars<'c', Chars<'o', Chars<'u', Chars<'n', Chars<'t', Nil>>>>>>,
                Value = u32,
            >,
        {
            fn count(__context__: &mut __Context__) -> &mut u32 {
                __context__
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'c', Chars<'o', Chars<'u', Chars<'n', Chars<'t', Nil>>>>>,
                            >,
                        >,
                    )
            }
        }
        impl<__Context__> IsProviderFor<CountGetterComponent, __Context__, ()> for UseFields
        where
            __Context__: HasFieldMut<
                Symbol<5, Chars<'c', Chars<'o', Chars<'u', Chars<'n', Chars<'t', Nil>>>>>>,
                Value = u32,
            >,
        {}
        impl<__Context__, __Tag__> CountGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasFieldMut<__Tag__, Value = u32>,
        {
            fn count(__context__: &mut __Context__) -> &mut u32 {
                __context__.get_field_mut(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<CountGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasFieldMut<__Tag__, Value = u32>,
        {}
        impl<__Context__, __Provider__> CountGetter<__Context__> for WithProvider<__Provider__>
        where
            __Provider__: MutFieldGetter<__Context__, CountGetterComponent, Value = u32>,
        {
            fn count(__context__: &mut __Context__) -> &mut u32 {
                __Provider__::get_field_mut(
                    __context__,
                    ::core::marker::PhantomData::<CountGetterComponent>,
                )
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<CountGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: MutFieldGetter<__Context__, CountGetterComponent, Value = u32>,
        {}
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub value: u32,
}

delegate_components! {
    App {
        CountGetterComponent: UseField<Symbol!("value")>,
    }
}

#[test]
pub fn test_mut_getter() {
    let mut context = App { value: 9 };

    *context.count() += 1;

    assert_eq!(context.value, 10);
}
