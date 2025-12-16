use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    // Box<Self> should be replaced with `Box<__Context__>` in provider traits.
    fn greet(self: Box<Self>);
}
