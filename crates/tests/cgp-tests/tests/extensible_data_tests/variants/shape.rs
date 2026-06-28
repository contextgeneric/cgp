use std::f64::consts::PI;

use cgp::core::field::impls::{CanDowncast, CanDowncastFields, CanUpcast};
use cgp::core::field::traits::FinalizeExtractResult;
use cgp::extra::dispatch::{
    ExtractFieldAndHandle, ExtractFirstFieldAndHandle, HandleFieldValue, HandleFirstFieldValue,
    MatchFirstWithHandlers, MatchFirstWithValueHandlers, MatchWithHandlers, MatchWithValueHandlers,
};
use cgp::extra::handler::{NoCode, UseInputDelegate};
use cgp::prelude::*;
use cgp_macro_test_util::{
    snapshot_check_components, snapshot_delegate_components, snapshot_derive_cgp_data,
};

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    #[derive(Debug, PartialEq)]
    pub enum Shape {
        Circle(Circle),
        Rectangle(Rectangle),
    }

    expand_shape(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Shape {
            type Fields = Either<
                Field<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                    Circle,
                >,
                Either<
                    Field<
                        Symbol<
                            9,
                            Chars<
                                'R',
                                Chars<
                                    'e',
                                    Chars<
                                        'c',
                                        Chars<
                                            't',
                                            Chars<
                                                'a',
                                                Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        Rectangle,
                    >,
                    Void,
                >,
            >;
        }
        impl HasFieldsRef for Shape {
            type FieldsRef<'__a> = Either<
                Field<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                    &'__a Circle,
                >,
                Either<
                    Field<
                        Symbol<
                            9,
                            Chars<
                                'R',
                                Chars<
                                    'e',
                                    Chars<
                                        'c',
                                        Chars<
                                            't',
                                            Chars<
                                                'a',
                                                Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        &'__a Rectangle,
                    >,
                    Void,
                >,
            >
            where
                Self: '__a;
        }
        impl FromFields for Shape {
            fn from_fields(rest: Self::Fields) -> Self {
                match rest {
                    Either::Left(field) => {
                        let field = field.value;
                        Self::Circle(field)
                    }
                    Either::Right(rest) => {
                        match rest {
                            Either::Left(field) => {
                                let field = field.value;
                                Self::Rectangle(field)
                            }
                            Either::Right(rest) => match rest {}
                        }
                    }
                }
            }
        }
        impl ToFields for Shape {
            fn to_fields(self) -> Self::Fields {
                match self {
                    Self::Circle(field) => Either::Left(field.into()),
                    Self::Rectangle(field) => Either::Right(Either::Left(field.into())),
                }
            }
        }
        impl ToFieldsRef for Shape {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                match self {
                    Self::Circle(field) => Either::Left(field.into()),
                    Self::Rectangle(field) => Either::Right(Either::Left(field.into())),
                }
            }
        }
        impl FromVariant<
            Symbol<
                6,
                Chars<'C', Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>>,
            >,
        > for Shape {
            type Value = Circle;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Circle(value)
            }
        }
        impl FromVariant<
            Symbol<
                9,
                Chars<
                    'R',
                    Chars<
                        'e',
                        Chars<
                            'c',
                            Chars<
                                't',
                                Chars<'a', Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        > for Shape {
            type Value = Rectangle;
            fn from_variant(
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'R',
                            Chars<
                                'e',
                                Chars<
                                    'c',
                                    Chars<
                                        't',
                                        Chars<
                                            'a',
                                            Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
                value: Self::Value,
            ) -> Self {
                Self::Rectangle(value)
            }
        }
        pub enum __PartialShape<__F0__: MapType, __F1__: MapType> {
            Circle(<__F0__ as MapType>::Map<Circle>),
            Rectangle(<__F1__ as MapType>::Map<Rectangle>),
        }
        pub enum __PartialRefShape<'__a__, __R__: MapTypeRef, __F0__: MapType, __F1__: MapType> {
            Circle(<__F0__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Circle>>),
            Rectangle(<__F1__ as MapType>::Map<<__R__ as MapTypeRef>::Map<'__a__, Rectangle>>),
        }
        impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialShape<__F0__, __F1__> {
            type Target = Shape;
        }
        impl<'__a__, __R__: MapTypeRef, __F0__: MapType, __F1__: MapType> PartialData
        for __PartialRefShape<'__a__, __R__, __F0__, __F1__> {
            type Target = Shape;
        }
        impl HasExtractor for Shape {
            type Extractor = __PartialShape<IsPresent, IsPresent>;
            fn to_extractor(self) -> Self::Extractor {
                match self {
                    Self::Circle(value) => __PartialShape::Circle(value),
                    Self::Rectangle(value) => __PartialShape::Rectangle(value),
                }
            }
            fn from_extractor(extractor: Self::Extractor) -> Self {
                match extractor {
                    __PartialShape::Circle(value) => Self::Circle(value),
                    __PartialShape::Rectangle(value) => Self::Rectangle(value),
                }
            }
        }
        impl HasExtractorRef for Shape {
            type ExtractorRef<'a> = __PartialRefShape<'a, IsRef, IsPresent, IsPresent>
            where
                Self: 'a;
            fn extractor_ref<'a>(&'a self) -> Self::ExtractorRef<'a> {
                match self {
                    Self::Circle(value) => __PartialRefShape::Circle(value),
                    Self::Rectangle(value) => __PartialRefShape::Rectangle(value),
                }
            }
        }
        impl HasExtractorMut for Shape {
            type ExtractorMut<'a> = __PartialRefShape<'a, IsMut, IsPresent, IsPresent>
            where
                Self: 'a;
            fn extractor_mut<'a>(&'a mut self) -> Self::ExtractorMut<'a> {
                match self {
                    Self::Circle(value) => __PartialRefShape::Circle(value),
                    Self::Rectangle(value) => __PartialRefShape::Rectangle(value),
                }
            }
        }
        impl FinalizeExtract for __PartialShape<IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<'a, __R__: MapTypeRef> FinalizeExtract
        for __PartialRefShape<'a, __R__, IsVoid, IsVoid> {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl<
            __F1__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'C', Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>>,
            >,
        > for __PartialShape<IsPresent, __F1__> {
            type Value = Circle;
            type Remainder = __PartialShape<IsVoid, __F1__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialShape::Circle(value) => Ok(value),
                    __PartialShape::Rectangle(value) => Err(__PartialShape::Rectangle(value)),
                }
            }
        }
        impl<
            __F0__: MapType,
        > ExtractField<
            Symbol<
                9,
                Chars<
                    'R',
                    Chars<
                        'e',
                        Chars<
                            'c',
                            Chars<
                                't',
                                Chars<'a', Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        > for __PartialShape<__F0__, IsPresent> {
            type Value = Rectangle;
            type Remainder = __PartialShape<__F0__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'R',
                            Chars<
                                'e',
                                Chars<
                                    'c',
                                    Chars<
                                        't',
                                        Chars<
                                            'a',
                                            Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialShape::Circle(value) => Err(__PartialShape::Circle(value)),
                    __PartialShape::Rectangle(value) => Ok(value),
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F1__: MapType,
        > ExtractField<
            Symbol<
                6,
                Chars<'C', Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>>,
            >,
        > for __PartialRefShape<'__a__, __R__, IsPresent, __F1__> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Circle>;
            type Remainder = __PartialRefShape<'__a__, __R__, IsVoid, __F1__>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        6,
                        Chars<
                            'C',
                            Chars<'i', Chars<'r', Chars<'c', Chars<'l', Chars<'e', Nil>>>>>,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefShape::Circle(value) => Ok(value),
                    __PartialRefShape::Rectangle(value) => {
                        Err(__PartialRefShape::Rectangle(value))
                    }
                }
            }
        }
        impl<
            '__a__,
            __R__: MapTypeRef,
            __F0__: MapType,
        > ExtractField<
            Symbol<
                9,
                Chars<
                    'R',
                    Chars<
                        'e',
                        Chars<
                            'c',
                            Chars<
                                't',
                                Chars<'a', Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        > for __PartialRefShape<'__a__, __R__, __F0__, IsPresent> {
            type Value = <__R__ as MapTypeRef>::Map<'__a__, Rectangle>;
            type Remainder = __PartialRefShape<'__a__, __R__, __F0__, IsVoid>;
            fn extract_field(
                self,
                _tag: ::core::marker::PhantomData<
                    Symbol<
                        9,
                        Chars<
                            'R',
                            Chars<
                                'e',
                                Chars<
                                    'c',
                                    Chars<
                                        't',
                                        Chars<
                                            'a',
                                            Chars<'n', Chars<'g', Chars<'l', Chars<'e', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            ) -> Result<Self::Value, Self::Remainder> {
                match self {
                    __PartialRefShape::Circle(value) => Err(__PartialRefShape::Circle(value)),
                    __PartialRefShape::Rectangle(value) => Ok(value),
                }
            }
        }
        ")
    }
}

