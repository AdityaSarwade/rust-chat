use crate::api::auth::HelloNameError;
use crate::auth::constants::UNAUTHORIZED;
use crate::auth::helpers::{parse_id_and_find_user_by_id, FindUserById};
use crate::db::connect_to_db::MongoDB;
use crate::models::hello_response::HelloNameResponse;
use crate::ErrorResponse;
use rocket::http::Status;

use crate::auth::token::request_access_token::AuthorizedUser;
use rocket::serde::json::Json;
use rocket::State;

//(private) request with authorization model (token)
#[get("/private/hello")]
pub async fn hello_name_user(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
) -> Result<Json<HelloNameResponse>, (Status, Json<ErrorResponse>)> {
    match check_from_db_real_names(database, auth.user_id).await {
        HelloNameError::OnlyUsername(res_only_username) => Ok(Json(HelloNameResponse {
            greetings: res_only_username,
        })),
        HelloNameError::NameAndUsername(res_no_only_username) => Ok(Json(HelloNameResponse {
            greetings: res_no_only_username,
        })),
        HelloNameError::ErrorID => Err(UNAUTHORIZED),
    }
}

//we check if the first and last names are in the database
async fn check_from_db_real_names(database: &State<MongoDB>, id_str: String) -> HelloNameError {
    match parse_id_and_find_user_by_id(database, id_str).await {
        FindUserById::Ok(user) => {
            if user.first_name == "" || user.last_name == "" {
                HelloNameError::OnlyUsername(format!("Hello {}", user.username,))
            } else {
                HelloNameError::NameAndUsername(format!(
                    "Hello {} <{}> {}",
                    user.first_name, user.username, user.last_name
                ))
            }
        }
        FindUserById::NoneUser => HelloNameError::ErrorID,
        FindUserById::BadId => HelloNameError::ErrorID,
    }
}

//(public) hello world
#[get("/public/hello")]
pub async fn hello_world() -> Json<&'static str> {
    Json("Hello world")
}
