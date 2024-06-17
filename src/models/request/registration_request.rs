use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,

    pub mail: String,

    pub first_name: String,
    pub last_name: String,
}
