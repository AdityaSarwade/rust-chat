use rocket::serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}
