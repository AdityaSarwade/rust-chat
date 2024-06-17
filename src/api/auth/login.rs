use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::auth::LoginRequestError;
use crate::auth::constants::{LEN_USERNAME, LEN_PASSWORD, WRONG_REQUEST};
use crate::auth::validation::get_valid_username_and_password;
use crate::auth::TypeValidTwoStr;
use crate::db::connect_to_db::MongoDB;
use crate::db::LoginError;
use crate::errors::error_responses::ErrorResponse;
use crate::models::request::login_request::LoginRequest;
use crate::models::tokens::Token;

#[post("/login", format = "json", data = "<option_login_request>")]
pub async fn login(
    database: &State<MongoDB>,
    option_login_request: Option<Json<LoginRequest>>,
) -> Result<Json<Token>, (Status, Json<ErrorResponse>)> {
    match check_login_request(option_login_request) {
        LoginRequestError::Ok(login_request) => match login_match(database, login_request).await {
            Ok(tokens) => Ok(Json(tokens)),
            Err(_) => Err(WRONG_REQUEST),
        },
        LoginRequestError::NoneUsernameRequest => Err(WRONG_REQUEST),
        LoginRequestError::BadUsername => Err(WRONG_REQUEST),
        LoginRequestError::BadPassword => Err(WRONG_REQUEST),
    }
}

fn check_login_request(option_login_request: Option<Json<LoginRequest>>) -> LoginRequestError {
    match option_login_request {
        None => LoginRequestError::NoneUsernameRequest,
        Some(login_request) => {
            match get_valid_username_and_password(
                &login_request.username,
                &login_request.password,
                LEN_USERNAME,
                LEN_PASSWORD,
            ) {
                TypeValidTwoStr::Ok => LoginRequestError::Ok(login_request),
                TypeValidTwoStr::BadFirst => LoginRequestError::BadUsername,
                TypeValidTwoStr::BadSecond => LoginRequestError::BadPassword,
            }
        }
    }
}

async fn login_match(
    database: &State<MongoDB>,
    login_request: Json<LoginRequest>,
) -> Result<Token, ()> {
    match database.login(login_request).await {
        Ok(LoginError::Ok(tokens)) => Ok(tokens),
        Ok(LoginError::WrongPassword) => Err(()),
        Ok(LoginError::WrongUsername) => Err(()),
        Ok(LoginError::Unknown) => Err(()),
        Err(_) => Err(()),
    }
}