#[derive(Debug, PartialEq, CgpData)]
pub enum TriangleOnly {
    Triangle(Triangle),
}

#[derive(Debug, PartialEq, CgpData)]
pub enum ShapePlus {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

#[test]
fn test_shape_area() {
    let shape = Shape::Circle(Circle { radius: 5.0 });

    let _area = match shape
        .to_extractor() // PartialShape<IsPresent, IsPresent>
        .extract_field(PhantomData::<Symbol!("Circle")>)
    {
        Ok(circle) => PI * circle.radius * circle.radius,
        // PartialShape<IsVoid, IsPresent>
        Err(remainder) => {
            let rectangle = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();

            rectangle.width * rectangle.height
        }
    };
}

#[test]
fn test_shape_upcast() {
    let shape = Shape::Circle(Circle { radius: 5.0 });
    let shape_plus = shape.upcast(PhantomData::<ShapePlus>);
    assert_eq!(shape_plus, ShapePlus::Circle(Circle { radius: 5.0 }));
}

#[test]
fn test_shape_downcast() {
    let shape = ShapePlus::Circle(Circle { radius: 5.0 });
    assert_eq!(
        shape.downcast(PhantomData::<Shape>).ok(),
        Some(Shape::Circle(Circle { radius: 5.0 }))
    );

    let shape_plus = ShapePlus::Triangle(Triangle {
        base: 3.0,
        height: 4.0,
    });

    let _area = match shape_plus.downcast(PhantomData::<Shape>) {
        Ok(shape) => match shape {
            Shape::Circle(circle) => PI * circle.radius * circle.radius,
            Shape::Rectangle(rectangle) => rectangle.width * rectangle.height,
        },
        // PartialShapePlus<IsPresent, IsVoid, IsVoid>
        Err(remainder) => {
            let TriangleOnly::Triangle(triangle) = remainder
                .downcast_fields(PhantomData::<TriangleOnly>)
                .finalize_extract_result();
            triangle.base * triangle.height / 2.0
        }
    };
}

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(self) -> f64;
}

