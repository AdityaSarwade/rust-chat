use bcrypt::verify;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use rocket::serde::json::Json;

use crate::auth::token::create_token::encode_token_and_refresh;
use crate::auth::constants::{EXPIRATION_REFRESH_TOKEN, EXPIRATION_TOKEN};
use crate::auth::helpers::{find_user_by_username_and_mail, hash_text};
use crate::db::connect_to_db::MongoDB;
use crate::db::{FindUserBy, LoginError, RegistrationError};
use crate::models::request::login_request::LoginRequest;
use crate::models::request::patch_request::EditUserRequest;
use crate::models::request::registration_request::RegistrationRequest;
use crate::models::user_model::User;
use crate::private;

impl MongoDB {
    pub fn new(database: Database) -> Self {
        MongoDB { database }
    }

    pub async fn edit_user(
        &self,
        edit_model: Json<EditUserRequest>,
        user: User,
    ) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<User>("user");
        dbg!(
            collection
                .find_one_and_replace(
                    bson::doc! { "_id": user._id },
                    User {
                        _id: user._id,
                        username: edit_model.username.clone(),
                        password: user.password,
                        mail: edit_model.mail.clone(),
                        first_name: edit_model.first_name.clone(),
                        last_name: edit_model.last_name.clone()
                    },
                    None
                )
                .await?
        );
        Ok(())
    }

    pub async fn delete_user(&self, username: &str) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<User>("user");
        collection
            .delete_one(bson::doc! { "username": username }, None)
            .await?;
        Ok(())
    }

    pub async fn find_user_by(
        &self,
        find_by: &str,
        data_find_in: &str,
    ) -> mongodb::error::Result<Option<User>> {
        let collection_user = self.database.collection::<User>("user");

        Ok(collection_user
            .find_one(bson::doc! { find_by: data_find_in }, None)
            .await?)
    }

    pub async fn find_user_by_id(
        &self,
        data_find_in: ObjectId,
    ) -> mongodb::error::Result<Option<User>> {
        let collection_user = self.database.collection::<User>("user");

        Ok(collection_user
            .find_one(bson::doc! { "_id": data_find_in }, None)
            .await?)
    }

    pub async fn login(
        &self,
        login_request: Json<LoginRequest>,
    ) -> mongodb::error::Result<LoginError> {
        match Self::find_user_by(self, "username", &login_request.username).await {
            Ok(option_user) => match option_user {
                None => Ok(LoginError::WrongUsername),
                Some(user) => match verify(&login_request.password, &user.password) {
                    Ok(true) => {
                        match encode_token_and_refresh(
                            user._id.clone(),
                            *private::JWT_SECRET,
                            *private::REFRESH_JWT_SECRET,
                            EXPIRATION_REFRESH_TOKEN,
                            EXPIRATION_TOKEN,
                        ) {
                            Ok(tokens) => Ok(LoginError::Ok(tokens)),
                            Err(_) => Ok(LoginError::Unknown),
                        }
                    }
                    Ok(false) => Ok(LoginError::WrongPassword),
                    Err(_) => Ok(LoginError::WrongPassword),
                },
            },
            Err(_) => Ok(LoginError::WrongUsername),
        }
    }

    pub async fn registration(
        &self,
        registration_request: Json<RegistrationRequest>,
    ) -> mongodb::error::Result<RegistrationError> {
        let collection_user = self.database.collection::<User>("user");
        match find_user_by_username_and_mail(
            self,
            &registration_request.mail,
            &registration_request.username,
        )
        .await
        {
            FindUserBy::UserNotFound => match hash_text(registration_request.password.clone(), 4) {
                Ok(hash_password) => {
                    let user = User {
                        _id: ObjectId::new(),
                        username: registration_request.username.clone(),
                        password: hash_password,
                        mail: registration_request.mail.to_string(),
                        first_name: registration_request.first_name.clone(),
                        last_name: registration_request.last_name.clone(),
                    };
                    collection_user.insert_one(&user, None).await?;
                    match encode_token_and_refresh(
                        user._id.clone(),
                        *private::JWT_SECRET,
                        *private::REFRESH_JWT_SECRET,
                        EXPIRATION_REFRESH_TOKEN,
                        EXPIRATION_TOKEN,
                    ) {
                        Ok(tokens) => Ok(RegistrationError::Ok(tokens)),
                        Err(_) => Ok(RegistrationError::Unknown),
                    }
                }
                Err(_) => Ok(RegistrationError::WrongPassword),
            },
            FindUserBy::UserFoundByEmail => Ok(RegistrationError::AlreadyRegisteredByEmail),
            FindUserBy::UserFoundByUsername => Ok(RegistrationError::AlreadyRegisteredByUsername),
        }
    }
}
