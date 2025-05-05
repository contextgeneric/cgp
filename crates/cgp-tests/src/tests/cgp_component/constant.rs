use cgp::prelude::*;

pub fn test_component_with_const() {
    #[cgp_component {
        provider: ConstantGetter
    }]
    pub trait HasConstant {
        const CONSTANT: u64;
    }
}
