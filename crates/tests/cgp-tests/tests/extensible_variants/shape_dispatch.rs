//! Dispatching an owned extensible-variant value to per-variant handlers, over
//! an enum whose variants carry struct payloads. A `#[cgp_auto_dispatch]` trait
//! (`HasArea`) yields a `ComputeArea` provider that each variant's payload
//! implements; the dispatch combinators (`MatchWithHandlers`/
//! `MatchWithValueHandlers` with `ExtractFieldAndHandle`/`HandleFieldValue`, and
//! their `First` variants for handlers that take extra arguments) route each
//! variant to it, and `UseInputDelegate` selects a per-input-type provider.
//!
//! The dispatch combinators, `delegate_components!`, and `check_components!` are
//! owned by other concepts, so those macros appear plainly here; the derives are
//! plain scaffolding. This file exercises the variant side of owned dispatch.
//!
//! See docs/concepts/extensible-variants.md.

use core::marker::PhantomData;
use std::f64::consts::PI;

use cgp::extra::dispatch::{
    ExtractFieldAndHandle, ExtractFirstFieldAndHandle, HandleFieldValue, HandleFirstFieldValue,
    MatchFirstWithHandlers, MatchFirstWithValueHandlers, MatchWithHandlers, MatchWithValueHandlers,
};
use cgp::extra::handler::{NoCode, UseInputDelegate};
use cgp::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
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

check_components! {
    App {
        ComputerComponent: [
            ((), Shape),
            ((), ShapePlus),
        ],
    }
}
