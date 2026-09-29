//! Two presets combine. `AppPreset: Types + Getters` inherits both, and listing
//! both preset entries on one table does the same. A later explicit entry
//! overrides a preset component.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_preset.md.

use cgp_fork::prelude::*;

#[cgp_type]
pub trait HasAgeType {
    type Age;
}

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

cgp_preset! {
    Types {
        type Age = u8,
    }
}

cgp_preset! {
    Getters {
        getter name,
    }
}

cgp_preset! {
    AppPreset: Types + Getters {}
}

#[derive(HasField)]
pub struct Inherited {
    pub name: String,
    pub age: u8,
}

delegate_components! {
    Inherited {
        preset AppPreset,
    }
}

#[derive(HasField)]
pub struct Listed {
    pub name: String,
    pub age: u8,
}

delegate_components! {
    Listed {
        preset Types,
        preset Getters,
    }
}

#[derive(HasField)]
pub struct Overridden {
    pub name: String,
    pub age: u16,
}

delegate_components! {
    Overridden {
        preset Types,
        type Age = u16,
        getter name,
    }
}

pub trait CheckInherited: HasAgeType<Age = u8> {}
impl CheckInherited for Inherited {}

pub trait CheckListed: HasAgeType<Age = u8> {}
impl CheckListed for Listed {}

pub trait CheckOverridden: HasAgeType<Age = u16> {}
impl CheckOverridden for Overridden {}

#[test]
fn combined_presets_read_the_name() {
    let inherited = Inherited {
        name: "Ada".to_owned(),
        age: 36,
    };
    let listed = Listed {
        name: "Ada".to_owned(),
        age: 36,
    };

    assert_eq!(inherited.name(), listed.name());
}
