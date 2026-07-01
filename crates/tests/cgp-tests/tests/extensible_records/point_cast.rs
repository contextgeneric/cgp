//! Structural record casts with `build_with_default`.
//!
//! `build_with_default` builds a larger record from a smaller one, filling every
//! field the source lacks with its `Default` — so a `Point2d` casts up into a
//! `Point3d`/`Point4d` with the extra coordinates zeroed. The snapshot pins the
//! `#[derive(CgpData)]` expansion for the private `Point2d` (this concept owns
//! the derive's expansion); `Point3d`/`Point4d` are plain `CgpData` fixtures.
//!
//! See docs/concepts/extensible-records.md and
//! docs/reference/derives/derive_cgp_data.md.

use cgp::extra::field::impls::CanBuildWithDefault;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Point2d {
        x: u64,
        y: u64,
    }

    expand_point_2d(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<1, Chars<'x', Nil>>> for Point2d {
            type Value = u64;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'x', Nil>>>,
            ) -> &Self::Value {
                &self.x
            }
        }
        impl HasFieldMut<Symbol<1, Chars<'x', Nil>>> for Point2d {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'x', Nil>>>,
            ) -> &mut Self::Value {
                &mut self.x
            }
        }
        impl HasField<Symbol<1, Chars<'y', Nil>>> for Point2d {
            type Value = u64;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'y', Nil>>>,
            ) -> &Self::Value {
                &self.y
            }
        }
        impl HasFieldMut<Symbol<1, Chars<'y', Nil>>> for Point2d {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'y', Nil>>>,
            ) -> &mut Self::Value {
                &mut self.y
            }
        }
        impl HasFields for Point2d {
            type Fields = Cons<
                Field<Symbol<1, Chars<'x', Nil>>, u64>,
                Cons<Field<Symbol<1, Chars<'y', Nil>>, u64>, Nil>,
            >;
        }
        impl HasFieldsRef for Point2d {
            type FieldsRef<'__a> = Cons<
                Field<Symbol<1, Chars<'x', Nil>>, &'__a u64>,
                Cons<Field<Symbol<1, Chars<'y', Nil>>, &'__a u64>, Nil>,
            >
            where
                Self: '__a;
        }
        impl FromFields for Point2d {
            fn from_fields(Cons(x, Cons(y, Nil)): Self::Fields) -> Self {
                Self { x: x.value, y: y.value }
            }
        }
        impl ToFields for Point2d {
            fn to_fields(self) -> Self::Fields {
                Cons(self.x.into(), Cons(self.y.into(), Nil))
            }
        }
        impl ToFieldsRef for Point2d {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.x).into(), Cons((&self.y).into(), Nil))
            }
        }
        struct __PartialPoint2d<__F0__: MapType, __F1__: MapType> {
            x: <__F0__ as MapType>::Map<u64>,
            y: <__F1__ as MapType>::Map<u64>,
        }
        impl HasBuilder for Point2d {
            type Builder = __PartialPoint2d<IsNothing, IsNothing>;
            fn builder() -> Self::Builder {
                __PartialPoint2d { x: (), y: () }
            }
        }
        impl IntoBuilder for Point2d {
            type Builder = __PartialPoint2d<IsPresent, IsPresent>;
            fn into_builder(self) -> Self::Builder {
                __PartialPoint2d {
                    x: self.x,
                    y: self.y,
                }
            }
        }
        impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialPoint2d<__F0__, __F1__> {
            type Target = Point2d;
        }
        impl FinalizeBuild for __PartialPoint2d<IsPresent, IsPresent> {
            fn finalize_build(self) -> Self::Target {
                Point2d { x: self.x, y: self.y }
            }
        }
        impl<
            __M1__: MapType,
            __M2__: MapType,
            __F1__: MapType,
        > UpdateField<Symbol<1, Chars<'x', Nil>>, __M2__> for __PartialPoint2d<__M1__, __F1__> {
            type Value = u64;
            type Mapper = __M1__;
            type Output = __PartialPoint2d<__M2__, __F1__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<Symbol<1, Chars<'x', Nil>>>,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.x,
                    __PartialPoint2d {
                        x: value,
                        y: self.y,
                    },
                )
            }
        }
        impl<
            __F0__: MapType,
            __M1__: MapType,
            __M2__: MapType,
        > UpdateField<Symbol<1, Chars<'y', Nil>>, __M2__> for __PartialPoint2d<__F0__, __M1__> {
            type Value = u64;
            type Mapper = __M1__;
            type Output = __PartialPoint2d<__F0__, __M2__>;
            fn update_field(
                self,
                _tag: ::core::marker::PhantomData<Symbol<1, Chars<'y', Nil>>>,
                value: __M2__::Map<Self::Value>,
            ) -> (__M1__::Map<Self::Value>, Self::Output) {
                (
                    self.y,
                    __PartialPoint2d {
                        x: self.x,
                        y: value,
                    },
                )
            }
        }
        impl<__F1__: MapType> HasField<Symbol<1, Chars<'x', Nil>>>
        for __PartialPoint2d<IsPresent, __F1__> {
            type Value = u64;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<Symbol<1, Chars<'x', Nil>>>,
            ) -> &Self::Value {
                &self.x
            }
        }
        impl<__F0__: MapType> HasField<Symbol<1, Chars<'y', Nil>>>
        for __PartialPoint2d<__F0__, IsPresent> {
            type Value = u64;
            fn get_field(
                &self,
                tag: ::core::marker::PhantomData<Symbol<1, Chars<'y', Nil>>>,
            ) -> &Self::Value {
                &self.y
            }
        }
        ")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point3d {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point4d {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

#[test]
pub fn test_point_cast() {
    let point_2d = Point2d { x: 1, y: 2 };
    let point_3d = Point3d::build_with_default(point_2d.clone());
    let point_4d = Point4d::build_with_default(point_2d.clone());

    assert_eq!(point_3d, Point3d { x: 1, y: 2, z: 0 });

    assert_eq!(
        point_4d,
        Point4d {
            x: 1,
            y: 2,
            z: 0,
            w: 0,
        }
    );
}
