mod api;
mod models;
mod repository;
mod plugins;

#[macro_use]
extern crate rocket;

use crate::api::chat::{events, health_check, post};
use models::message_model::Message;
use rocket::{
    fs::{relative, FileServer},
    tokio::sync::broadcast::channel,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![health_check, post, events])
        .mount("/", FileServer::from(relative!("static")))
}
