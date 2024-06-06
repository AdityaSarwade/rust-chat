use crate::plugins::chuck_norris::models::{CategoriesResult, RandomJokeResult};
use reqwest;
use rocket::error;
use rocket::serde::DeserializeOwned;

pub const CHUCKNORRIS_BASE_URL: &str = "https://api.chucknorris.io/jokes";
pub const CHUCKNORRIS_RANDOM_ENDPOINT: &str = "/random";
pub const CHUCKNORRIS_CATEGORIES_ENDPOINT: &str = "/categories";

async fn fetch_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}

pub fn get_help() -> String {
    return String::from(
        "<ul>
<li><b style='color: #ffbb33;'>/chuck help</b> - get help</li>
<li><b style='color: #ffbb33;'>/chuck</b> - random chuck norris Joke</li>
<li><b style='color: #ffbb33;'>/chuck cat</b> - get categories</li>
<li><b style='color: #ffbb33;'>/chuck @[name]</b> - personalized chuck norris joke</li>
<li><b style='color: #ffbb33;'>/chuck @[name] cat [categories]</b> - personalized chuck norris joke</li>
</ul>",
    );
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

    match fetch_api_response::<RandomJokeResult>(&request_url).await {
        Ok(result) => result.value,
        Err(e) => {
            error!("{}", e);
            "try with valid categories (hint: use /chuck cat)".to_string()
        }
    }
}

pub async fn get_random_joke_from_name(name: String) -> String {
    let request_url: String = format!(
        "{base}{endpoint}{queryParams}",
        base = CHUCKNORRIS_BASE_URL,
        endpoint = CHUCKNORRIS_RANDOM_ENDPOINT,
        queryParams = format!("?name={}", name)
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

pub async fn get_random_joke_from_name_and_categories(name: String, categories: String) -> String {
    let request_url: String = format!(
        "{base}{endpoint}{queryParams}",
        base = CHUCKNORRIS_BASE_URL,
        endpoint = CHUCKNORRIS_RANDOM_ENDPOINT,
        queryParams = format!("?name={}&category={}", name, categories)
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
