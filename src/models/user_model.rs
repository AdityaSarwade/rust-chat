use mongodb::bson::oid::ObjectId;
use rocket::serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub _id: ObjectId,

    pub username: String,
    pub password: String,

    pub mail: String,

    pub first_name: String,
    pub last_name: String,
}
