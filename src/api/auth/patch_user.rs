use crate::auth::constants::UNAUTHORIZED;
use crate::auth::constants::{
    LEN_FIRST_NAME, LEN_LAST_NAME, LEN_USERNAME, UNKNOWN, WEAK_USERNAME, WRONG_FIRST_NAME,
    WRONG_LAST_NAME, WRONG_MAIL, WRONG_REQUEST,
};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::auth::EditUserRequestError;
use crate::auth::helpers::{parse_id_and_find_user_by_id, FindUserById};
use crate::auth::token::request_access_token::AuthorizedUser;
use crate::auth::validation::{valid_edit_model, ValidEditModelError};
use crate::db::connect_to_db::MongoDB;
use crate::models::request::patch_request::EditUserRequest;
use crate::ErrorResponse;

//edit user data without id and password
#[patch("/user", data = "<option_edit_model>", format = "json")]
pub async fn edit_user(
    auth: AuthorizedUser,
    option_edit_model: Option<Json<EditUserRequest>>,
    database: &State<MongoDB>,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match parse_id_and_find_user_by_id(database, auth.user_id).await {
        FindUserById::Ok(user) => match check_edit_data_user_request(option_edit_model) {
            EditUserRequestError::Ok(edit_model) => {
                match database.edit_user(edit_model, user).await {
                    Ok(_) => Ok(Status::Ok),
                    Err(_) => Err(UNKNOWN),
                }
            }
            EditUserRequestError::NoneEditModel => Err(WRONG_REQUEST),
            EditUserRequestError::BadMail => Err(WRONG_MAIL),
            EditUserRequestError::BadUsername => Err(WEAK_USERNAME),
            EditUserRequestError::BadFirstName => Err(WRONG_FIRST_NAME),
            EditUserRequestError::BadLastName => Err(WRONG_LAST_NAME),
        },
        FindUserById::NoneUser => Err(UNAUTHORIZED),
        FindUserById::BadId => Err(UNAUTHORIZED),
    }
}

fn check_edit_data_user_request(
    option_edit_model: Option<Json<EditUserRequest>>,
) -> EditUserRequestError {
    match option_edit_model {
        None => EditUserRequestError::NoneEditModel,
        Some(edit_model) => {
            match valid_edit_model(&edit_model, LEN_FIRST_NAME, LEN_LAST_NAME, LEN_USERNAME) {
                ValidEditModelError::Ok => EditUserRequestError::Ok(edit_model),
                ValidEditModelError::BadMail => EditUserRequestError::BadMail,
                ValidEditModelError::BadUsername => EditUserRequestError::BadUsername,
                ValidEditModelError::BadFirstName => EditUserRequestError::BadFirstName,
                ValidEditModelError::BadLastName => EditUserRequestError::BadLastName,
            }
        }
    }
}
