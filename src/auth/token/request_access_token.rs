use crate::auth::helpers::check_data_from_auth_header;
use crate::auth::token::create_token::{decode_jwt, DecodeJwtHelper};
use crate::private;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct AuthorizedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match check_data_from_auth_header(auth_header) {
            Ok(vec_header) => match decode_jwt(vec_header[1].to_string(), *private::JWT_SECRET) {
                DecodeJwtHelper::Ok(token_data) => Outcome::Success(AuthorizedUser {
                    user_id: token_data.claims.user_id,
                }),
                DecodeJwtHelper::Err => Outcome::Error((Status::Unauthorized, ())),
            },
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
