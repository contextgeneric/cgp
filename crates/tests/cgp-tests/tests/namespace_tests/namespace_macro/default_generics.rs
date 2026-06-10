use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;
use cgp_tests::namespaces::default_impl_a::{
    DefaultShowComponents, ShowImplComponent, ShowWithDisplay,
};

pub struct AppA;

delegate_components! {
    AppA {
        namespace DefaultNamespace;

        for <T, Provider> in DefaultImpls1<ShowImplComponent> {
            @test.ShowImplComponent.T: Provider,
        }

        @test.ShowImplComponent.u64:
            ShowWithDisplay,
    }
}

check_components! {
    AppA {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}

pub struct AppB;

delegate_components! {
    AppB {
        namespace DefaultNamespace;

        for <T, Provider> in DefaultShowComponents {
            @test.ShowImplComponent.T: Provider,
        }
    }
}

check_components! {
    AppB {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}
