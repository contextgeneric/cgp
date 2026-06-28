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
        type Types: HasScalarType;
    }

    expand_has_types(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasTypes {
            type Types: HasScalarType;
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
            type Types: HasScalarType;
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
        impl<Types, __Context__> TypesTypeProvider<__Context__> for UseType<Types>
        where
            Types: HasScalarType,
        {
            type Types = Types;
        }
        impl<Types, __Context__> IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for UseType<Types>
        where
            Types: HasScalarType,
        {}
        impl<__Provider__, Types, __Context__> TypesTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            Types: HasScalarType,
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
            Types: HasScalarType,
            __Provider__: TypeProvider<__Context__, TypesTypeProviderComponent, Type = Types>,
        {}
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(
        HasTypes::Types,
        @Types::HasScalarType::{Scalar = f64},
    )]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        let res: f64 = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasTypes {
            fn rectangle_area(&self) -> <<Self as HasTypes>::Types as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
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
            <Self as HasTypes>::Types: HasScalarType<Scalar = f64>,
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
                let res: f64 = width * height;
                res
            }
        }
        ")
    }
}

pub trait HasFooType {
    type Foo: Ord + Clone;
}

pub trait HasBarType {
    type Bar: HasFooType;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasFooType::Foo)]
    pub fn do_foo(&self) -> Foo {
        todo!()
    }

    expand_do_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait DoFoo: HasFooType {
            fn do_foo(&self) -> <Self as HasFooType>::Foo;
        }
        impl<__Context__> DoFoo for __Context__
        where
            Self: HasFooType,
        {
            fn do_foo(&self) -> <Self as HasFooType>::Foo {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasBarType::Bar, @Bar::HasFooType::Foo)]
    pub fn do_bar(&self) -> Foo {
        todo!()
    }

    expand_do_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait DoBar: HasBarType {
            fn do_bar(&self) -> <<Self as HasBarType>::Bar as HasFooType>::Foo;
        }
        impl<__Context__> DoBar for __Context__
        where
            Self: HasBarType,
            <Self as HasBarType>::Bar: HasFooType,
        {
            fn do_bar(&self) -> <<Self as HasBarType>::Bar as HasFooType>::Foo {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(
        HasFooType::Foo,
        HasBarType::Bar,
        @Bar::HasFooType::{Foo as BarFoo = Foo},
    )]
    #[uses(DoFoo, DoBar)]
    fn return_foo_or_bar(&self, flag: bool, #[implicit] foo: &Foo, #[implicit] bar: &BarFoo) -> Foo {
        if flag {
            let res: Foo = self.do_foo();
            if &res < foo { res } else { foo.clone() }
        } else {
            let res: BarFoo = self.do_bar();
            if &res < bar { res } else { bar.clone() }
        }
    }

    expand_return_foo_or_bar(output) {
        insta::assert_snapshot!(output, @"
        trait ReturnFooOrBar: HasFooType + HasBarType {
            fn return_foo_or_bar(&self, flag: bool) -> <Self as HasFooType>::Foo;
        }
        impl<__Context__> ReturnFooOrBar for __Context__
        where
            Self: DoFoo + DoBar,
            Self: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = <Self as HasFooType>::Foo,
                >
                + HasField<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    Value = <<Self as HasBarType>::Bar as HasFooType>::Foo,
                >,
            Self: HasFooType,
            Self: HasBarType,
            <Self as HasBarType>::Bar: HasFooType<Foo = <Self as HasFooType>::Foo>,
        {
            fn return_foo_or_bar(&self, flag: bool) -> <Self as HasFooType>::Foo {
                let foo: &<Self as HasFooType>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    );
                let bar: &<<Self as HasBarType>::Bar as HasFooType>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                        >,
                    );
                if flag {
                    let res: <Self as HasFooType>::Foo = self.do_foo();
                    if &res < foo { res } else { foo.clone() }
                } else {
                    let res: <<Self as HasBarType>::Bar as HasFooType>::Foo = self.do_bar();
                    if &res < bar { res } else { bar.clone() }
                }
            }
        }
        ")
    }
}
