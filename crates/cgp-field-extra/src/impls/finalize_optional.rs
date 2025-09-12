use alloc::string::String;

use cgp_field::traits::PartialData;

pub trait FinalizeOptional: PartialData {
    fn finalize_optional(self) -> Result<Self::Target, String>;
}
