use cgp::core::component::DefaultNamespace1;
use cgp::prelude::*;
use cgp_tests::namespaces::generics::{ShowImplComponent, ShowWithDisplay};

pub struct App;

delegate_components! {
    App {
        // use DefaultNamespace;
        // for <Component, Provider> in DefaultNamespace {
        //      Component: Provider,
        // }
        namespace default;

        for <T, Provider> in DefaultNamespace1<ShowImplComponent> {
            @test.ShowImplComponent.T: Provider,
        }

        @test.ShowImplComponent.u64:
            ShowWithDisplay,

        // namespace DefaultNamespace1<ShowImplComponent> => @test.ShowImplComponent;
    }
}

impl<T, Wildcard>
    DelegateComponent<PathCons<Symbol!("test"), PathCons<ShowImplComponent, PathCons<T, Wildcard>>>>
    for App
where
    T: DefaultNamespace1<ShowImplComponent, App>,
{
    type Delegate = T::Provider;
}

check_components! {
    App {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}
