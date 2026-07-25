//! `#[use_type]` importing a *foreign* abstract type into a `#[cgp_auto_getter]`
//! whose type parameter owns the type: `#[use_type(HasUserIdType.UserId in App)]`.
//!
//! This is the recommended form for a getter whose return type names an abstract
//! type living on another type (here the `App` parameter, not `Self`). The `in App`
//! clause rewrites the bare `UserId` to `<App as HasUserIdType>::UserId` and adds
//! `App: HasUserIdType` as a `where` bound on the generated trait, so the author
//! writes neither the bound nor the qualified `App::UserId` by hand. It replaces
//! the verbose form that declares `where App: HasUserIdType` and writes
//! `&Option<App::UserId>` at the use site.
//!
//! The derived blanket impl reads a `logged_in_user` field whose `HasField` value
//! type is that qualified associated type, so the context supplies the field while
//! `App` supplies the concrete `UserId`.
//!
//! See cgp-knowledge-base/cgp/guides/importing-abstract-types.md and
//! cgp-knowledge-base/cgp/reference/attributes/use_type.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasUserIdType {
    type UserId;
}

// Recommended: import the foreign type with `#[use_type]` and write the bare
// alias, rather than a hand-written `where App: HasUserIdType` plus `App::UserId`.
#[cgp_auto_getter]
#[use_type(HasUserIdType.UserId in App)]
pub trait HasLoggedInUser<App> {
    fn logged_in_user(&self) -> &Option<UserId>;
}

// `App` owns the concrete user-id type; it need not be the context.
pub struct App;

impl HasUserIdType for App {
    type UserId = u64;
}

#[derive(HasField)]
pub struct Server {
    pub logged_in_user: Option<u64>,
}

#[test]
fn test_logged_in_user() {
    let server = Server {
        logged_in_user: Some(7),
    };
    // `Server: HasLoggedInUser<App>` holds through the derived blanket impl, with
    // `UserId` grounded to `<App as HasUserIdType>::UserId` = `u64`. The `App`
    // parameter is named explicitly here since the getter is generic over it.
    assert_eq!(
        *<Server as HasLoggedInUser<App>>::logged_in_user(&server),
        Some(7u64)
    );
}
