use reqwest;
use rocket::error;
use rocket::serde::{Deserialize, DeserializeOwned};

pub const CHUCKNORRIS_BASE_URL: &str = "https://api.chucknorris.io/jokes";
pub const CHUCKNORRIS_RANDOM_ENDPOINT: &str = "/random";
// pub const CHUCKNORRIS_CATEGORIES_ENDPOINT: &str = "/categories";

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RandomJokeResult {
    // icon_url: String,
    // id: String,
    // url: String,
    value: String,
}

async fn fetch_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}

// #[tokio::main]
pub async fn get_random_joke() -> String {
    let request_url: String = format!(
        "{base}{endpoint}",
        base = CHUCKNORRIS_BASE_URL,
        endpoint = CHUCKNORRIS_RANDOM_ENDPOINT
    );
    println!("{}", request_url);

    match fetch_api_response::<RandomJokeResult>(&request_url).await {
        Ok(result) => result.value,
        Err(e) => {
            error!("{}", e);
            "Chuck Norris so powerful you failed, Try Again".to_string()
        }
    }
}
