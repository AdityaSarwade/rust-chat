use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref MONGOURI: &'static str = {
        dotenv().ok();
        Box::leak(
            env::var("MONGOURI")
                .expect("MONGOURI not found in .env")
                .into_boxed_str(),
        )
    };
    pub static ref JWT_SECRET: &'static str = {
        dotenv().ok();
        Box::leak(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET not found in .env")
                .into_boxed_str(),
        )
    };
    pub static ref REFRESH_JWT_SECRET: &'static str = {
        dotenv().ok();
        Box::leak(
            env::var("REFRESH_JWT_SECRET")
                .expect("REFRESH_JWT_SECRET not found in .env")
                .into_boxed_str(),
        )
    };
}

pub fn load_env() {
    // This function is intentionally left blank since lazy_static will load the env variables
}
