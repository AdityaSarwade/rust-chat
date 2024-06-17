use crate::auth::constants::{UNAUTHORIZED, UNKNOWN, WRONG_REQUEST};
use crate::auth::helpers::{parse_id_and_find_user_by_id, FindUserById};
use crate::auth::token::request_access_token::AuthorizedUser;
use crate::db::connect_to_db::MongoDB;
use crate::ErrorResponse;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[delete("/user")]
pub async fn delete_user(
    database: &State<MongoDB>,
    auth: AuthorizedUser,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match parse_id_and_find_user_by_id(database, auth.user_id).await {
        FindUserById::Ok(user) => match database.delete_user(&user.username).await {
            Ok(_) => Ok(Status::NoContent),
            Err(_) => Err(UNKNOWN),
        },
        FindUserById::NoneUser => Err(WRONG_REQUEST),
        FindUserById::BadId => Err(UNAUTHORIZED),
    }
}