impl HasArea for Circle {
    fn area(self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(self) -> f64 {
        self.width * self.height
    }
}

impl HasArea for Triangle {
    fn area(self) -> f64 {
        self.base * self.height / 2.0
    }
}

#[test]
fn test_match_with_handlers() {
    let circle = Shape::Circle(Circle { radius: 5.0 });

    let _area = MatchWithHandlers::<
        Product![
            ExtractFieldAndHandle<Symbol!("Circle"), HandleFieldValue<ComputeArea>>,
            ExtractFieldAndHandle<Symbol!("Rectangle"), HandleFieldValue<ComputeArea>>,
        ],
    >::compute(&(), PhantomData::<()>, circle);
}

pub trait Container {
    fn contains(self, x: f64, y: f64) -> bool;
}

impl Container for Circle {
    fn contains(self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Rectangle {
    fn contains(self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Triangle {
    fn contains(self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Shape {
    fn contains(self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlers::<Contains>::compute(&(), NoCode, (self, (x, y)))
    }
}

impl Container for ShapePlus {
    fn contains(self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlers::<Contains>::compute(&(), NoCode, (self, (x, y)))
    }
}

#[cgp_computer]
fn contains<T: Container>(shape: T, (x, y): (f64, f64)) -> bool {
    shape.contains(x, y)
}

#[test]
fn test_dispatch_contains() {
    let circle = Shape::Circle(Circle { radius: 5.0 });

    let _is_contained = MatchFirstWithHandlers::<
        Product![
            ExtractFirstFieldAndHandle<Symbol!("Circle"), HandleFirstFieldValue<Contains>>,
            ExtractFirstFieldAndHandle<Symbol!("Rectangle"), HandleFirstFieldValue<Contains>>,
        ],
    >::compute(&(), PhantomData::<()>, (circle, (1.0, 2.0)));
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            ComputerComponent: UseInputDelegate<new AreaComputers {
                [
                    Circle,
                    Rectangle,
                    Triangle,
                ]:
                    ComputeArea,
                [
                    Shape,
                    ShapePlus,
                ]: MatchWithValueHandlers,
            }>
        }
    }

    expand_app(output) {
        insta::assert_snapshot!(output, @"
        pub struct AreaComputers;
        impl DelegateComponent<ComputerComponent> for App {
            type Delegate = UseInputDelegate<AreaComputers>;
        }
        impl<__Context__, __Params__> IsProviderFor<ComputerComponent, __Context__, __Params__>
        for App
        where
            UseInputDelegate<
                AreaComputers,
            >: IsProviderFor<ComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<Circle> for AreaComputers {
            type Delegate = ComputeArea;
        }
        impl<__Context__, __Params__> IsProviderFor<Circle, __Context__, __Params__>
        for AreaComputers
        where
            ComputeArea: IsProviderFor<Circle, __Context__, __Params__>,
        {}
        impl DelegateComponent<Rectangle> for AreaComputers {
            type Delegate = ComputeArea;
        }
        impl<__Context__, __Params__> IsProviderFor<Rectangle, __Context__, __Params__>
        for AreaComputers
        where
            ComputeArea: IsProviderFor<Rectangle, __Context__, __Params__>,
        {}
        impl DelegateComponent<Triangle> for AreaComputers {
            type Delegate = ComputeArea;
        }
        impl<__Context__, __Params__> IsProviderFor<Triangle, __Context__, __Params__>
        for AreaComputers
        where
            ComputeArea: IsProviderFor<Triangle, __Context__, __Params__>,
        {}
        impl DelegateComponent<Shape> for AreaComputers {
            type Delegate = MatchWithValueHandlers;
        }
        impl<__Context__, __Params__> IsProviderFor<Shape, __Context__, __Params__>
        for AreaComputers
        where
            MatchWithValueHandlers: IsProviderFor<Shape, __Context__, __Params__>,
        {}
        impl DelegateComponent<ShapePlus> for AreaComputers {
            type Delegate = MatchWithValueHandlers;
        }
        impl<__Context__, __Params__> IsProviderFor<ShapePlus, __Context__, __Params__>
        for AreaComputers
        where
            MatchWithValueHandlers: IsProviderFor<ShapePlus, __Context__, __Params__>,
        {}
        ")
    }
}

snapshot_check_components! {
    check_components! {
        App {
            ComputerComponent: [
                ((), Shape),
                ((), ShapePlus),
            ],
        }
    }

    expand_check_app(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckApp<ComputerComponent, ((), Shape)> for App {}
        impl __CheckApp<ComputerComponent, ((), ShapePlus)> for App {}
        ")
    }
}
