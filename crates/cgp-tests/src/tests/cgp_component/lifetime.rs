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

#[cgp_context]
pub struct App<'a> {
    pub value: &'a str,
}

#[cgp_provider]
impl<'a> ReferenceGetter<'a, App<'a>, str> for AppComponents {
    fn get_reference(app: &App<'a>) -> &'a str {
        app.value
    }
}

check_components! {
    <'a> CanUseApp for App<'a> {
        ReferenceGetterComponent:
            (Life<'a>, str),
    }
}
