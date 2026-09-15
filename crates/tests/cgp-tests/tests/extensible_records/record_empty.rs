//! `#[derive(CgpData)]` on a **fieldless struct**, the degenerate record shape.
//!
//! A struct with no fields is the record-side counterpart of the variantless
//! enum, and like it the expansion degenerates rather than failing: the
//! `__Partial…` companion struct takes no `MapType` parameters at all, so there
//! is exactly one configuration of it and `HasBuilder`, `IntoBuilder` and
//! `FinalizeBuild` all name the same type. `Fields` is the empty product `Nil`,
//! and no per-field `UpdateField`, `HasField`, or `HasFieldMut` impls are
//! emitted because there is no field to key one on.
//!
//! The practical consequence is that `builder()` is already finalizable — the
//! present/absent tracking that makes a premature `finalize_build` a compile
//! error has nothing to track.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_cgp_data.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct NoConfig {}

    expand_no_config(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for NoConfig {
            type Fields = Nil;
        }
        impl HasFieldsRef for NoConfig {
            type FieldsRef<'__a> = Nil where Self: '__a;
        }
        impl FromFields for NoConfig {
            fn from_fields(Nil: Self::Fields) -> Self {
                Self {}
            }
        }
        impl ToFields for NoConfig {
            fn to_fields(self) -> Self::Fields {
                Nil
            }
        }
        impl ToFieldsRef for NoConfig {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Nil
            }
        }
        pub struct __PartialNoConfig {}
        impl HasBuilder for NoConfig {
            type Builder = __PartialNoConfig;
            fn builder() -> Self::Builder {
                __PartialNoConfig {}
            }
        }
        impl IntoBuilder for NoConfig {
            type Builder = __PartialNoConfig;
            fn into_builder(self) -> Self::Builder {
                __PartialNoConfig {}
            }
        }
        impl PartialData for __PartialNoConfig {
            type Target = NoConfig;
        }
        impl FinalizeBuild for __PartialNoConfig {
            fn finalize_build(self) -> Self::Target {
                NoConfig {}
            }
        }
        ")
    }
}

#[test]
fn test_empty_builder_finalizes_immediately() {
    let config: NoConfig = NoConfig::builder().finalize_build();

    assert_eq!(config, NoConfig {});
}

#[test]
fn test_empty_record_round_trip() {
    let config1 = NoConfig {};

    let fields = config1.clone().to_fields();
    assert_eq!(fields, Nil);

    let config2 = NoConfig::from_fields(fields);
    assert_eq!(config1, config2);
}
