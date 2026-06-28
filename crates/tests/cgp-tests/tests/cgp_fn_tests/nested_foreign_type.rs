use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_fn, snapshot_cgp_type};

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasScalarType {
        type Scalar;
    }

    expand_has_scalar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalarType {
            type Scalar;
        }
        impl<__Context__> HasScalarType for __Context__
        where
            __Context__: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <__Context__ as ScalarTypeProvider<__Context__>>::Scalar;
        }
        pub trait ScalarTypeProvider<
            __Context__,
        >: IsProviderFor<ScalarTypeProviderComponent, __Context__, ()> {
            type Scalar;
        }
        impl<__Provider__, __Context__> ScalarTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ScalarTypeProviderComponent>
                + IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ScalarTypeProviderComponent,
            >>::Delegate: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <<__Provider__ as DelegateComponent<
                ScalarTypeProviderComponent,
            >>::Delegate as ScalarTypeProvider<__Context__>>::Scalar;
        }
        pub struct ScalarTypeProviderComponent;
        impl<__Context__> ScalarTypeProvider<__Context__> for UseContext
        where
            __Context__: HasScalarType,
        {
            type Scalar = <__Context__ as HasScalarType>::Scalar;
        }
        impl<__Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseContext
        where
            __Context__: HasScalarType,
        {}
        impl<__Context__, __Components__, __Path__> ScalarTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as ScalarTypeProvider<__Context__>>::Scalar;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
                + ScalarTypeProvider<__Context__>,
        {}
        impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar> {
            type Scalar = Scalar;
        }
        impl<Scalar, __Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseType<Scalar> {}
        impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
        {
            type Scalar = Scalar;
        }
        impl<
            __Provider__,
            Scalar,
            __Context__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
        {}
        ")
    }
}

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasTypes {
        type Types;
    }

    expand_has_types(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasTypes {
            type Types;
        }
        impl<__Context__> HasTypes for __Context__
        where
            __Context__: TypesTypeProvider<__Context__>,
        {
            type Types = <__Context__ as TypesTypeProvider<__Context__>>::Types;
        }
        pub trait TypesTypeProvider<
            __Context__,
        >: IsProviderFor<TypesTypeProviderComponent, __Context__, ()> {
            type Types;
        }
        impl<__Provider__, __Context__> TypesTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<TypesTypeProviderComponent>
                + IsProviderFor<TypesTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                TypesTypeProviderComponent,
            >>::Delegate: TypesTypeProvider<__Context__>,
        {
            type Types = <<__Provider__ as DelegateComponent<
                TypesTypeProviderComponent,
            >>::Delegate as TypesTypeProvider<__Context__>>::Types;
        }
        pub struct TypesTypeProviderComponent;
        impl<__Context__> TypesTypeProvider<__Context__> for UseContext
        where
            __Context__: HasTypes,
        {
            type Types = <__Context__ as HasTypes>::Types;
        }
        impl<__Context__> IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for UseContext
        where
            __Context__: HasTypes,
        {}
        impl<__Context__, __Components__, __Path__> TypesTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: TypesTypeProvider<__Context__>,
        {
            type Types = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as TypesTypeProvider<__Context__>>::Types;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
                + TypesTypeProvider<__Context__>,
        {}
        impl<Types, __Context__> TypesTypeProvider<__Context__> for UseType<Types> {
            type Types = Types;
        }
        impl<Types, __Context__> IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for UseType<Types> {}
        impl<__Provider__, Types, __Context__> TypesTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, TypesTypeProviderComponent, Type = Types>,
        {
            type Types = Types;
        }
        impl<
            __Provider__,
            Types,
            __Context__,
        > IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, TypesTypeProviderComponent, Type = Types>,
        {}
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasTypes::Types, @Types::HasScalarType::Scalar)]
    #[extend_where(Types: HasScalarType)]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar
    where
        Scalar: Mul<Output = Scalar> + Copy,
    {
        let res: Scalar = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasTypes
        where
            <Self as HasTypes>::Types: HasScalarType,
        {
            fn rectangle_area(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            <<Self as HasTypes>::Types as HasScalarType>::Scalar: Mul<
                    Output = <<Self as HasTypes>::Types as HasScalarType>::Scalar,
                > + Copy,
            <Self as HasTypes>::Types: HasScalarType,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = <<Self as HasTypes>::Types as HasScalarType>::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = <<Self as HasTypes>::Types as HasScalarType>::Scalar,
                >,
            Self: HasTypes,
            <Self as HasTypes>::Types: HasScalarType,
        {
            fn rectangle_area(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar {
                let width: <<Self as HasTypes>::Types as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: <<Self as HasTypes>::Types as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'h',
                                    Chars<
                                        'e',
                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone();
                let res: <<Self as HasTypes>::Types as HasScalarType>::Scalar = width * height;
                res
            }
        }
        ")
    }
}

pub struct MyTypes;

impl HasScalarType for MyTypes {
    type Scalar = f64;
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasTypes for Rectangle {
    type Types = MyTypes;
}

pub trait CheckRectangle: RectangleArea
where
    Self::Types: HasScalarType,
{
}

impl CheckRectangle for Rectangle {}
