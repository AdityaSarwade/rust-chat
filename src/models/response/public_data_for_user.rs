use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicDataForUser {
    pub id: String,
    pub username: String,
    pub mail: String,
    pub first_name: String,
    pub last_name: String,
}
