use serde::Deserialize;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Validate, Deserialize)]
pub struct UserRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 18))]
    pub age: u8,
}