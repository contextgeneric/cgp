pub trait HasComponentAt<const I: usize, Params> {
    type Component;
}

pub type ComponentAt<Context, const I: usize, Params> =
    <Context as HasComponentAt<I, Params>>::Component;
