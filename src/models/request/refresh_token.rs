use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RefreshToken {
    pub(crate) refresh_token: String,
}
