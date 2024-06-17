use crate::models::request::{
    login_request::LoginRequest, patch_request::EditUserRequest,
    registration_request::RegistrationRequest,
};
use rocket::serde::json::Json;

pub mod delete_user;
pub mod get_data_user;
pub mod hello_name;
pub mod login;
pub mod patch_user;
pub mod refresh_tokens;
pub mod registration;

enum HelloNameError {
    OnlyUsername(String),
    NameAndUsername(String),
    ErrorID,
}

pub enum EditUserRequestError {
    Ok(Json<EditUserRequest>),
    NoneEditModel,
    BadMail,
    BadUsername,
    BadFirstName,
    BadLastName,
}

pub enum RegistrationRequestError {
    Ok(Json<RegistrationRequest>),
    NoneRegistrationRequest,
    BadFirstName,
    BadLastName,
    BadUsername,
    BadPassword,
    BadMail,
}

pub enum LoginRequestError {
    Ok(Json<LoginRequest>),
    NoneUsernameRequest,
    BadUsername,
    BadPassword,
}
