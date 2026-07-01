//! `#[cgp_getter]` on a getter trait with a local associated return type
//! (`type Scalar`): the full getter component is generated, including the
//! `UseContext`/`RedirectLookup`/`UseFields`/`UseField`/`WithProvider` provider
//! impls that carry the associated type through. The context then binds the
//! source field by wiring the getter to `UseField`.
//!
//! See docs/reference/macros/cgp_getter.md and docs/reference/providers/use_field.md.

use core::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasScalar {
        type Scalar: Mul<Output = Self::Scalar> + Clone;

        fn scalar(&self) -> &Self::Scalar;
    }

    expand_has_scalar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalar {
            type Scalar: Mul<Output = Self::Scalar> + Clone;
            fn scalar(&self) -> &Self::Scalar;
        }
        impl<__Context__> HasScalar for __Context__
        where
            __Context__: ScalarGetter<__Context__>,
        {
            type Scalar = <__Context__ as ScalarGetter<__Context__>>::Scalar;
            fn scalar(&self) -> &Self::Scalar {
                __Context__::scalar(self)
            }
        }
        pub trait ScalarGetter<
            __Context__,
        >: IsProviderFor<ScalarGetterComponent, __Context__, ()> {
            type Scalar: Mul<Output = Self::Scalar> + Clone;
            fn scalar(__context__: &__Context__) -> &Self::Scalar;
        }
        impl<__Provider__, __Context__> ScalarGetter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ScalarGetterComponent>
                + IsProviderFor<ScalarGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ScalarGetterComponent,
            >>::Delegate: ScalarGetter<__Context__>,
        {
            type Scalar = <<__Provider__ as DelegateComponent<
                ScalarGetterComponent,
            >>::Delegate as ScalarGetter<__Context__>>::Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                <__Provider__ as DelegateComponent<
                    ScalarGetterComponent,
                >>::Delegate::scalar(__context__)
            }
        }
        pub struct ScalarGetterComponent;
        impl<__Context__> ScalarGetter<__Context__> for UseContext
        where
            __Context__: HasScalar,
        {
            type Scalar = <__Context__ as HasScalar>::Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __Context__::scalar(__context__)
            }
        }
        impl<__Context__> IsProviderFor<ScalarGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasScalar,
        {}
        impl<__Context__, __Components__, __Path__> ScalarGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: ScalarGetter<__Context__>,
        {
            type Scalar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as ScalarGetter<__Context__>>::Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::scalar(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ScalarGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ScalarGetterComponent, __Context__, ()>
                + ScalarGetter<__Context__>,
        {}
        impl<__Context__, Scalar> ScalarGetter<__Context__> for UseFields
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'s', Chars<'c', Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>>>,
                >,
                Value = Scalar,
            >,
        {
            type Scalar = Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    's',
                                    Chars<
                                        'c',
                                        Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
            }
        }
        impl<__Context__, Scalar> IsProviderFor<ScalarGetterComponent, __Context__, ()>
        for UseFields
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'s', Chars<'c', Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>>>,
                >,
                Value = Scalar,
            >,
        {}
        impl<__Context__, Scalar, __Tag__> ScalarGetter<__Context__> for UseField<__Tag__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<__Tag__, Value = Scalar>,
        {
            type Scalar = Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, Scalar, __Tag__> IsProviderFor<ScalarGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<__Tag__, Value = Scalar>,
        {}
        impl<__Context__, Scalar, __Provider__> ScalarGetter<__Context__>
        for WithProvider<__Provider__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Provider__: FieldGetter<__Context__, ScalarGetterComponent, Value = Scalar>,
        {
            type Scalar = Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<ScalarGetterComponent>,
                )
            }
        }
        impl<
            __Context__,
            Scalar,
            __Provider__,
        > IsProviderFor<ScalarGetterComponent, __Context__, ()> for WithProvider<__Provider__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Provider__: FieldGetter<__Context__, ScalarGetterComponent, Value = Scalar>,
        {}
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub scalar: f64,
}

delegate_components! {
    App {
        ScalarGetterComponent:
            UseField<Symbol!("scalar")>,
    }
}

#[test]
fn test_auto_getter_scalar() {
    let app = App { scalar: 2.0 };

    assert_eq!(*app.scalar(), 2.0);
}
