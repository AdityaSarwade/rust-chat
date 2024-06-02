use reqwest;
use rocket::error;
use rocket::serde::{Deserialize, DeserializeOwned};

pub const CHUCKNORRIS_BASE_URL: &str = "https://api.chucknorris.io/jokes";
pub const CHUCKNORRIS_RANDOM_ENDPOINT: &str = "/random";
pub const CHUCKNORRIS_CATEGORIES_ENDPOINT: &str = "/categories";

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RandomJokeResult {
    // icon_url: String,
    // id: String,
    // url: String,
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RandomCategoriesJokeResult {
    // categories: Vec<String>,
    // created_at: String,
    // icon_url: String,
    // id: String,
    // updated_at: String
    // url: String,
    value: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CategoriesResult(Vec<String>);

async fn fetch_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}

pub fn get_help() -> String {
    return String::from("/chuck help - get help\n /chuck - random chuck norris Joke\n /chuck cat - get categories\n");
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

pub async fn get_categories() -> String {
    let request_url: String = format!(
        "{base}{endpoint}",
        base = CHUCKNORRIS_BASE_URL,
        endpoint = CHUCKNORRIS_CATEGORIES_ENDPOINT
    );
    println!("{}", request_url);

    match fetch_api_response::<CategoriesResult>(&request_url).await {
        Ok(result) => {
            print!("{:?}", result.0); // Access the Vec<String> directly
            result.0.join(", ") // Join the vector of strings
        }
        Err(e) => {
            error!("{}", e);
            "Chuck Norris so powerful you failed, Try Again".to_string()
        }
    }
}

pub async fn get_random_joke_from_categories(categories: String) -> String {
    let request_url: String = format!(
        "{base}{endpoint}{queryParams}",
        base = CHUCKNORRIS_BASE_URL,
        endpoint = CHUCKNORRIS_RANDOM_ENDPOINT,
        queryParams = format!("?category={}", categories)
    );
    println!("{}", request_url);

    match fetch_api_response::<RandomCategoriesJokeResult>(&request_url).await {
        Ok(result) => result.value,
        Err(e) => {
            error!("{}", e);
            "try with valid categories (hint: use /chuck cat)".to_string()
        }
    }
}
