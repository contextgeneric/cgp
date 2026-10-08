mod has_runtime;
mod has_runtime_type;

pub use has_runtime::{HasRuntime, RuntimeGetter, RuntimeGetterComponent};
pub use has_runtime_type::{
    HasRuntimeType, RuntimeOf, RuntimeTypeProvider, RuntimeTypeProviderComponent,
};
