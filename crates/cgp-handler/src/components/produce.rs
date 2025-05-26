use core::marker::PhantomData;

use cgp_core::component::UseDelegate;
use cgp_core::prelude::*;

#[cgp_component(Producer)]
pub trait CanProduce<Tag> {
    type Output;

    fn produce(&self, _tag: PhantomData<Tag>) -> Self::Output;
}

#[cgp_provider]
impl<Context, Tag, Components, Delegate> Producer<Context, Tag> for UseDelegate<Components>
where
    Context: HasAsyncErrorType,
    Components: DelegateComponent<Tag, Delegate = Delegate>,
    Delegate: Producer<Context, Tag>,
    Tag: Send,
{
    type Output = Delegate::Output;

    fn produce(context: &Context, tag: PhantomData<Tag>) -> Self::Output {
        Delegate::produce(context, tag)
    }
}
