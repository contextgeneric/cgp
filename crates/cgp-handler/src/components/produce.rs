use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component(Producer)]
pub trait CanProduce<Code> {
    type Output;

    fn produce(&self, _tag: PhantomData<Code>) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Components, Delegate> Producer<Context, Code> for UseDelegate<Components>
where
    Context: HasAsyncErrorType,
    Components: DelegateComponent<Code, Delegate = Delegate>,
    Delegate: Producer<Context, Code>,
    Code: Send,
{
    type Output = Delegate::Output;

    fn produce(context: &Context, tag: PhantomData<Code>) -> Self::Output {
        Delegate::produce(context, tag)
    }
}
