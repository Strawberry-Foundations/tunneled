use crate::auth::strawberry_id::StrawberryId;

pub mod authenticator;
pub mod secret;
pub mod strawberry_id;

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
