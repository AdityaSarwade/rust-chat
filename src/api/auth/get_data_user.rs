use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::auth::constants::UNAUTHORIZED;
use crate::auth::helpers::{parse_id_and_find_user_by_id, FindUserById};
use crate::auth::token::request_access_token::AuthorizedUser;
use crate::db::connect_to_db::MongoDB;
use crate::models::response::public_data_for_user::PublicDataForUser;
use crate::ErrorResponse;

#[get("/user")]
pub async fn get_data_user(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
) -> Result<Json<PublicDataForUser>, (Status, Json<ErrorResponse>)> {
    match parse_id_and_find_user_by_id(database, auth.user_id).await {
        FindUserById::Ok(user) => Ok(Json(PublicDataForUser {
            id: user._id.to_string().clone(),
            username: user.username.clone(),
            mail: user.mail.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
        })),
        FindUserById::NoneUser => Err(UNAUTHORIZED),
        FindUserById::BadId => Err(UNAUTHORIZED),
    }
}
