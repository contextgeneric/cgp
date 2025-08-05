use std::f64::consts::PI;

use cgp::extra::dispatch::{MatchFirstWithValueHandlersRef, MatchWithValueHandlersRef};
use cgp::extra::handler::{ComputerRef, NoCode};
use cgp::prelude::*;

use crate::tests::extractor::shape::{Circle, Rectangle, Shape, ShapePlus, Triangle};

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

impl<Context> HasAreaRef for Context
where
    Context: HasExtractorRef,
    MatchWithValueHandlersRef<ComputeAreaRef>: ComputerRef<(), (), Context, Output = f64>,
{
    fn area(&self) -> f64 {
        MatchWithValueHandlersRef::<ComputeAreaRef>::compute_ref(&(), NoCode, self)
    }
}

#[cgp_computer]
fn compute_area_ref<T: HasAreaRef>(shape: &T) -> f64 {
    shape.area()
}

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

impl<Context> ContainerRef for Context
where
    Context: HasExtractorRef,
    MatchFirstWithValueHandlersRef<ContainsRef>:
        for<'a> Computer<(), (), (&'a Context, (f64, f64)), Output = bool>,
{
    fn contains_ref(&self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlersRef::compute(&(), NoCode, (self, (x, y)))
    }
}

#[cgp_computer]
fn contains_ref<T: ContainerRef>(shape: &T, (x, y): (f64, f64)) -> bool {
    shape.contains_ref(x, y)
}

pub trait CheckHasArea: HasAreaRef + ContainerRef {}
impl CheckHasArea for Shape {}
impl CheckHasArea for ShapePlus {}
