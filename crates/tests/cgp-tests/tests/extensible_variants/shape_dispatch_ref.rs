//! Dispatching an extensible-variant value by *reference* and by *mutable
//! reference*. `HasExtractorMut`/`HasExtractorRef` expose the variants for
//! borrowing dispatch, and the `Ref`/`Mut` dispatch combinators
//! (`MatchFirstWithValueHandlersRef`/`Mut`, `MatchWithValueHandlersRef`) route
//! each borrowed variant to a per-variant handler — including a `&mut` handler
//! (`Scale`) that mutates the payload in place. A blanket `CanScale` impl over
//! any `HasExtractorMut` context shows dispatch driving a trait method.
//!
//! The dispatch combinators are owned elsewhere; the derives here are plain
//! scaffolding. This file exercises the by-reference / by-mut variant side.
//!
//! See docs/concepts/extensible-variants.md.

use std::f64::consts::PI;

use cgp::extra::dispatch::{
    MatchFirstWithValueHandlersMut, MatchFirstWithValueHandlersRef, MatchWithValueHandlersRef,
};
use cgp::extra::handler::NoCode;
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
pub trait HasAreaRef {
    fn area(&self) -> f64;
}

impl HasAreaRef for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl HasAreaRef for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl HasAreaRef for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

#[cgp_computer]
fn compute_area_ref<T: HasAreaRef>(shape: &T) -> f64 {
    shape.area()
}

#[cgp_auto_dispatch]
pub trait ContainerRef {
    fn contains_ref(&self, x: f64, y: f64) -> bool;
}

impl ContainerRef for Circle {
    fn contains_ref(&self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl ContainerRef for Rectangle {
    fn contains_ref(&self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl ContainerRef for Triangle {
    fn contains_ref(&self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

pub trait CanScale {
    fn scale(&mut self, factor: f64);
}

impl CanScale for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl CanScale for Rectangle {
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

impl CanScale for Triangle {
    fn scale(&mut self, factor: f64) {
        self.base *= factor;
        self.height *= factor;
    }
}

#[cgp_computer]
pub fn scale<T: CanScale>(shape: &mut T, factor: f64) {
    shape.scale(factor)
}

impl<Context> CanScale for Context
where
    Context: HasExtractorMut,
    MatchFirstWithValueHandlersMut<Scale>: for<'a> Computer<(), (), (&'a mut Context, f64)>,
{
    fn scale(&mut self, factor: f64) {
        MatchFirstWithValueHandlersMut::compute(&(), NoCode, (self, factor));
    }
}

pub trait CheckHasArea: HasAreaRef + ContainerRef + CanScale {}
impl CheckHasArea for Shape {}
impl CheckHasArea for ShapePlus {}
