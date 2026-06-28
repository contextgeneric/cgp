pub use core::marker::PhantomData;

pub use cgp_async_macro::async_trait;
pub use cgp_base::macro_prelude::{ConcatPath, PathCons};
pub use cgp_component::{
    CanUseComponent, DefaultNamespace, DelegateComponent, IsProviderFor, RedirectLookup,
    UseContext, UseDelegate, UseFields, WithContext, WithProvider,
};
pub use cgp_error::{CanRaiseError, CanWrapError, HasErrorType};
pub use cgp_field::impls::{IsMut, IsNothing, IsPresent, IsRef, IsVoid, UseField};
pub use cgp_field::traits::{
    BuildField, ExtractField, FieldGetter, FinalizeBuild, FinalizeExtract, FromFields, FromVariant,
    HasBuilder, HasExtractor, HasExtractorMut, HasExtractorRef, HasField, HasFieldMut, HasFields,
    HasFieldsRef, IntoBuilder, MapType, MapTypeRef, MutFieldGetter, PartialData, ToFields,
    ToFieldsRef, UpdateField,
};
pub use cgp_field::types::{Chars, Cons, Either, Field, Index, Life, Nil, Symbol, Void};
pub use cgp_macro::{
    BuildField, CgpData, CgpRecord, CgpVariant, ExtractField, FromVariant, HasField, HasFields,
    Product, Sum, Symbol, cgp_auto_getter, cgp_component, cgp_fn, cgp_getter, cgp_impl,
    cgp_namespace, cgp_new_provider, cgp_provider, cgp_type, check_components,
    delegate_and_check_components, delegate_components, product,
};
pub use cgp_type::{HasType, TypeProvider, UseType};
