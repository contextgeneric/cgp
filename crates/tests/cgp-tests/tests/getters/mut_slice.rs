//! `#[cgp_getter]` with a `&mut self` receiver returning `&mut [u32]`: the
//! generated provider impls read a field whose value implements `AsMut<[u32]>`
//! (e.g. `Vec<u32>`) mutably through `get_field_mut` and call `.as_mut()`. The
//! `UseFields`/`UseField` impls bound the context by `HasFieldMut<..., Value:
//! AsMut<[u32]>>` and the `WithProvider` impl delegates to a `MutFieldGetter`
//! with the same `AsMut<[u32]>` bound — the mutable mirror of the shared `&[T]`
//! getter's `FieldGetter`/`AsRef<[u32]>`. This pins the non-auto getter's
//! mutable-slice path, which the `#[cgp_auto_getter]` snapshots do not exercise.
//!
//! See docs/reference/macros/cgp_getter.md and docs/reference/providers/use_field.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasItems {
        fn items(&mut self) -> &mut [u32];
    }

    expand_has_items(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasItems {
            fn items(&mut self) -> &mut [u32];
        }
        impl<__Context__> HasItems for __Context__
        where
            __Context__: ItemsGetter<__Context__>,
        {
            fn items(&mut self) -> &mut [u32] {
                __Context__::items(self)
            }
        }
        pub trait ItemsGetter<
            __Context__,
        >: IsProviderFor<ItemsGetterComponent, __Context__, ()> {
            fn items(__context__: &mut __Context__) -> &mut [u32];
        }
        impl<__Provider__, __Context__> ItemsGetter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ItemsGetterComponent>
                + IsProviderFor<ItemsGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ItemsGetterComponent,
            >>::Delegate: ItemsGetter<__Context__>,
        {
            fn items(__context__: &mut __Context__) -> &mut [u32] {
                <__Provider__ as DelegateComponent<
                    ItemsGetterComponent,
                >>::Delegate::items(__context__)
            }
        }
        pub struct ItemsGetterComponent;
        impl<__Context__> ItemsGetter<__Context__> for UseContext
        where
            __Context__: HasItems,
        {
            fn items(__context__: &mut __Context__) -> &mut [u32] {
                __Context__::items(__context__)
            }
        }
        impl<__Context__> IsProviderFor<ItemsGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasItems,
        {}
        impl<__Context__, __Components__, __Path__> ItemsGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: ItemsGetter<__Context__>,
        {
            fn items(__context__: &mut __Context__) -> &mut [u32] {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::items(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ItemsGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ItemsGetterComponent, __Context__, ()>
                + ItemsGetter<__Context__>,
        {}
        impl<__Context__> ItemsGetter<__Context__> for UseFields
        where
            __Context__: HasFieldMut<
                Symbol<5, Chars<'i', Chars<'t', Chars<'e', Chars<'m', Chars<'s', Nil>>>>>>,
                Value: AsMut<[u32]> + 'static,
            >,
        {
            fn items(__context__: &mut __Context__) -> &mut [u32] {
                __context__
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'i', Chars<'t', Chars<'e', Chars<'m', Chars<'s', Nil>>>>>,
                            >,
                        >,
                    )
                    .as_mut()
            }
        }
        impl<__Context__> IsProviderFor<ItemsGetterComponent, __Context__, ()> for UseFields
        where
            __Context__: HasFieldMut<
                Symbol<5, Chars<'i', Chars<'t', Chars<'e', Chars<'m', Chars<'s', Nil>>>>>>,
                Value: AsMut<[u32]> + 'static,
            >,
        {}
        impl<__Context__, __Tag__> ItemsGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasFieldMut<__Tag__, Value: AsMut<[u32]> + 'static>,
        {
            fn items(__context__: &mut __Context__) -> &mut [u32] {
                __context__.get_field_mut(::core::marker::PhantomData::<__Tag__>).as_mut()
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<ItemsGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasFieldMut<__Tag__, Value: AsMut<[u32]> + 'static>,
        {}
        impl<__Context__, __Provider__> ItemsGetter<__Context__> for WithProvider<__Provider__>
        where
            __Provider__: MutFieldGetter<
                __Context__,
                ItemsGetterComponent,
                Value: AsMut<[u32]> + 'static,
            >,
        {
            fn items(__context__: &mut __Context__) -> &mut [u32] {
                __Provider__::get_field_mut(
                        __context__,
                        ::core::marker::PhantomData::<ItemsGetterComponent>,
                    )
                    .as_mut()
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<ItemsGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: MutFieldGetter<
                __Context__,
                ItemsGetterComponent,
                Value: AsMut<[u32]> + 'static,
            >,
        {}
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub values: Vec<u32>,
}

delegate_components! {
    App {
        ItemsGetterComponent: UseField<Symbol!("values")>,
    }
}

#[test]
pub fn test_mut_slice_getter() {
    let mut context = App {
        values: vec![1, 2, 3],
    };

    context.items()[0] = 10;

    assert_eq!(context.values, vec![10, 2, 3]);
}
