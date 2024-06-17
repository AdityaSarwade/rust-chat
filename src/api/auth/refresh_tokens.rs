use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::auth::constants::{EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN, UNAUTHORIZED};
use crate::auth::helpers::object_id_parse_str;
use crate::auth::token::create_token::{decode_jwt, encode_token_and_refresh, DecodeJwtHelper};
use crate::models::request::refresh_token::RefreshToken;
use crate::models::tokens::Token;
use crate::private;

use crate::db::connect_to_db::MongoDB;
use crate::ErrorResponse;

//refresh_tokens
#[post("/refresh-token", format = "json", data = "<option_refresh_token>")]
pub async fn refresh_tokens(
    database: &State<MongoDB>,
    option_refresh_token: Option<Json<RefreshToken>>,
) -> Result<Json<Token>, (Status, Json<ErrorResponse>)> {
    match option_refresh_token {
        Some(refresh_token) => match decode_jwt_return_id(refresh_token) {
            Ok(id) => match encode_token(database, id).await {
                Ok(token) => Ok(Json(token)),
                Err(_) => Err(UNAUTHORIZED),
            },
            Err(_) => Err(UNAUTHORIZED),
        },
        None => Err(UNAUTHORIZED),
    }
}

//encode prepare data
async fn encode_token(database: &State<MongoDB>, id: ObjectId) -> Result<Token, ()> {
    match database.find_user_by_id(id).await {
        Ok(Some(_)) => {
            match encode_token_and_refresh(
                id.clone(),
                *private::JWT_SECRET,
                *private::REFRESH_JWT_SECRET,
                EXPIRATION_REFRESH_TOKEN,
                EXPIRATION_TOKEN,
            ) {
                Ok(tokens) => Ok(Token {
                    token: tokens.token,
                    refresh_token: tokens.refresh_token,
                }),
                Err(_) => Err(()),
            }
        }
        Ok(None) => Err(()),
        Err(_) => Err(()),
    }
}

//decode jwt from body and return id
fn decode_jwt_return_id(refresh_token: Json<RefreshToken>) -> Result<ObjectId, ()> {
    match decode_jwt(
        refresh_token.refresh_token.to_string(),
        *private::REFRESH_JWT_SECRET,
    ) {
        DecodeJwtHelper::Ok(token_data) => {
            let id_str = token_data.claims.user_id;
            match object_id_parse_str(id_str) {
                Ok(id) => Ok(id),
                Err(_) => Err(()),
            }
        }
        DecodeJwtHelper::Err => Err(()),
    }
}
