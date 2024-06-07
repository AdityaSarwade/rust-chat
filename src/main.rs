mod api;
mod auth;
mod db;
pub mod errors;
mod models;
mod plugins;
mod private;

#[macro_use]
extern crate rocket;

use crate::api::{
    auth::{
        delete_user::delete_user,
        get_data_user::get_data_user,
        hello_name::{hello_name_user, hello_world},
        login::login,
        patch_user::edit_user,
        refresh_tokens::refresh_tokens,
        registration::registration,
    },
    chat::{events, post},
    health_check::health_check,
};
// use crate::auth::constants::{UNAUTHORIZED, UNKNOWN};
use crate::errors::error_responses::{
    ErrorResponse, NOT_FOUND_JSON, UNAUTHORIZED_JSON, UNKNOWN_JSON,
};
use db::connect_to_db::init;
use models::message_model::Message;
use rocket::{
    fs::{relative, FileServer},
    http::Method,
    serde::json::Json,
    tokio::sync::broadcast::channel,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[launch]
async fn rocket() -> _ {
    // Load the environment variables
    private::load_env();
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    rocket::build()
        .attach(init().await)
        .manage(channel::<Message>(1024).0)
        .mount(
            "/",
            routes![
                health_check,
                post,
                events,
                registration,
                login,
                hello_name_user,
                hello_world,
                refresh_tokens,
                delete_user,
                edit_user,
                get_data_user
            ],
        )
        .mount("/", FileServer::from(relative!("static")))
        .manage(cors.to_cors())
        .register(
            "/",
            catchers![unauthorized, not_found, internal_sever_error,],
        )
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(UNAUTHORIZED_JSON)
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(NOT_FOUND_JSON)
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ErrorResponse> {
    Json(UNKNOWN_JSON)
}
