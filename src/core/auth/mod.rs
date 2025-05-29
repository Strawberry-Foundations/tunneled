use crate::core::auth::strawberry_id::StrawberryId;

pub mod strawberry_id;
pub mod authenticator;
pub mod secret;

pub struct Auth;

impl Auth {
    pub fn strawberry_id() -> StrawberryId {
        StrawberryId {
            email: String::new(),
            full_name: String::new(),
            profile_picture: String::new(),
            username: String::new(),
            token: String::new(),
        }
    }
}
