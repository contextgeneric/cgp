use cgp::prelude::*;

#[cgp_component(ReferenceGetter)]
pub trait HasReference<'a, T: 'a + ?Sized> {
    fn get_reference(&self) -> &'a T;
}

#[cgp_provider]
impl<'a, Context, Tag, T: 'a + ?Sized> ReferenceGetter<'a, Context, T> for UseField<Tag>
where
    Context: HasField<Tag, Value = &'a T>,
{
    fn get_reference(context: &Context) -> &'a T {
        context.get_field(PhantomData)
    }
}

pub struct App<'a> {
    pub value: &'a str,
}

#[cgp_impl(new GetReference)]
impl<'a> ReferenceGetter<'a, str> for App<'a> {
    fn get_reference(&self) -> &'a str {
        self.value
    }
}

delegate_components! {
    <'a> App<'a> {
        ReferenceGetterComponent:
            GetReference,
    }
}

check_components! {
    <'a> App<'a> {
        ReferenceGetterComponent:
            (Life<'a>, str),
    }
}
