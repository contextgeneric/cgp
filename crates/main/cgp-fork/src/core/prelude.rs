pub use core::marker::PhantomData;

pub use cgp_fork_macro::{
    BuildField, CgpData, CgpRecord, CgpVariant, Enum, ExtractField, FromVariant, HasField,
    HasFields, Path, Product, Struct, Sum, Symbol, async_trait, cgp_auto_error, cgp_auto_getter,
    cgp_auto_impl, cgp_component, cgp_fn, cgp_for_each, cgp_getter, cgp_impl, cgp_namespace,
    cgp_new_provider, cgp_preset, cgp_provider, cgp_type, check_components,
    delegate_and_check_components, delegate_components, derive_provider, product,
};

pub use crate::core::base::macro_prelude::{ConcatPath, PathCons};
pub use crate::core::component::{
    CanUseComponent, DefaultNamespace, DelegateComponent, IsProviderFor, RedirectLookup,
    UseContext, UseDelegate, UseFields, WithContext, WithProvider,
};
pub use crate::core::error::{CanRaiseError, CanWrapError, HasErrorType};
pub use crate::core::field::impls::{IsMut, IsNothing, IsPresent, IsRef, IsVoid, UseField};
pub use crate::core::field::traits::{
    BuildField, ExtractField, FieldGetter, FinalizeBuild, FinalizeExtract, FromFields, FromVariant,
    HasBuilder, HasExtractor, HasExtractorMut, HasExtractorRef, HasField, HasFieldMut, HasFields,
    HasFieldsRef, IntoBuilder, MapType, MapTypeRef, MutFieldGetter, PartialData, ToFields,
    ToFieldsRef, UpdateField,
};
pub use crate::core::field::types::{
    Chars, Cons, Either, Field, Index, Life, MRef, Nil, Symbol, Void,
};
pub use crate::core::types::{HasType, TypeProvider, UseType};
