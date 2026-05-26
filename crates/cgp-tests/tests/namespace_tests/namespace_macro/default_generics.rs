use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;
use cgp_tests::namespaces::generics::{ShowImplComponent, ShowWithDisplay};

pub struct App;

delegate_components! {
    App {
        // use DefaultNamespace;
        // for <Component, Provider> in DefaultNamespace {
        //      Component: Provider,
        // }
        namespace DefaultNamespace;

        for <T, Provider> in DefaultImpls1<ShowImplComponent> {
            @test.ShowImplComponent.T: Provider,
        }

        @test.ShowImplComponent.u64:
            ShowWithDisplay,

        // namespace DefaultNamespace1<ShowImplComponent> => @test.ShowImplComponent;
    }
}

check_components! {
    App {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}
