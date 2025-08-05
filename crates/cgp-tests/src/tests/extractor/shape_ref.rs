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

pub trait CheckHasArea: HasAreaRef {}
impl CheckHasArea for Shape {}
impl CheckHasArea for ShapePlus {}

pub trait Container {
    fn contains(&self, x: f64, y: f64) -> bool;
}

impl Container for Circle {
    fn contains(&self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Rectangle {
    fn contains(&self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Triangle {
    fn contains(&self, _x: f64, _y: f64) -> bool {
        true // stub
    }
}

impl Container for Shape {
    fn contains(&self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlersRef::<Contains>::compute(&(), NoCode, (self, (x, y)))
    }
}

impl Container for ShapePlus {
    fn contains(&self, x: f64, y: f64) -> bool {
        MatchFirstWithValueHandlersRef::<Contains>::compute(&(), NoCode, (self, (x, y)))
    }
}

#[cgp_computer]
fn contains<T: Container>(shape: &T, (x, y): (f64, f64)) -> bool {
    shape.contains(x, y)
}
