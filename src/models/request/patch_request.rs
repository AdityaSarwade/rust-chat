use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EditUserRequest {
    pub username: String,

    pub mail: String,

    pub first_name: String,
    pub last_name: String,
}
