use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::auth::RegistrationRequestError;
use crate::auth::constants::{
    ALREADY_REGISTERED_MAIL, ALREADY_REGISTERED_USERNAME, LEN_FIRST_NAME, LEN_LAST_NAME,
    LEN_PASSWORD, LEN_USERNAME, UNKNOWN, WEAK_PASSWORD, WEAK_USERNAME, WRONG_FIRST_NAME,
    WRONG_LAST_NAME, WRONG_MAIL, WRONG_REQUEST,
};
use crate::auth::validation::valid_registration_data_user;
use crate::auth::TypeValidDataFromRegistration;
use crate::db::connect_to_db::MongoDB;
use crate::db::RegistrationError;
use crate::errors::error_responses::ErrorResponse;
use crate::models::request::registration_request::RegistrationRequest;
use crate::models::tokens::Token;

#[post(
    "/registration",
    format = "json",
    data = "<option_registration_request>"
)]
pub async fn registration(
    database: &State<MongoDB>,
    option_registration_request: Option<Json<RegistrationRequest>>,
) -> Result<Json<Token>, (Status, Json<ErrorResponse>)> {
    match check_registration_request(option_registration_request) {
        RegistrationRequestError::Ok(registration_request) => {
            match database.registration(registration_request).await {
                Ok(RegistrationError::Ok(token)) => Ok(Json(Token {
                    token: token.token,
                    refresh_token: token.refresh_token,
                })),
                Ok(RegistrationError::AlreadyRegisteredByEmail) => Err(ALREADY_REGISTERED_MAIL),
                Ok(RegistrationError::AlreadyRegisteredByUsername) => {
                    Err(ALREADY_REGISTERED_USERNAME)
                }
                Ok(RegistrationError::WrongPassword) => Err(WEAK_PASSWORD),
                Ok(RegistrationError::Unknown) => Err(UNKNOWN),
                Err(_) => Err(UNKNOWN),
            }
        }
        RegistrationRequestError::NoneRegistrationRequest => Err(WRONG_REQUEST),
        RegistrationRequestError::BadFirstName => Err(WRONG_FIRST_NAME),
        RegistrationRequestError::BadLastName => Err(WRONG_LAST_NAME),
        RegistrationRequestError::BadUsername => Err(WEAK_USERNAME),
        RegistrationRequestError::BadPassword => Err(WEAK_PASSWORD),
        RegistrationRequestError::BadMail => Err(WRONG_MAIL),
    }
}

fn check_registration_request(
    option_registration_request: Option<Json<RegistrationRequest>>,
) -> RegistrationRequestError {
    match option_registration_request {
        None => RegistrationRequestError::NoneRegistrationRequest,
        Some(registration_request) => {
            match valid_registration_data_user(
                &registration_request,
                LEN_FIRST_NAME,
                LEN_LAST_NAME,
                LEN_USERNAME,
                LEN_PASSWORD,
            ) {
                TypeValidDataFromRegistration::Ok => {
                    RegistrationRequestError::Ok(registration_request)
                }
                TypeValidDataFromRegistration::BadFirstName => {
                    RegistrationRequestError::BadFirstName
                }
                TypeValidDataFromRegistration::BadLastName => RegistrationRequestError::BadLastName,
                TypeValidDataFromRegistration::BadUsername => RegistrationRequestError::BadUsername,
                TypeValidDataFromRegistration::BadPassword => RegistrationRequestError::BadPassword,
                TypeValidDataFromRegistration::BadMail => RegistrationRequestError::BadMail,
            }
        }
    }
}
